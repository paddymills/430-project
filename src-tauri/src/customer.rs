
/*
    customer functions:
        - list loans
        - get all customers
        - get customer by id
        - delete customer
        - add customer
        - edit customer
*/

use crate::get_oracle_error_text;
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

#[tauri::command]
pub fn get_customer(id: u32) -> Option<Customer> {
    let res = db::get_cnxn().query_row_as::<Customer>(
        "
            select *
            from customer
            where customer_id = :1
        ", &[&id]
    );

    match res {
        Ok(row) => Some(row),
        _ => None
    }
}

#[tauri::command]
pub fn delete_customer(id: u32) -> Result<(), String> {
    match db::get_cnxn().remove_customer(&id) {
        Ok(_) => Ok(()),
        Err(oracle::Error::OciError(e)) => Err(get_oracle_error_text(e)),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn add_customer(fname: String, lname: String, email: String, phone: String) -> Result<(), String> {
    match db::get_cnxn().add_customer(&fname, &lname, &email, &phone) {
        Ok(_) => Ok(()),
        Err(oracle::Error::OciError(e)) => Err(get_oracle_error_text(e)),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn edit_customer(id: u32, fname: String, lname: String, email: String, phone: String) -> Result<(), String> {
    match db::get_cnxn().edit_customer(&id, &fname, &lname, &email, &phone) {
        Ok(_) => Ok(()),
        Err(oracle::Error::OciError(e)) => Err(get_oracle_error_text(e)),
        Err(e) => Err(e.to_string())
    }
}
