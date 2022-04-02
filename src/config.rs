
use figment::{
    Figment,
    providers::{Format, Toml}
};
use requestty::{Question, prompt_one};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub password: String
}

pub fn get_db_cred() -> Config {
    let mut cfg: Config = Figment::new()
        .merge(Toml::file("db.toml"))
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

    cfg
}
