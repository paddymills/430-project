
use loans::db;
use oracle::Error;

fn main() {

    let cnxn = db::get_cnxn();

    let sql = "select id, val from tests where id > :1";
    match cnxn.query_as::<(i32, String)>(sql, &[&1u32]) {
        Ok(rows) => {
            for row in rows {
                let (owner, name) = row.unwrap();
                println!("{:} | {:}", owner, name);
            }
        },
        Err(Error::OciError(e)) => println!("OracleDB Error: {:?}", e.message()),
        Err(e) => println!("OracleDB Error: {:?}", e)
    }
}
