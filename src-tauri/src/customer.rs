
/*
    customer functions:
        - list loans
*/

use loans::{
    db,
    schema::{CustomerOps, Loan}
};

#[tauri::command]
pub fn get_cust_loans(user: String) -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans(&user)
}
