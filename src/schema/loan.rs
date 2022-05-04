
use chrono::naive::NaiveDate;
use oracle::{self, Connection, Row, RowValue};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Tabled, Deserialize, Serialize)]
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
    fn list_loans(self: &Self) -> Option<Vec<Loan>> where Self: Sized;
    fn add_loan(self: &Self, loan_data: &Loan) -> oracle::Result<oracle::Statement> where Self: Sized;
    fn edit_loan(self: &Self, loan_data: &Loan) -> oracle::Result<oracle::Statement> where Self: Sized;
    fn remove_loan(self: &Self, id: &u32) -> oracle::Result<oracle::Statement> where Self: Sized;
}

impl LoanOps for Connection {
    fn get_loans(self: &Self) -> Option<Vec<Loan>> {
        let res = self.query_as::<Loan>("select * from loan", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn list_loans(self: &Self) -> Option<Vec<Loan>>
        where Self: Sized
    {
        let res = self.query_as::<Loan>("select * from loan", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }
    fn add_loan(self: &Self, loan_data: &Loan) -> oracle::Result<oracle::Statement> {
        let result = self.execute(
            "call add_loan(:1, :2, :3, :4, :5, :6, :7)",
            &[
                &loan_data.customer_id,
                &loan_data.loan_amount,
                &loan_data.interest_rate,
                &loan_data.amount_paid,
                &loan_data.start_date,
                &loan_data.end_date,
                &loan_data.number_of_payments
            ]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }

    fn edit_loan(self: &Self, loan_data: &Loan) -> oracle::Result<oracle::Statement> {
        let result = self.execute(
            "call change_loan(:1, :2, :3, :4, :5, :6, :7, :8)",
            &[
                &loan_data.loan_id,
                &loan_data.customer_id,
                &loan_data.loan_amount,
                &loan_data.interest_rate,
                &loan_data.amount_paid,
                &loan_data.start_date,
                &loan_data.end_date,
                &loan_data.number_of_payments
            ]
        );
        
        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }

    fn remove_loan(self: &Self, id: &u32) -> oracle::Result<oracle::Statement> {
        let result = self.execute(
            "call remove_loan(:1)",
            &[id]
        );

        if let Ok(_) = result {
            let _ = self.commit();
        }

        result
    }
}
