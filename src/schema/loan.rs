
use chrono::Date;

struct Loan {
    loan_id: u32,
    customer_id: u32,
    loan_amount: f32,
    interest_rate: f32,
    amount_paid: f32,
    start_date: Date,
    end_date: Date,
    number_of_payments: u32
}
