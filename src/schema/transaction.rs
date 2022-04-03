
use chrono::naive::NaiveDate;
use oracle::{self, Row, RowValue, Connection};
use tabled::Tabled;

#[derive(Tabled)]
pub struct Transaction {
    pub transaction_id: u32,
    pub customer_id: u32,
    pub loan_id: u32,
    pub transaction_amount: f32,
    pub transaction_date: NaiveDate
}

impl RowValue for Transaction {
    fn get(row: &Row) -> oracle::Result<Self> {
        Ok(Self {
            transaction_id: row.get("transaction_id")?,
            customer_id: row.get("customer_id")?,
            loan_id: row.get("loan_id")?,
            transaction_amount: row.get("transaction_amount")?,
            transaction_date: row.get("transaction_date")?
        })
    }
}

pub trait TransactionOps {
    fn add_transaction(self: &Self, customer_id: &u32, loan_id: &u32, date: NaiveDate, amount: &f32) -> oracle::Result<()>;
    fn edit_transaction(self: &Self, transaction_id: &u32, date: NaiveDate, amount: &f32) -> oracle::Result<()>;
    fn remove_transaction(self: &Self, transaction_id: &u32) -> oracle::Result<()>;
    fn list_transactions(self: &Self) -> Option<Vec<Transaction>>;
    fn search_transaction_by_customer(self: &Self, fname: &String, lname: &String) -> Option<Vec<Transaction>>;
    fn search_transaction_by_loan_type(self: &Self, loan_type: &String) -> Option<Vec<Transaction>>;
    fn search_transaction_by_date(self: &Self, date: NaiveDate) -> Option<Vec<Transaction>>;
}

impl TransactionOps for Connection {
    fn add_transaction(
        self: &Self,
        customer_id: &u32,
        loan_id: &u32,
        date: NaiveDate,
        amount: &f32
    ) -> oracle::Result<()> {
        let _ = self.execute(
            "call add_transaction(:1, :2, :3, :4)",
            &[customer_id, loan_id, &date, amount]
        );
        
        self.commit()
    }

    fn edit_transaction(
        self: &Self,
        transaction_id: &u32,
        date: NaiveDate,
        amount: &f32
    ) -> oracle::Result<()> {
        let _ = self.execute(
            "call change_transaction(:1, :2, :3)",
            &[transaction_id, &date, amount]
        );
        
        self.commit()
    }

    fn remove_transaction(self: &Self, transaction_id: &u32) -> oracle::Result<()> {
        let _ = self.execute(
            "call remove_transaction(:1)",
            &[transaction_id]
        );
        
        self.commit()
    }

    fn list_transactions(self: &Self) -> Option<Vec<Transaction>> {
        let res = self.query_as::<Transaction>("select * from transaction", &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn search_transaction_by_customer(self: &Self, fname: &String, lname: &String) -> Option<Vec<Transaction>> {
        let res = self.query_as::<Transaction>(
            "
                select *
                from transaction
                where customer_id in (
                    select customer_id
                    from customer
                    where first_name = :1 and last_name = :2
                )
            ",
            &[fname, lname]
        );

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn search_transaction_by_loan_type(self: &Self, loan_type: &String) -> Option<Vec<Transaction>> {
        let query = match loan_type.as_str() {
            "Auto"     => "select * from transaction where loan_id in ( select loan_id from auto_loan )",
            "Mortgage" => "select * from transaction where loan_id in ( select loan_id from mortgage_loan )",
            "Personal" => "select * from transaction where loan_id in ( select loan_id from personal_loan )",
            _ => unreachable!(),
        };

        let res = self.query_as::<Transaction>(query, &[]);

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

    fn search_transaction_by_date(self: &Self, date: NaiveDate) -> Option<Vec<Transaction>> {
        let res = self.query_as::<Transaction>(
            "select * from transaction where date = :1",
            &[&date]
        );

        if let Ok(rows) = res {
            return Some(rows.filter_map(|c| c.ok()).collect());
        }

        None
    }

}
