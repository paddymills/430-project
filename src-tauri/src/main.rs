#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;

use loans::schema;

struct App {
  users: HashMap<String, String>
}

impl App {
  fn new() -> Self {
    let mut users: HashMap<String, String> = HashMap::new();
    for user in schema::Auth::get_users() {
      users.insert(user.username, user.pwd_hash);
    }

    Self {
      users: users
    }
  }
}

#[tauri::command]
fn get_usernames(app: tauri::State<App>) -> Vec<String> {
  app.users.keys().map(|x| x.clone()).collect()
}

#[tauri::command]
fn validate_password(app: tauri::State<App>, user: String, pwd: String) -> bool {
  app.users.get(&user).unwrap() == &pwd
}

fn main() {
  tauri::Builder::default()
    .manage(App::new())
    .invoke_handler(tauri::generate_handler![
      get_usernames,
      validate_password
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
