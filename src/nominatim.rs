use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LatLon {
    pub lat: String,
    pub lon: String,
}

pub fn get_address_lat_lon(
    address: String,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let query = format!(
        "https://nominatim.openstreetmap.org/?addressdetails=1&format=json&limit=1&q={}",
        address
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent("osm-geo-mapper")
        .build()?;
    let response = client.get(&query).send()?;
    let result = response.json::<Vec<LatLon>>()?;
    Ok((
        result[0].lat.parse::<f64>().unwrap(),
        result[0].lon.parse::<f64>().unwrap(),
    ))
}
