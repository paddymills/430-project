
use serde::{Serialize, Deserialize};

use loans::{
    db,
    schema::{LoanOps, Loan, AutoLoan, MortgageLoan, PersonalLoan}
};

#[derive(Default, Serialize, Deserialize)]
pub struct LoanChild {
    pub child: String,

    pub auto: Option<AutoLoan>,
    pub mortgage: Option<MortgageLoan>,
    pub personal: Option<PersonalLoan>
}

#[tauri::command]
pub fn get_loans() -> Option<Vec<Loan>> {
    db::get_cnxn().get_loans()
}

#[tauri::command]
pub fn get_loan(id: u32) -> Option<LoanChild> {
    let res = db::get_cnxn().query_row_as::<AutoLoan>(
        "select * from auto_loan where loan_id = :1 ", &[&id]
    );

    if let Ok(row) =  res {
        return Some(LoanChild {
            child: "auto".into(),
            auto: Some(row),

            ..Default::default()
        })
    }

    let res = db::get_cnxn().query_row_as::<MortgageLoan>(
        "select * from mortgage_loan where loan_id = :1 ", &[&id]
    );

    if let Ok(row) =  res {
        return Some(LoanChild {
            child: "mortgage".into(),
            mortgage: Some(row),

            ..Default::default()
        })
    }

    let res = db::get_cnxn().query_row_as::<PersonalLoan>(
        "select * from personal_loan where loan_id = :1 ", &[&id]
    );

    if let Ok(row) =  res {
        return Some(LoanChild {
            child: "personal".into(),
            personal: Some(row),

            ..Default::default()
        })
    }

    None
}