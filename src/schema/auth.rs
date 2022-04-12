
use oracle::{self, Connection, Row, RowValue};

use crate::{
    db,
    schema::Loan
};
use sha2::{Digest, Sha256};

pub enum LoginResult {
    Pass(bool),
    BadUsername,
    BadPassword
}

pub fn hash_pwd(pwd: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pwd);
    
    format!("{:x}", hasher.finalize())
}

pub struct Auth {
    pub username: String,
    pub pwd_hash: String,
    pub is_admin: char,
}

impl RowValue for Auth {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            username: row.get("username")?,
            pwd_hash: row.get("pwd_hash")?,

            // ok to use unwrap, because is_admin is '0' or '1'
            is_admin: row.get::<_, String>("is_admin")?
                .chars()
                .next()
                .unwrap()
        })
    }
}

pub trait AuthOps {
    fn get_users(self: &Self) -> Option<Vec<Auth>>;
    fn validate_login(self: &Self, user: String, pwd: String) -> LoginResult;
    fn get_loans(self: &Self, user: &String) -> Option<Vec<Loan>>;
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
                if row.pwd_hash == hash_pwd(pwd) {
                    LoginResult::Pass(row.is_admin == '1')
                } else {
                    LoginResult::BadPassword
                }
            },
            Err(_) => LoginResult::BadUsername
        }
    }

    fn get_loans(self: &Self, user: &String) -> Option<Vec<Loan>> {
        let res = self.query_as::<Loan>(
            "
                select *
                from loan
                where customer_id = (
                    select customer_id
                    from auth
                    where username = :1
                )
            ",
            &[user]
        );

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }
}

impl Auth {
    pub fn get_users() -> Vec<Auth> {
        db::get_cnxn().get_users().unwrap()
    }
}
