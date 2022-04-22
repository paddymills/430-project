
/*
    customer functions:
        - list loans
        - get all customers
        - get customer by id
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
