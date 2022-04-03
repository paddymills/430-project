
use requestty::{
    DefaultSeparator, prompt, prompt_one,
    Question, Answer
};
use tabled::{Table, Style};

use crate::{
    cli::{AnswerValue, MENU_SEP},
    db,
    schema::CustomerOps
};

pub fn menu() {
    loop {
        let select = Question::select("customer")
            .message("Customer Menu")
            .choices(vec![
                "Add Customer".into(),
                "Edit Customer".into(),
                "Remove Customer".into(),
                "Search for a Customer".into(),
                "List Customers".into(),
                DefaultSeparator,
                "Exit to Main Menu".into(),
                "Exit Loan System".into()
            ])
            .build();

        println!("{}", MENU_SEP);
    
        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => add(),
                1 => edit(),
                2 => remove(),
                3 => find(),
                4 => list(),
                6 => break,
                7 => break,
                _ => unreachable!()
            }
        }

        println!("{}", MENU_SEP);
    }
}

fn add() {
    let questions = vec![
        Question::input("fname").message("First Name:").build(),
        Question::input("lname").message("Last Name:").build(),
        Question::input("email").message("Email Address:").build(),
        Question::input("phone").message("Phone Number:").build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let fname = &answers.get_str("fname");
            let lname = &answers.get_str("lname");
            let email = &answers.get_str("email");
            let phone = &answers.get_str("phone");

            let _ = db::get_cnxn().add_customer(&fname, &lname, &email, &phone);
            

        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn edit() {
    let questions = vec![
        Question::input("id").message("Customer ID:").build(),
        Question::input("fname").message("First Name:").build(),
        Question::input("lname").message("Last Name:").build(),
        Question::input("email").message("Email Address:").build(),
        Question::input("phone").message("Phone Number:").build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let cid = &answers.get_int("id");
            let fname = &answers.get_str("fname");
            let lname = &answers.get_str("lname");
            let email = &answers.get_str("email");
            let phone = &answers.get_str("phone");

            let _ = db::get_cnxn().edit_customer(cid, &fname, &lname, &email, &phone);
            

        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn remove() {
    let questions = vec![
        Question::input("id").message("Customer ID:").build()
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let cid = &answers.get_int("id");

            let _ = db::get_cnxn().remove_customer(cid);
            

        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn find() {
    let questions = vec![
        Question::input("fname").message("First Name:").build(),
        Question::input("lname").message("Last Name:").build()
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let fname = &answers.get_str("fname");
            let lname = &answers.get_str("lname");

            let _ = db::get_cnxn().find_customer(fname, lname);
            

        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn list() {
    let results = db::get_cnxn().list_customers();

    if let Some(rows) = results {
        println!("{}", Table::new(rows).with(Style::psql()));
    }

    else {
        println!("No results returned");
    }
}
