
use chrono::naive::NaiveDate;

struct Transaction {
    transaction_id: u32,
    customer_id: u32,
    loan_id: u32,
    transaction_amount: f32,
    transaction_data: NaiveDate
}
