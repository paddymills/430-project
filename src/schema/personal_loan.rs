

use oracle::{self, Row, RowValue};

pub struct PersonalLoan {
    pub loan_id: u32,
    pub purpose: String
}

impl RowValue for PersonalLoan {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            loan_id: row.get("loan_id")?,
            purpose: row.get("purpose")?
        })
    }
}
