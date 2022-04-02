
use oracle::Connection;
use crate::{config, HOST_SERVICE};

pub fn get_cnxn() -> Connection {
    let cfg = config::get_db_cred();

    Connection::connect(cfg.username, cfg.password, HOST_SERVICE).unwrap()
}
