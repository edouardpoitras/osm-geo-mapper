use std::{
    fs::File,
    io::{ BufReader, Read },
};
use geojson as gj;

pub fn parse_geojson_file(geojson_file: &str) -> gj::GeoJson {
    let mut input_file =
        BufReader::new(File::open(geojson_file).expect("Could not open input file"));
    let mut geojson_str = "".to_owned();
    input_file
        .read_to_string(&mut geojson_str)
        .expect("Could not read geojson data to string");
    geojson_str.parse::<gj::GeoJson>().unwrap()
    // TODO: Running into issues on Windows where osmtogeojson produces non UTF-8 files.
    //let mut geojson_data = b"".to_owned();
    //input_file
    //.read(&mut geojson_data)
    //.expect("Could not read geojson data");
    //let geojson_str = UTF_8.decode(&geojson_data, DecoderTrap::Strict).ok().unwrap();
    //let geojson_str_encoded = UTF_8.encode(&geojson_str, DecoderTrap::Strict).unwrap();
    //geojson_str_encoded.parse::<gj::GeoJson>().unwrap()
}