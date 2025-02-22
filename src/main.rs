mod data;
mod analysis;
mod tui;

use analysis::{average_mag_by_location, earthquake_number_by_location, get_top_ten, measure_risk};
use data::{load_client_data, load_earthquake_data, Asset, Earthquake};
use tui::App;

// By making our simple application return a Result<()>,
// we greatly simplify error handling
#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    // Load client data
    // Shortcut: Using the ".expect" simplifies the error handling as it simply panics and halts
    // the program. Given more time the error would be ralayed gracefully in the UI
    let client_data: Vec<Asset> = load_client_data().expect("Failed to load client data");

    // load earthquake data
    // Since the function is async we have to await the data
    // Shortcut: Using the ".expect" simplifies the error handling as it simply panics and halts
    // the program. Given more time the error would be ralayed gracefully in the UI
    let earthquake_data: Vec<Earthquake> = load_earthquake_data().await.expect("Failed to load earthquake data");

    // filter for recent data
    let one_week_ago: chrono::DateTime<chrono::Utc> = chrono::Utc::now() - chrono::Duration::days(7);
    let recent_eq_data: Vec<Earthquake> = earthquake_data.into_iter()
        .filter(|e| e.time >= one_week_ago)
        .collect();

    // Stats
    let earthquake_occ_by_state: Vec<(String, usize)> =
        get_top_ten(earthquake_number_by_location(&recent_eq_data));
    let average_mag_by_state: Vec<(String, f32)> =
        get_top_ten(average_mag_by_location(&recent_eq_data));

    // Assign risk
    let high_occ_states = earthquake_occ_by_state.iter()
        .map(|(state, _)| state.as_str())
        .collect();

    let high_mag_states = average_mag_by_state.iter()
        .map(|(state, _)| state.as_str())
        .collect();

    // Render ui
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(
        measure_risk(high_occ_states, high_mag_states, client_data),
        earthquake_occ_by_state,
        average_mag_by_state).run(terminal);
    ratatui::restore();
    app_result
}


// Test
// Check if client data is loaded properly,
// Result types can be seen as intate checks/checks
#[cfg(test)]
mod tests {
    use crate::data::{load_client_data, Asset};

    #[test]
    fn test_loading_client_data() {
        let client_data = load_client_data().unwrap_or_default();
        let first_asset = Asset {
            building_name: "West Anchorage High School".to_string(),
            location: "Anchorage, Alask".to_string(),
            full_address: "1700 Hillcrest Dr, Anchorage, AK 9951".to_string(),
        };
        
        assert_eq!(client_data.first(), Some(&first_asset));
    }
}
