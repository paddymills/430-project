
/*
    customer functions:
        - list loans
*/

use loans::{
    db,
    schema::{CustomerOps, Customer, Loan}
};

#[tauri::command]
pub fn get_cust_loans(user: String) -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans(&user)
}

#[tauri::command]
pub fn get_customers() -> Option<Vec<Customer>> {
    db::get_cnxn().list_customers()
}
