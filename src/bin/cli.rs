
use requestty::{Question, Answer, prompt_one};

pub fn main() -> Result<(), String> {
    loop {
        let select = Question::select("main")
            .message("Main Menu")
            .choices::<Vec<String>, String>(vec![
                "Customer Menu".into(),
                "Transaction Menu".into(),
                "Exit".into()
            ])
            .build();
        if let Ok(Answer::ListItem(result)) = prompt_one(select) {
            match result.index {
                0 => customers_menu(),
                1 => transactions_menu(),
                2 => break,
                _ => ()
            }
        }
    }

    Ok(())
}

fn customers_menu() {
    let _select = Question::select("main")
        .message("Customer Menu")
        .choices::<Vec<String>, String>(vec![
            "Add Customer".into(),
            "Edit Customer".into(),
            "Remove Customer".into(),
            "Search for a Customer".into(),
            "Browse Customer List".into()
        ])
        .build();
}

fn transactions_menu() {
    let _select = Question::select("main")
        .message("Customer Menu")
        .choices::<Vec<String>, String>(vec![
            "Add Transaction".into(),
            "Edit Transaction".into(),
            "Remove Transaction".into(),
            "Browse Transactions".into(),
            "Search for a Transaction (by customer)".into(),
            "Search for a Transaction (by product)".into(),
            "Search for a Transaction (by date)".into(),
        ])
        .build();

}
