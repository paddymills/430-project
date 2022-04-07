
use oracle::{self, Connection, Row, RowValue};

use crate::db;

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
}

impl AuthOps for Connection {
    fn get_users(self: &Self) -> Option<Vec<Auth>> {
        let res = self.query_as("select * from customer", &[]);

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
