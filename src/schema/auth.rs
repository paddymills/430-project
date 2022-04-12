
use oracle::{self, Connection, Row, RowValue};

use crate::db;

pub enum LoginResult {
    Pass,
    BadUsername,
    BadPassword
}

pub struct Auth {
    pub username: String,
    pub pwd_hash: String
}

impl RowValue for Auth {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            username: row.get("username")?,
            pwd_hash: row.get("pwd_hash")?
        })
    }
}

pub trait AuthOps {
    fn get_users(self: &Self) -> Option<Vec<Auth>>;
    fn validate_login(self: &Self, user: String, pwd: String) -> LoginResult;
}

impl AuthOps for Connection {
    fn get_users(self: &Self) -> Option<Vec<Auth>> {
        let res = self.query_as("select * from auth", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn validate_login(self: &Self, user: String, pwd: String) -> LoginResult {
        let res = self.query_row_as::<Auth>("select * from auth where username=:1", &[&user]);

        match res {
            Ok(row) => {
                if row.pwd_hash == pwd {
                    LoginResult::Pass
                } else {
                    LoginResult::BadPassword
                }
            },
            Err(_) => LoginResult::BadUsername
        }
    }
}

impl Auth {
    pub fn get_users() -> Vec<Auth> {
        db::get_cnxn().get_users().unwrap()
    }
}
