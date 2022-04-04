
mod answer;
mod customer;
mod main;
mod transaction;

pub use answer::AnswerValue;
pub use customer::*;
pub use main::*;
pub use transaction::*;

const MENU_SEP: &str = "\n\t\t────────────────────────────\n";

pub fn run() {
    main::menu();
}
