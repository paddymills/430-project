
mod answer;
mod customer;
mod main;
mod transaction;

pub use answer::AnswerValue;
pub use customer::*;
pub use main::*;
pub use transaction::*;

use crate::db;
use spinach::Spinach;

const MENU_SEP: &str = "\n\t\t────────────────────────────\n";

pub fn run() {
    {
        let spinner = Spinach::new("connecting to OracleDB...");
        db::init();
        spinner.succeed("OracleDB connected.");
    }
    
    main::menu();
}
