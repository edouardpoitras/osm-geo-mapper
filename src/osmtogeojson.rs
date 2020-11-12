use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{error::Error, fmt};

#[derive(Debug)]
struct ConvertError { message: String }

impl Error for ConvertError {}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn convert_osm_to_geojson(
    osm_file: String,
    geojson_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file;
    if let Result::Ok(output) = Command::new("osmtogeojson").arg(&osm_file).output() {
        file = match File::create(geojson_file) {
            Result::Ok(f) => f,
            Result::Err(err) => return Err(Box::new(err)),
        };
        match write!(file, "{}", String::from_utf8_lossy(&output.stdout)) {
            Result::Ok(_) => return Ok(()),
            Result::Err(err) => return Err(Box::new(err)),
        };
    } else if let Result::Ok(output) = Command::new("osmtogeojson.cmd").arg(&osm_file).output() { // Windows
        file = match File::create(geojson_file) {
            Result::Ok(f) => f,
            Result::Err(err) => return Err(Box::new(err)),
        };
        match write!(file, "{}", String::from_utf8_lossy(&output.stdout)) {
            Result::Ok(_) => return Ok(()),
            Result::Err(err) => return Err(Box::new(err)),
        };
    }
    Err(Box::new(ConvertError { message: "Failed to convert osm file to geojson file - ensure you have the osmtogeojson tool installed (npm install -g osmtogeojson)".to_string() }))
}
