
use google_maps::{prelude::*, places::place_details};
use crate::routes::*;
use std::env;

pub async fn get_driving_direction (direction_request: DirectionRequest, google_maps_client: GoogleMapsClient) -> Option<DirectionsResponse> {
    let start = LatLng::try_from_f64(direction_request.from_lat, direction_request.from_long).unwrap_or_default();
    let destination = LatLng::try_from_f64(direction_request.to_lat, direction_request.to_long).unwrap_or_default();
    let directions = google_maps_client.directions(
        Location::LatLng(start),
        Location::LatLng(destination),
    )
    .with_travel_mode(TravelMode::Driving)
    .execute()
    .await;
    match directions {
        Err(message) => {
            print!("{}",message);
            None
        }
        Ok(res) => {
            Some(res)
        } 
    }
}

pub async fn get_direction_with_matrix (direction_request: DirectionRequest, google_maps_client: GoogleMapsClient) -> Option<DistanceMatrixResponse> {
    let start = LatLng::try_from_f64(direction_request.from_lat, direction_request.from_long).unwrap_or_default();
    let destination = LatLng::try_from_f64(direction_request.to_lat, direction_request.to_long).unwrap_or_default();
    let start_way_point = Waypoint::LatLng(start);
    let destination_way_point = Waypoint::LatLng(destination);
    let direction_matrix =  google_maps_client.distance_matrix(
        vec![
            start_way_point
        ],
        vec![
            destination_way_point
        ]
    ).execute().await;

    match direction_matrix {
        Err(message) => {
            print!("{}",message);
            None
        },
        Ok(res) => {
            Some(res)
        }
    }
}

pub async fn get_places (google_maps_client: GoogleMapsClient, place: String) -> Option<AutocompleteResponse> {
    let places_response = google_maps_client.place_autocomplete(place).execute().await;
    match places_response {
        Err(message) => {
            print!("{}",message);
            None
        }
        Ok(res) => {
            Some(res)
        } 
    }
}

pub async fn get_place_details_gmaps (google_maps_client: GoogleMapsClient, place_id: String) -> Option<PlaceDetailsResponse> {
    let place_details = google_maps_client.place_details(place_id).execute().await;
    match place_details {
        Err(message) => {
            print!("{}",message);
            None
        }
        Ok(res) => {
            Some(res)
        } 
    }
}