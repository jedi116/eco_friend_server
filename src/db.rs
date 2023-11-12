use mysql::{prelude::*, PooledConn};
use std::env;

pub struct Database {
    pub pool: Option<PooledConn>,
}

impl Database {
    pub fn new() -> Option<Self> { 
        let url = env::var("DATABASE_URL_TEST").unwrap_or_default();
        let sql_options = mysql::Opts::from_url(&url).unwrap_or_default();
        let builder = mysql::OptsBuilder::from_opts(sql_options);
        let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default()));
        match pool {
            Ok(_pool) => {
                let mut _conn = _pool.get_conn();
                match _conn {
                    Ok(connection) => {
                        println!("Successfully connected to PlanetScale!");
                        Some(Database { pool: Some(connection) })
                    }
                    Err(error) => {
                        print!("{:?}",error);
                        None  
                    }
                }
                
            }
            Err(error) => {
                print!("{:?}",error);
                None
            }
        }
        
    }
}

