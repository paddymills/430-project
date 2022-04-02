
use loans::db;

fn main() {

    let cnxn = db::get_cnxn();

    let sql = "select id, val from test where id > :1";
    let rows = cnxn.query_as::<(i32, String)>(sql, &[&1]).unwrap();
    for row in rows {
        let (owner, name) = row.unwrap();
        println!("{:} | {:}", owner, name);
    }
}
