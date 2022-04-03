
use lazy_static::lazy_static;
use oracle::Connection;
use r2d2::{Pool, PooledConnection};
use r2d2_oracle::OracleConnectionManager;

use crate::{config, HOST_SERVICE};

lazy_static! {
    static ref POOL: Pool<OracleConnectionManager> = {
        let cfg = config::db_cred();
        let manager: OracleConnectionManager = OracleConnectionManager::new(
            &cfg.username,
            &cfg.password,
            HOST_SERVICE
        );

        Pool::builder()
        .max_size(10)
        .build(manager)
        .unwrap()
    };
}

pub fn get_cnxn() -> PooledConnection<OracleConnectionManager> {
    POOL.clone().get().unwrap()
}


pub fn get_one_cnxn() -> Connection {
    let cfg = config::db_cred();

    Connection::connect(cfg.username, cfg.password, HOST_SERVICE).unwrap()
}
