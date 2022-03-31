
use figment::{
    Figment,
    providers::{Format, Toml, Env}
};
use oracle::Connection;
use requestty::{Question, prompt_one};
use serde::Deserialize;

const HOST_SERVICE: &str = "//h3oracle.ad.psu.edu:1521/orclpdb.ad.psu.edu";

#[derive(Deserialize, Debug)]
struct Config {
    username: String,
    password: String
}

fn main() {

    let mut cfg: Config = Figment::new()
        .merge(Toml::file("db.toml"))
        .merge(Env::prefixed("ORCL_"))
        .extract()
        .unwrap();

    if cfg.password == "test" {
        let passwd = prompt_one(
            Question::password("password")
                .message("Password: ")
                .mask('*')
                .build()
        );
        cfg.password = passwd.unwrap().as_string().unwrap().into();
    }

    let cnxn = Connection::connect(cfg.username, cfg.password, HOST_SERVICE).unwrap();

    let sql = "select id, val from test where id > :1";
    let rows = cnxn.query_as::<(i32, String)>(sql, &[&1]).unwrap();
    for row in rows {
        let (owner, name) = row.unwrap();
        println!("{:} | {:}", owner, name);
    }
}
