
use requestty::{
    DefaultSeparator, prompt, prompt_one,
    Question, Answer
};

use crate::db;
use crate::schema::CustomerOps;

pub fn menu() {
    loop {
        let select = Question::select("customer")
            .message("Customer Menu")
            .choices(vec![
                "Add Customer".into(),
                "Edit Customer".into(),
                "Remove Customer".into(),
                "Search for a Customer".into(),
                "Browse Customer List".into(),
                DefaultSeparator,
                "Exit to Main Menu".into()
            ])
            .build();
    
        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => add(),
                1 => edit(),
                2 => remove(),
                3 => find(),
                4 => list(),
                6 => break,
                _ => unreachable!()
            }
        }
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

fn find() {
    
}

fn list() {
    
}
