#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use loans::{
    db,
    schema::{AuthOps, Loan}
};
use app::auth;


#[tauri::command]
fn validate_login(user: String, pwd: String) -> auth::AuthResult {
    auth::validate_login(user, pwd)
}

#[tauri::command]
fn get_customer_loans(user: String) -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans(&user)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_customer_loans,
            validate_login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
