
use oracle::{self, Connection, Row, RowValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

pub trait PersonalLoanOps {
    fn add_personal_loan(self: &Self, loan_data: &PersonalLoan) -> oracle::Result<oracle::Statement> where Self: Sized;
    fn edit_personal_loan(self: &Self, loan_data: &PersonalLoan) -> oracle::Result<oracle::Statement> where Self: Sized;
}

impl PersonalLoanOps for Connection {
    fn add_personal_loan(self: &Self, loan_data: &PersonalLoan) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call add_personal_loan(:1, :2)",
            &[&loan_data.loan_id, &loan_data.purpose]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
    fn edit_personal_loan(self: &Self, loan_data: &PersonalLoan) -> oracle::Result<oracle::Statement> where Self: Sized {
        let result = self.execute(
            "call change_personal_loan(:1, :2)",
            &[&loan_data.loan_id, &loan_data.purpose]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
}
