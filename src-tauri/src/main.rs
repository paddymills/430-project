#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;

use loans::schema;
use app::auth;

struct App {
  _users: HashMap<String, String>
}

fn test(db: bool) -> Vec<schema::Auth> {
  match db {
    true => schema::Auth::get_users(),
    false => vec![
      schema::Auth {
        username: "admin".into(),
        pwd_hash: "pwd123".into()
      },
      schema::Auth {
        username: "cust1".into(),
        pwd_hash: "passwrd".into()
      },
      schema::Auth {
        username: "cust3".into(),
        pwd_hash: "anotherstr".into()
      }
    ]
  }
}

impl App {
  fn new() -> Self {
    let mut users: HashMap<String, String> = HashMap::new();
    // for user in schema::Auth::get_users() {
    for user in test(false) {
      users.insert(user.username, user.pwd_hash);
    }

    Self {
      _users: users
    }
  }
}

#[tauri::command]
fn validate_login(_app: tauri::State<App>, user: String, pwd: String) -> auth::AuthResult {
  auth::validate_login(user, pwd)
}

fn main() {
  tauri::Builder::default()
    .manage(App::new())
    .invoke_handler(tauri::generate_handler![
      validate_login
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
