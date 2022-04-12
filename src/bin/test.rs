
use loans::{
    db,
    schema
};
use oracle::Result;
// use tabled::Table;

fn main() -> Result<()> {

    let cnxn = db::get_cnxn();

    let mut sql = cnxn.prepare("insert into auth (username, pwd_hash, customer_id) values (:1, :2, :3)", &[])?;

    let users = [
        ("admin", "pwd123", None),
        ("cust1", "loans", Some(1)),
        ("cust2", "mortgage", Some(4)),
    ];

    for x in &users {
        sql.execute(&[&x.0, &schema::hash_pwd(x.1.into()), &x.2])?;
    }

    cnxn.commit()
}
