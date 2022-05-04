
use oracle::{self, Connection, Row, RowValue};
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

pub trait AutoLoanOps {
    fn add_auto_loan(
        self: &Self,
        loan_data: &AutoLoan
    ) -> oracle::Result<oracle::Statement> where Self: Sized;
    fn edit_auto_loan(
        self: &Self,
        loan_data: &AutoLoan
    ) -> oracle::Result<oracle::Statement> where Self: Sized;
}

impl AutoLoanOps for Connection {
    fn add_auto_loan(
        self: &Self,
        loan_data: &AutoLoan
    ) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call add_auto_loan(:1, :2, :3, :4, :5)",
            &[&loan_data.loan_id, &loan_data.make, &loan_data.model, &loan_data.year, &loan_data.vin]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
    fn edit_auto_loan(
        self: &Self,
        loan_data: &AutoLoan
    ) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call change_auto_loan(:1, :2, :3, :4, :5)",
            &[&loan_data.loan_id, &loan_data.make, &loan_data.model, &loan_data.year, &loan_data.vin]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
}