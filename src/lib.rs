pub extern crate geo_types;

pub mod features;
pub mod nominatim;
pub mod openstreetmap;
pub mod operations;
pub mod osmtogeojson;
pub mod viewer;

pub fn address_to_mapper(address: String, radius: Option<u32>) -> Result<operations::Mapper, Box<dyn std::error::Error>> {
    let (latitude, longitude) = nominatim::get_address_lat_lon(address)?;
    lat_lon_to_mapper(latitude, longitude, radius)
}

pub fn lat_lon_to_mapper(latitude: f64, longitude: f64, radius: Option<u32>) -> Result<operations::Mapper, Box<dyn std::error::Error>> {
    let rad = radius.unwrap_or(200);
    let radiusf = operations::from_tile_scale(rad as i32);
    let geojson_file = operations::get_geojson_file_by_lat_lon(latitude, longitude, radiusf)?;
    operations::get_mapper_from_geojson_file(
        geojson_file,
        rad,
        Some(
            operations::Location::Coordinates {
                latitude,
                longitude
            }
        )
    )
}

pub fn geojson_file_to_mapper(geojson_file: String, location: Option<operations::Location>) -> Result<operations::Mapper, Box<dyn std::error::Error>> {
    operations::get_mapper_from_geojson_file(geojson_file, 0, location)
}

pub fn load_more_geo_data_from_lat_lon(data_structure: features::GeoTilesDataStructure, latitude: f64, longitude: f64, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let radiusf = operations::from_tile_scale(radius.unwrap_or(200) as i32);
    let geojson_file = operations::get_geojson_file_by_lat_lon(latitude, longitude, radiusf)?;
    let geojson = operations::parse_geojson_file(&geojson_file.to_string());
    operations::process_geojson_with_data_structure(&geojson, data_structure);
    Ok(())
}

pub fn load_more_geo_data_from_address(data_structure: features::GeoTilesDataStructure, address: String, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let (latitude, longitude) = nominatim::get_address_lat_lon(address)?;
    load_more_geo_data_from_lat_lon(data_structure, latitude, longitude, radius)
}

pub fn load_more_geo_data_from_geojson_file(data_structure: features::GeoTilesDataStructure, geojson_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let geojson = operations::parse_geojson_file(&geojson_file.to_string());
    operations::process_geojson_with_data_structure(&geojson, data_structure);
    Ok(())
}