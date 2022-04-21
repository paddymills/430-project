

use loans::{
    db,
    schema::{LoanOps, Loan}
};

#[tauri::command]
pub fn get_loans() -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans()
}