use mysql::{prelude::*, PooledConn, Pool};
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tranport {
    pub id: i64,
    pub name: String,
    pub t_type: String,
    pub fuel: String,
    pub unit: String,
    pub region: String,
    pub ef_factor: f64,
}

#[derive(Debug)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new() -> Option<Self> { 
        let url = env::var("DATABASE_URL_TEST").unwrap_or_default();
        let sql_options = mysql::Opts::from_url(&url).unwrap_or_default();
        let builder = mysql::OptsBuilder::from_opts(sql_options);
        let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default()));
        match pool {
            Ok(_pool) => {
                //let mut _conn = _pool.get_conn();
                Some(Database { pool: _pool })
            }
            Err(error) => {
                print!("{:?}",error);
                None
            }
        }
        
    }

    pub fn get_passenger_cars (&self) -> Option<Vec<Tranport>> {
        let pool = &self.pool;
        let poolCon = &mut pool.get_conn();
        match poolCon  {
            Ok(connection) => {
                let query_result = connection.query_map(
                    "SELECT id, tName, tType, fuel, unit, region, efFactor from `eco-friend`.Transport  where tType = 'Passenger Cars'",
                    |(id, tName, tType, fuel, unit, region, efFactor)| {
                        Tranport {
                            id,
                            name: tName,
                            t_type: tType,
                            fuel,
                            unit,
                            region,
                            ef_factor: efFactor
                        }
                    }
                );
                match query_result {
                    Ok(dataset) => {
                        Some(dataset)
                    }
                    Err(error) => {
                        print!("{:?}",error);
                        None
                    }
                    
                }
            }

            Err(error) => {
                println!("{:?}",error);
                None
            }
        }
    }
}

