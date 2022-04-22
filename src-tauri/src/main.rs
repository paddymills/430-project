#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{auth, admin, customer, loan};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            auth::validate_login,
            admin::get_all_loans,
            customer::get_cust_loans,
            customer::get_customers,
            customer::get_customer,
            loan::get_loans,
            loan::get_loan
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
