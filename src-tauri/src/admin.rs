
/*
    admin functions:
        - add/edit/remove/search for a customer
        - list Customers
        - add/edit/remove a loan for a customer
        - search for a loan
        - list loans
*/

use loans::{
    db,
    schema::{LoanOps, Loan}
};

#[tauri::command]
pub fn get_all_loans() -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans()
}

