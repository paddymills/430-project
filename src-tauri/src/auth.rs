
use loans::{
    db,
    schema::{AuthOps, LoginResult}
};
use sha2::{Digest, Sha256};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResult {
    pub username: bool,
    pub password: bool
}

pub fn validate_login(user: String, pwd: String) -> AuthResult {
    let conn = db::get_cnxn();
    match conn.validate_login(user, pwd) {
        LoginResult::BadUsername => AuthResult { username: false, password: false },
        LoginResult::BadPassword => AuthResult { username: true, password: false },
        LoginResult::Pass =>        AuthResult { username: true, password: true },
    }
}

pub fn hash_pwd(pwd: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pwd);
    
    format!("{:x}", hasher.finalize())
}
