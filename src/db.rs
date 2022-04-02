
use oracle::Connection;
use crate::{config, HOST_SERVICE};

pub fn get_cnxn() -> Connection {
    let cfg = config::get_db_cred();

    Connection::connect(cfg.username, cfg.password, HOST_SERVICE).unwrap()
}

// let sql = "select id, val from test where id > :1";
// let rows = cnxn.query_as::<(i32, String)>(sql, &[&1]).unwrap();
// for row in rows {
//     let (owner, name) = row.unwrap();
//     println!("{:} | {:}", owner, name);
// }
