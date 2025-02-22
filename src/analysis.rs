// Given that it is mainly a technical assessment,
// the analytics carried out will be rudimentary

use std::collections::HashMap;
use crate::data::{Asset, Earthquake};
use itertools::Itertools;

// Simple unit enum for risk level
#[derive(Debug, Default)]
pub enum Risk {
    #[default]
    Low,
    Medium,
    High,
}

// We can take a reference of the earthquake data,
// since we are not manipulating it in place
pub fn earthquake_number_by_location(data: &[Earthquake]) -> HashMap<String, usize> {
    // Stats
    data.iter()
        //Shortcut: Given the brevity of the assiment,
        //taking the performance hit and cloning field string
        .map(|e| e.place.clone().split(",").last().unwrap_or("").to_string())
        .filter(|p| !p.is_empty())
        .counts()
}

pub fn average_mag_by_location(data: &[Earthquake]) -> HashMap<String, f32> {
    // Group data by state
    let state_data = data.iter()
        .into_group_map_by(|e| e.place.clone().split(",").last().unwrap_or("").trim().to_string());
   
    // Init our HashMap
    let mut average_mag_by_location: HashMap<String, f32> = HashMap::new();

    for (state, earthquakes) in state_data {
        let total_mag: f32 = earthquakes.iter()
            .map(|e| e.mag.unwrap_or(0.0))
            .sum();
        let earthquake_number = earthquakes.len() as f32;
        average_mag_by_location.insert(state, total_mag/earthquake_number);
    }

    average_mag_by_location
}

// Shortcut: Using generics so that this function works with both data hashMaps
// Works since both f32 and usize implment the PartialOrd trait
pub fn get_top_ten<T: PartialOrd>(data: HashMap<String, T>) -> Vec<(String, T)> {
    let mut kv_pairs: Vec<_> = data.into_iter().collect();
    kv_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    kv_pairs.into_iter().take(10).collect()
}

// Assigns the risk of a Client Asset
pub fn measure_risk(
    high_eq_occ_locations: Vec<&str>,
    high_mag_locations: Vec<&str>,
    client_data: Vec<Asset>
) -> Vec<(Asset, Risk)> {
    client_data.into_iter()
        .map(|a| assign_risk(&high_eq_occ_locations, &high_mag_locations, a))
        .collect()
}

// Assets risk is based on if they reside in either a high occarance or magnitude area
fn assign_risk(high_occ_locations: &[&str], high_mag_locations: &[&str], asset: Asset) -> (Asset, Risk) {
    let location = asset.location.split(',').last().unwrap_or("").trim();

    let is_high_occ = high_occ_locations.contains(&location);
    let is_high_mag = high_mag_locations.contains(&location);

    if is_high_occ && is_high_mag {
        (asset, Risk::High)
    } else if is_high_occ || is_high_mag {
        (asset, Risk::Medium)
    } else {
        (asset, Risk::Low)
    }
}

