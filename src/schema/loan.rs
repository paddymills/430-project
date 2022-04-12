
use chrono::naive::NaiveDate;
use oracle::{self, Connection, Row, RowValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Loan {
    pub loan_id: u32,
    pub customer_id: u32,
    pub loan_amount: f32,
    pub interest_rate: f32,
    pub amount_paid: f32,

    
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    
    pub number_of_payments: u32
}

impl RowValue for Loan {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            loan_id: row.get("loan_id")?,
            customer_id: row.get("customer_id")?,
            loan_amount: row.get("loan_amount")?,
            interest_rate: row.get("interest_rate")?,
            amount_paid: row.get("amount_paid")?,
            start_date: row.get("start_date")?,
            end_date: row.get("end_date")?,
            number_of_payments: row.get("number_of_payments")?
        })
    }
}

pub trait LoanOps {
    fn get_loans(self: &Self) -> Option<Vec<Loan>>;
}

impl LoanOps for Connection {
    fn get_loans(self: &Self) -> Option<Vec<Loan>> {
        let res = self.query_as::<Loan>("select * from loan", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }
}
