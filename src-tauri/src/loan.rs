
use serde::{Serialize, Deserialize};

use crate::return_oracle_text;
use loans::{
    db,
    schema::{
        Loan, LoanOps,
        AutoLoan, AutoLoanOps,
        MortgageLoan, MortgageLoanOps,
        PersonalLoan, PersonalLoanOps
    }
};

#[derive(Default, Serialize, Deserialize)]
pub struct LoanChild {
    pub child: String,

    pub auto: Option<AutoLoan>,
    pub mortgage: Option<MortgageLoan>,
    pub personal: Option<PersonalLoan>
}

#[derive(Serialize, Deserialize)]
pub struct ExtLoanChild {
    pub loan: Loan,

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

#[tauri::command]
pub fn add_loan(data: ExtLoanChild) -> Result<(), String> {
    let conn = db::get_cnxn();

    let add_loan_res = conn.add_loan(&data.loan);
    if let Err(_) = add_loan_res {
        return return_oracle_text(add_loan_res);
    }

    let add_child_loan_res = match data.child.as_str() {
        "auto"     => conn.add_auto_loan(&data.auto.unwrap()),
        "mortgage" => conn.add_mortgage_loan(&data.mortgage.unwrap()),
        "personal" => conn.add_personal_loan(&data.personal.unwrap()),
        _ => unreachable!()
    };

    return_oracle_text(add_child_loan_res)
}

#[tauri::command]
pub fn update_loan(data: ExtLoanChild) -> Result<(), String> {
    let conn = db::get_cnxn();

    // remove child loan type
    let _ = conn.execute("delete from auto_loan where loan_id = :1" ,&[&data.loan.loan_id]);
    let _ = conn.execute("delete from mortgage_loan where loan_id = :1" ,&[&data.loan.loan_id]);
    let _ = conn.execute("delete from personal_loan where loan_id = :1" ,&[&data.loan.loan_id]);
    let _ = conn.commit();

    let update_loan_res = conn.edit_loan(&data.loan);
    if let Err(_) = update_loan_res {
        return return_oracle_text(update_loan_res);
    }

    let update_child_loan_res = match data.child.as_str() {
        "auto"     => conn.add_auto_loan(&data.auto.unwrap()),
        "mortgage" => conn.add_mortgage_loan(&data.mortgage.unwrap()),
        "personal" => conn.add_personal_loan(&data.personal.unwrap()),
        _ => unreachable!()
    };

    return_oracle_text(update_child_loan_res)
}

#[tauri::command]
pub fn delete_loan(id: u32) -> Result<(), String> {
    return_oracle_text( db::get_cnxn().remove_loan(&id) )
}
