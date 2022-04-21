
use requestty::{
    DefaultSeparator, prompt_one,
    Question, Answer
};
use tabled::{Table, Style};

use crate::{
    cli::{MENU_SEP},
    db,
    schema::LoanOps
};

pub fn menu() {
    loop {
        let select = Question::select("loan")
            .message("Customer Menu")
            .choices(vec![
                "List Loans".into(),
                DefaultSeparator,
                "Exit to Main Menu".into()
            ])
            .build();
    
        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => list(),
                2 => break,
                _ => unreachable!()
            }
        }

        println!("{}", MENU_SEP);
    }
}

fn list() {
    let results = db::get_cnxn().list_loans();

    if let Some(rows) = results {
        println!("{}", Table::new(rows).with(Style::psql()));
    }

    else {
        println!("No results returned");
    }
}
