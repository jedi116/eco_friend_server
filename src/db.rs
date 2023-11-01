use mysql::{prelude::*, PooledConn};
use std::env;
pub struct Database {
    pub pool: PooledConn,
}

impl Database {
    pub fn new() -> Self { 
        let url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL not found");
        let sql_options = mysql::Opts::from_url(&url).expect("Problem parsing data base url");
        let builder = mysql::OptsBuilder::from_opts(sql_options);
        let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
        let mut _conn = pool.get_conn().unwrap();
        println!("Successfully connected to PlanetScale!");
        Database { pool: _conn }
    }
}

