
use oracle::{self, Row, RowValue};

struct Auth {
    username: String,
    pwd_hash: String
}

impl RowValue for Auth {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            username: row.get("username")?,
            pwd_hash: row.get("pwd_hash")?
        })
    }
}
