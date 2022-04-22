
use oracle::{self, Row, RowValue};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct AutoLoan {
    pub loan_id: u32,
    pub make: String,
    pub model: String,
    pub year: u32,
    pub vin: String
}

impl RowValue for AutoLoan {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            loan_id: row.get("loan_id")?,
            make: row.get("make")?,
            model: row.get("model")?,
            year: row.get("year")?,
            vin: row.get("vin")?,
        })
    }
}
