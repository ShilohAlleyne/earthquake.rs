use std::error::Error;

// Datatypes and functions for loading client
// and earthquake data

// Cleint Data
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Asset {
    pub building_name: String,
    pub location: String,
    pub full_address: String,
}

// USGS data
// Not all fields may be present
// we can represent that with an Option type
#[derive(Debug,  serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Earthquake {
    pub time: chrono::DateTime<chrono::Utc>,
    latitude: f32,
    longitude: f32,
    depth: Option<f32>,
    pub mag: Option<f32>,
    mag_type: String,
    nst: Option<u32>,
    gap: Option<f32>,
    dmin: Option<f32>,
    rms: Option<f32>,
    net: String,
    id: String,
    updated: chrono::DateTime<chrono::Utc>,
    pub place: String,
    #[serde(rename = "type")]
    pub eq_type: String,
    horizontal_error: Option<f32>,
    depth_error: Option<f32>,
    mag_error: Option<f32>,
    mag_nst: Option<f32>,
    status: String,
    pub location_source: String,
    mag_source: String,
}

// Load client data function we return our serialized data or
// "Any Error", which we can cleanly relay to the user
pub fn load_client_data() -> Result<Vec<Asset>, Box<dyn Error>> {
    // Load client data, given the scope of this project,
    // we can hard code this path to the csv verison of the provided table
    let mut rdr = csv::Reader::from_path("data/clientlocations.csv")?;

    // Serialize, return error if encountered
    let mut data = Vec::new();
    for res in rdr.deserialize() {
        let record: Asset = res?;
        data.push(record);
    }

    Ok(data)
}

// Load earthquake data (from the start of the year),
// also using a generic error data type
pub async fn load_earthquake_data() -> Result<Vec<Earthquake>,Box<dyn Error>> {
    let body: String = reqwest::get(
    "https://earthquake.usgs.gov/fdsnws/event/1/query?format=csv&starttime=2025-01-21&endtime=2025-02-21")
        .await?
        .text()
        .await?;

    // Read the csv as bytes
    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    // Serialize, return error if encountered
    let mut data = Vec::new();
    for res in rdr.deserialize() {
        let record: Earthquake = res?;
        data.push(record);
    }

    // Clean serialized data
    let filtered_data = data.into_iter()
        .filter(|e| e.eq_type == "earthquake") // filter non earthquakes
        .filter(|e| e.location_source != "hi") // filter hawaii locations
        .filter(|e| e.location_source != "hv")
        .filter(|e| e.location_source != "us") // filter generalised us data
        .collect::<Vec<Earthquake>>();

    Ok(filtered_data)
}

