pub use crate::types::*;
use mysql::*;
use mysql::prelude::*;

pub struct Database<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub ip: &'a str,
    pub port: i32,
    pub database: &'a str
}

impl Database<'_> {
    pub fn connect(&self) -> Result<PooledConn, mysql::Error> {
        let pool = Pool::new(
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.ip,
                self.port,
                self.database
            ).as_str()
        )?;
        pool.get_conn()
    }
}
