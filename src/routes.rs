use actix_web::{get, HttpResponse, Responder, Result, post, web::{Json, Data}};
use google_maps::directions;
use serde::{Serialize, Deserialize};
use crate::maps::*;
use crate::AppState;
use crate::calculator::*;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectionRequest {
    pub from_long: f64,
    pub from_lat: f64,
    pub to_long: f64,
    pub to_lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlacesRequest {
    pub place: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceDetailsRequest {
    pub place_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressDirectionRequest {
    pub origin_address: String,
    pub destination_address: String
}

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/drivingDirections")]
pub async fn get_driving_directions(app: Data<AppState>, request: Json<DirectionRequest>) -> impl Responder {
    let response =  get_driving_direction(DirectionRequest { from_long: request.from_long, from_lat: request.from_lat, to_long: request.to_long, to_lat: request.to_lat }, app.google_map_client.clone()).await;
    match response {
        Some(direction) => {
            HttpResponse::Ok().json(direction.routes)
        }
        None => {
            HttpResponse::InternalServerError().json(Response {
                message: "Run into issues".to_string()
            })
        }
    }
}

#[post("/getDrivingDirectionsWithPlaceName")]
pub async fn get_driving_direction_with_place_name(app: Data<AppState>, request: Json<AddressDirectionRequest>) -> impl Responder {
    let response = get_driving_directions_by_address(
        app.google_map_client.clone(), 
        AddressDirectionRequest { 
            origin_address: request.origin_address.clone(), 
            destination_address: request.destination_address.clone() 
        }
        ).await;
        match response {
            Some(direction) => {
                HttpResponse::Ok().json(direction.routes)
            }
            None => {
                HttpResponse::InternalServerError().json(Response {
                    message: "Run into issues".to_string()
                })
            }
        }
}

#[post("/getTransitDirectionsWithPlaceName")]
pub async fn get_transit_direction_with_place_name(app: Data<AppState>, request: Json<AddressDirectionRequest>) -> impl Responder {
    let response = get_transit_directions_by_address(
        app.google_map_client.clone(), 
        AddressDirectionRequest { 
            origin_address: request.origin_address.clone(), 
            destination_address: request.destination_address.clone() 
        }
        ).await;
        match response {
            Some(direction) => {
                HttpResponse::Ok().json(direction.routes)
            }
            None => {
                HttpResponse::InternalServerError().json(Response {
                    message: "Run into issues".to_string()
                })
            }
        }
}

#[post("/getDirectionMatrix")]
pub async fn get_direction_metrix (request: Json<DirectionRequest>, app: Data<AppState>) -> impl Responder {
    let response = get_direction_with_matrix(DirectionRequest { from_long: request.from_long, from_lat: request.from_lat, to_long: request.to_long, to_lat: request.to_lat }, app.google_map_client.clone()).await;
    match response {
        Some(direction) => {
            HttpResponse::Ok().json(direction)
        }
        None => {
            HttpResponse::InternalServerError().json(Response {
                message: "Run into issues".to_string()
            })
        }
    }
}

#[post("/getPlacePredictions")]
pub async fn get_place_predications (request: Json<PlacesRequest>, app: Data<AppState>) -> impl Responder {
    let response = get_places(app.google_map_client.clone(), request.place.clone()).await;
    match response {
        Some(places) => {
            HttpResponse::Ok().json(places)
        }
        None => {
            HttpResponse::InternalServerError().json(Response {
                message: "Run into issues".to_string()
            })
        }
    }
}

#[post("/getPlaceDetails")]
pub async fn get_place_details (request: Json<PlaceDetailsRequest>, app: Data<AppState>) -> impl Responder {
    let response = get_place_details_gmaps(app.google_map_client.clone(), request.place_id.clone()).await;
    match response {
        Some(places) => {
            HttpResponse::Ok().json(places)
        }
        None => {
            HttpResponse::InternalServerError().json(Response {
                message: "Run into issues".to_string()
            })
        }
    }
}

#[get("/getPassengerCarEfFactors")]
pub async fn get_passenger_cars (app: Data<AppState>) -> impl Responder {
    let result = app.db.as_ref().and_then(|data| {
        data.get_passenger_cars()
    });
    
    match result {
        Some(places) => {
            HttpResponse::Ok().json(places)
        }
        None => {
            HttpResponse::InternalServerError().json(Response {
                message: "Run into issues".to_string()
            })
        }
    }
}


#[post("/getDrivingDirectionsCarbonFootPrint")]
pub async fn get_driving_carbon_foot_print_with_place_name(app: Data<AppState>, request: Json<AddressDirectionRequest>) -> impl Responder {
    let direction_response = get_driving_directions_by_address(
        app.google_map_client.clone(), 
        AddressDirectionRequest { 
            origin_address: request.origin_address.clone(), 
            destination_address: request.destination_address.clone() 
        }
        ).await;
    let passenger_car_results = app.db.as_ref().and_then(|data| {
        data.get_passenger_cars()
    });    
        match direction_response {
            Some(direction) => {
                match passenger_car_results {
                    Some(cars) => {
                        let result = calculate_carbon_footprint_for_car_rides(cars, direction).await;
                        HttpResponse::Ok().json(result)
                    }
                    None => {
                        HttpResponse::InternalServerError().json(Response {
                            message: "Run into issues".to_string()
                        })
                    }
                }
            }
            None => {
                HttpResponse::InternalServerError().json(Response {
                    message: "Run into issues".to_string()
                })
            }
        }
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}