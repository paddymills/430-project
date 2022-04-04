
use chrono::{
    naive::NaiveDate,
    prelude::*
};
use requestty::{
    DefaultSeparator, prompt, prompt_one,
    Question, Answer
};
use tabled::{Table, Style};

use crate::{
    cli::{AnswerValue, MENU_SEP},
    db,
    schema::TransactionOps
};
const DATE_FMT: &str = "%m/%d/%Y";

pub fn menu() {
    loop {
        let select = Question::select("transaction")
            .message("Customer Menu")
            .choices(vec![
                "Add Transaction".into(),
                "Edit Transaction".into(),
                "Remove Transaction".into(),
                "List Transactions".into(),
                "Search for a Transaction (by customer)".into(),
                "Search for a Transaction (by product)".into(),
                "Search for a Transaction (by date)".into(),
                DefaultSeparator,
                "Exit to Main Menu".into()
            ])
            .build();
            
            println!("{}", MENU_SEP);

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
        Question::input("dt").message("Transaction Date (month/day/year):")
            .default(Local::today().format(DATE_FMT).to_string())
            .validate(|name, _| match NaiveDate::parse_from_str(name, DATE_FMT) {
                Ok(_) => Ok(()), 
                Err(_) => Err("Date entered does not match format".to_owned())
            }).build(),
        Question::float("amt").message("Amount:").build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let cid = &answers.get_int("lid");
            let lid = &answers.get_int("cid");
            let dt = NaiveDate::parse_from_str(&answers.get_str("dt"), DATE_FMT).unwrap();
            let amt = &answers.get_float("amt");

            match db::get_cnxn().add_transaction(cid, lid, dt, amt) {
                Ok(_) => println!("Transaction added"),
                Err(_) => println!("Error adding transaction. Rolling back database.")
            }
        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn edit() {
    let questions = vec![
        Question::int("tid").message("Transaction ID:").build(),
        Question::input("dt").message("Transaction Date (month/day/year):")
            .default(Local::today().format(DATE_FMT).to_string())
            .validate(|name, _| match NaiveDate::parse_from_str(name, DATE_FMT) {
                Ok(_) => Ok(()), 
                Err(_) => Err("Date entered does not match format".to_owned())
            }).build(),
        Question::float("amt").message("Amount:").build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let tid = &answers.get_int("tid");
            let dt = NaiveDate::parse_from_str(&answers.get_str("dt"), DATE_FMT).unwrap();
            let amt = &answers.get_float("amt");

            match db::get_cnxn().edit_transaction(tid, dt, amt) {
                Ok(_) => println!("Transaction updated"),
                Err(_) => println!("Error updating transaction. Rolling back database.")
            }
        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn remove() {
    let questions = vec![
        Question::int("tid").message("Transaction ID:").build()
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let tid = &answers.get_int("tid");

            match db::get_cnxn().remove_transaction(tid) {
                Ok(_) => println!("Transaction removed"),
                Err(_) => println!("Error removing transaction. Rolling back database.")
            }
        },
        _ => println!("Input error. Returning to menu.")
    }
}

fn list() {
    let results = db::get_cnxn().list_transactions();

    if let Some(rows) = results {
        println!("{}", Table::new(rows).with(Style::psql()));
    }

    else {
        println!("No results returned");
    }
}

fn list_by_customer() {
    let questions = vec![
        Question::input("fname").message("First Name:").build(),
        Question::input("lname").message("Last Name:").build()
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let fname = &answers.get_str("fname");
            let lname = &answers.get_str("lname");

            let results = db::get_cnxn().search_transaction_by_customer(fname, lname);
            if let Some(rows) = results {
                println!("{}", Table::new(rows).with(Style::psql()));
            }
        
            else {
                println!("No results returned");
            }

        },
        _ => println!("Input error. Returning to menu.")
    }

}

fn list_by_type() {
    let select = Question::select("loan_type")
        .message("Loan Type")
        .choices(vec![
            "Auto".into(),
            "Mortgage".into(),
            "Personal".to_string()
        ])
        .build();

    if let Ok(Answer::ListItem(result)) = prompt_one(select) {
        let results = db::get_cnxn().search_transaction_by_loan_type(&result.text);
        if let Some(rows) = results {
            println!("{}", Table::new(rows).with(Style::psql()));
        }
    
        else {
            println!("No results returned");
        }
    }
}

fn list_by_date() {
    let questions = vec![
        Question::input("dt").message("Transaction Date (month/day/year):")
            .default(Local::today().format(DATE_FMT).to_string())
            .validate(|name, _| match NaiveDate::parse_from_str(name, DATE_FMT) {
                Ok(_) => Ok(()), 
                Err(_) => Err("Date entered does not match format".to_owned())
            }).build(),
    ];

    match prompt(questions) {
        
        Ok(answers) => {
            let dt = NaiveDate::parse_from_str(&answers.get_str("dt"), DATE_FMT).unwrap();

            let results = db::get_cnxn().search_transaction_by_date(dt);
            if let Some(rows) = results {
                println!("{}", Table::new(rows).with(Style::psql()));
            }
        
            else {
                println!("No results returned");
            }
        },
        _ => println!("Input error. Returning to menu.")
    }    
}
