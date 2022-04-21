
use requestty::{
    DefaultSeparator, prompt_one,
    Question, Answer
};

use crate::cli::{customer, transaction, loan, MENU_SEP};

pub fn menu() {
    loop {
        let select = Question::select("main")
            .message("Main Menu")
            .choices(vec![
                "Customer Menu".into(),
                "Transaction Menu".into(),
                "Loan Menu".into(),
                DefaultSeparator,
                "Exit Loan System".into()
            ])
            .build();

        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => customer::menu(),
                1 => transaction::menu(),
                2 => loan::menu(),
                4 => break,
                _ => unreachable!()
            }
        }
    }

    println!("{}", MENU_SEP);
}