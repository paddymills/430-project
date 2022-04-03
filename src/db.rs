
use oracle::Connection;
use r2d2_oracle::OracleConnectionManager;
use std::sync::Once;

use crate::{config, HOST_SERVICE};

static mut CNXN_POOL: r2d2::Pool<OracleConnectionManager>;
static INIT: Once = Once::new();

pub fn get_cnxn() -> Connection {
    let cfg = config::db_cred();

    unsafe {
        INIT.call_once(|| {
            let manager: OracleConnectionManager = OracleConnectionManager::new(
                &cfg.username,
                &cfg.password,
                HOST_SERVICE
            );

            CNXN_POOL = r2d2::Pool::builder()
                .max_size(10)
                .build(manager)
                .unwrap();
        });

    }
    
    CNXN_POOL.get().unwrap()
}


// pub fn get_cnxn() -> Connection {
//     let cfg = config::db_cred();

//     Connection::connect(cfg.username, cfg.password, HOST_SERVICE).unwrap()
// }
