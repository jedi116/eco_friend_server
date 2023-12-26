
use google_maps::{prelude::*, places::place_details};
use serde::{Deserialize, Serialize};
use crate::db::Tranport;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalculatedTranportCarbonFootPrint {
    name: String,
    t_type: String,
    fuel: String,
    unit: String,
    region: String,
    footprint: f64
}


pub async fn calculate_carbon_footprint_for_car_rides (transports: Vec<Tranport>, directions: DirectionsResponse ) -> Vec<Vec<CalculatedTranportCarbonFootPrint>>{
    let routes: Vec<f64> = directions.routes.iter().map(|data| {
        let mut distance = 0.0;
        data.legs.iter().for_each(|leg| {
            distance+= leg.distance.value as f64
        });
        distance
    }).collect();
    let calculated_result:Vec<Vec<CalculatedTranportCarbonFootPrint>> = routes.iter().map(|distance| {
        let result: Vec<CalculatedTranportCarbonFootPrint> = transports.iter().map(|transport| {
            CalculatedTranportCarbonFootPrint {
                            name: transport.name.clone(),
                            t_type: transport.t_type.clone(),
                            fuel: transport.fuel.clone(),
                            unit: transport.unit.clone(),
                            region: transport.region.clone(),
                            footprint: transport.ef_factor * distance
                        }
        }).collect();
        result
    }).collect();
    calculated_result
}