mod db;
mod routes;
mod maps;

use db::Database;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer, http};
use crate::routes::*;
use google_maps::prelude::GoogleMapsClient;
use std::env;
use actix_cors::Cors;

pub struct AppState {
    pub db: Database,
    pub google_map_client: GoogleMapsClient
}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    // Load environment variables from .env file if present
    dotenv().ok();
    let mut database = match Database::new() {
        Some(database) => {
            database
        }
        None => {
            Database { pool: None }
        }
    };
    let api_key = env::var("GOOGLE_API_KEY").ok();
    let google_api_key = match api_key {
        Some(key) => key,
        None => {
            println!("unable to get google api key from env");
            "".to_string()
        }
    };
    let google_maps_client = GoogleMapsClient::new(&google_api_key);
    let app_data = web::Data::new(AppState {
        db: database,
        google_map_client: google_maps_client
    });
    
    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("http://localhost:8000")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
    App::new()
        .wrap(cors)
        .app_data(app_data.clone())
        .service(healthcheck)
        .service(get_driving_directions)
        .service(get_direction_metrix)
        .service(get_place_predications)
        .service(get_place_details)
        .default_service(web::route().to(not_found))
        .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}