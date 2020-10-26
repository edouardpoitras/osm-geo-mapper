use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn convert_osm_to_geojson(
    osm_file: String,
    geojson_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file;
    if let Result::Ok(output) = Command::new("osmtogeojson").arg(&osm_file).output() {
        file = File::create(geojson_file)?;
        write!(file, "{}", String::from_utf8_lossy(&output.stdout))?;
    } else if let Result::Ok(output) = Command::new("osmtogeojson.cmd").arg(&osm_file).output() { // Windows
        file = File::create(geojson_file)?;
        write!(file, "{}", String::from_utf8_lossy(&output.stdout))?;
    } else {
        panic!("failed to convert osm file to geojson file - ensure you have the osmtogeojson tool installed (npm install -g osmtogeojson)");
    }
    Ok(())
}
