
use loans::{
    db,
    schema::Customer
};
use oracle::Error;
use tabled::Table;

fn main() {

    let cnxn = db::get_cnxn();

    let sql = "select * from customer";
    match cnxn.query_as::<Customer>(sql, &[]) {
        Ok(rows) => {
            let r: Vec<Customer> = rows.filter_map(|c| c.ok()).collect();
            println!("{:}", Table::new(r).to_string());
        },
        Err(Error::OciError(e)) => println!("OracleDB Error: {:?}", e.message()),
        Err(e) => println!("OracleDB Error: {:?}", e)
    }
}
