
use oracle::{self, Connection, Row, RowValue};
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

pub trait MortgageLoanOps {
    fn add_mortgage_loan(self: &Self, loan_data: &MortgageLoan) -> oracle::Result<oracle::Statement> where Self: Sized;
    fn edit_mortgage_loan(self: &Self, loan_data: &MortgageLoan) -> oracle::Result<oracle::Statement> where Self: Sized;
}

impl MortgageLoanOps for Connection {
    fn add_mortgage_loan(self: &Self, loan_data: &MortgageLoan) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call add_mortgage_loan(:1, :2, :3, :4, :5, :6)",
            &[&loan_data.loan_id, &loan_data.address, &loan_data.area, &loan_data.num_bedrooms, &loan_data.num_bathrooms, &loan_data.price]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
    fn edit_mortgage_loan(self: &Self, loan_data: &MortgageLoan) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call change_mortgage_loan(:1, :2, :3, :4, :5, :6)",
            &[&loan_data.loan_id, &loan_data.address, &loan_data.area, &loan_data.num_bedrooms, &loan_data.num_bathrooms, &loan_data.price]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
}
