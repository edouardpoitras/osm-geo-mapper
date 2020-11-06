use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub fn download_osm_data_by_bbox(
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "https://overpass-api.de/api/map?bbox={},{},{},{}",
        left, bottom, right, top
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent("osm-geo-mapper")
        .build()?;
    let response = client.get(&query).send()?;
    let result = response.text()?;
    let mut tempfile = temp_dir();
    tempfile.push(Uuid::new_v4().to_string());
    tempfile.set_extension("xml");
    let mut file = File::create(&tempfile)?;
    write!(file, "{}", result)?;
    Ok(tempfile.as_path().to_str().unwrap().to_string())
}
