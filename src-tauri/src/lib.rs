
pub mod admin;
pub mod auth;
pub mod customer;
pub mod loan;

pub fn get_oracle_error_text(error: oracle::DbError) -> String {
    let msg = String::from(error.message());
    let start = msg.find(' ').unwrap();
    let end = msg.find('\n').unwrap();
    
    String::from(&msg[start+1..end])
}

pub fn return_oracle_text(res: oracle::Result<oracle::Statement>) -> Result<(), String> {
    match res {
        Ok(_) => Ok(()),
        Err(oracle::Error::OciError(e)) => Err(get_oracle_error_text(e)),
        Err(e) => Err(e.to_string())
    }
}
