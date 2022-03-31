
use requestty::{Question, prompt_one};

pub fn main() {
    let select = Question::select("main")
        .message("Main Menu")
        .choices::<Vec<String>, String>(vec![
            "Add Customer".into(),
            "Add Loan".into(),
            "Delete Loan".into()
        ])
        .build();

    println!("{:?}", prompt_one(select));
}
