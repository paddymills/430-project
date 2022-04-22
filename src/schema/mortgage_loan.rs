
use oracle::{self, Row, RowValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MortgageLoan {
    pub loan_id: u32,
    pub address: String,
    pub area: f32,
    pub num_bedrooms: u32,
    pub num_bathrooms: u32,
    pub price: f32
}

impl RowValue for MortgageLoan {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            loan_id: row.get("loan_id")?,
            address: row.get("address")?,
            area: row.get("area")?,
            num_bedrooms: row.get("num_bedrooms")?,
            num_bathrooms: row.get("num_bathrooms")?,
            price: row.get("price")?
        })
    }
}
