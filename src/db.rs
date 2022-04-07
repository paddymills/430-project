
use lazy_static::lazy_static;
use r2d2::{Error, Pool, PooledConnection};
use r2d2_oracle::OracleConnectionManager;

use crate::{config, HOST_SERVICE};

lazy_static! {
    static ref POOL: Pool<OracleConnectionManager> = create_conn_pool().unwrap();
}

pub fn create_conn_pool() -> Result<Pool<OracleConnectionManager>, Error> {
    let cfg = config::db_cred();
        let manager: OracleConnectionManager = OracleConnectionManager::new(
            &cfg.username,
            &cfg.password,
            HOST_SERVICE
        );

        Pool::builder()
            .max_size(10)
            .build(manager)
}

// method to force a database login on startup
pub fn init() -> Pool<OracleConnectionManager> {
    POOL.clone()
}

pub fn get_cnxn() -> PooledConnection<OracleConnectionManager> {
    POOL.clone().get().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::db;
    use oracle::Error;

    #[test]
    fn test_connection() {
        let cnxn = db::get_cnxn();

        match cnxn.query_row_as::<u32>("select 1 from customer", &[]) {
            Ok(row) => {
                assert_eq!(row, 1u32);
            },
            Err(Error::OciError(e)) => println!("OracleDB Error: {:?}", e.message()),
            Err(e) => println!("OracleDB Error: {:?}", e)
        }
    }
}
