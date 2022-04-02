
use requestty::{
    DefaultSeparator, prompt, prompt_one,
    Question, Answer
};

use crate::db;

pub fn menu() {
    loop {
        let select = Question::select("transaction")
            .message("Customer Menu")
            .choices(vec![
                "Add Transaction".into(),
                "Edit Transaction".into(),
                "Remove Transaction".into(),
                "Browse Transactions".into(),
                "Search for a Transaction (by customer)".into(),
                "Search for a Transaction (by product)".into(),
                "Search for a Transaction (by date)".into(),
                DefaultSeparator,
                "Exit to Main Menu".into()
            ])
            .build();

            if let Ok(Answer::ListItem(result)) = prompt_one(select) {
                match result.index {
                    0 => add(),
                    1 => edit(),
                    2 => remove(),
                    3 => list(),
                    4 => list_by_customer(),
                    5 => list_by_type(),
                    6 => list_by_date(),
                    8 => break,
                    _ => unreachable!()
                }
            }
    }
}

fn add() {
    let questions = vec![
        Question::int("cid").message("Customer ID:").build(),
        Question::int("lid").message("Loan ID:").build(),
        Question::input("dt").message("Date (m/d/y):").build(),
        Question::float("amt").message("Amount:").build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let mut fname = String::new();
            let mut lname = String::new();
            let mut email = String::new();
            let mut phone = String::new();


            if let Some(Answer::String(val)) = answers.get("fname") { fname = val.to_string() }
            if let Some(Answer::String(val)) = answers.get("lname") { lname = val.to_string() }
            if let Some(Answer::String(val)) = answers.get("email") { email = val.to_string() }
            if let Some(Answer::String(val)) = answers.get("phone") { phone = val.to_string() }

            let _ = db::get_cnxn().add_customer(&fname, &lname, &email, &phone);
            

        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn edit() {
    
}

fn remove() {
    
}

fn list() {
    
}

fn list_by_customer() {
    
}

fn list_by_type() {
    
}

fn list_by_date() {
    
}
