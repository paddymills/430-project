
use requestty::{
    DefaultSeparator, prompt_one,
    Question, Answer
};

pub fn main() {
    loop {
        let select = Question::select("main")
            .message("Main Menu")
            .choices(vec![
                "Customer Menu".into(),
                "Transaction Menu".into(),
                DefaultSeparator,
                "Exit".into()
            ])
            .build();

        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => customers_menu(),
                1 => transactions_menu(),
                3 => break,
                _ => unreachable!()
            }
        }
    }
}

fn customers_menu() {
    loop {
        let select = Question::select("main")
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
                0 => todo!("Add Customer"),
                1 => todo!("Edit Customer"),
                2 => todo!("Remove Customer"),
                3 => todo!("Search for a Customer"),
                4 => todo!("Browse Customer List"),
                6 => break,
                _ => unreachable!()
            }
        }
    }
}

fn transactions_menu() {
    loop {
        let select = Question::select("main")
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
                    0 => todo!("Add Transaction"),
                    1 => todo!("Edit Transaction"),
                    2 => todo!("Remove Transaction"),
                    3 => todo!("Browse Transactions"),
                    4 => todo!("Search for a Transaction (by customer)"),
                    5 => todo!("Search for a Transaction (by product)"),
                    6 => todo!("Search for a Transaction (by date)"),
                    8 => break,
                    _ => unreachable!()
                }
            }
    }

}
