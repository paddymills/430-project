
use loans::{
    db,
    schema
};
use oracle::Result;
// use tabled::Table;

fn main() -> Result<()> {
    // add_users()
    test_delete()
}

#[allow(dead_code)]
fn add_users() -> Result<()> {

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

#[allow(dead_code)]
fn test_delete() -> Result<()> {
    use loans::schema::CustomerOps;

    match db::get_cnxn().remove_customer(&1u32) {
        Ok(_) => println!("Success!"),
        Err(oracle::Error::OciError(e)) => {
            let msg = String::from(e.message());
            let start = msg.find(' ').unwrap();
            let end = msg.find('\n').unwrap();

            println!("oracle OCI Error");
            println!("MESSAGE: {}\n-------------------------------------", &msg[start+1..end]);
            println!("FN: {}\n-------------------------------------", e.fn_name());
            println!("ACTION: {}\n-------------------------------------", e.action());
        },
        Err(e) => println!("General Error: {}", e.to_string())
    }

    // cnxn.commit()

    Ok(())
}
