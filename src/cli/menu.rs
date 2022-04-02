
use requestty::{
    DefaultSeparator, prompt_one,
    Question, Answer
};

use crate::cli::{customer, transaction};

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
                0 => customer::menu(),
                1 => transaction::menu(),
                3 => break,
                _ => unreachable!()
            }
        }
    }
}
