use log::warn;
use geo_types;
use crate::{features, nominatim, operations};

#[derive(Debug, Clone)]
pub enum Location {
    Coordinates {
        latitude: f64,
        longitude: f64
    },
    Center
}

#[derive(Debug, Clone)]
pub struct OSMGeoMapper {
    pub data_structure: features::GeoTilesDataStructure,
    pub coordinates: geo_types::Coordinate<i32>,
    pub radius: u32
}

impl OSMGeoMapper {
    pub fn from_geojson_file_with_radius(geojson_file: String, radius: u32, location: Option<Location>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>> {
        let geojson = operations::parse_geojson_file(&geojson_file);
        let data_structure = operations::process_geojson(&geojson);
        let coordinates = match location {
            Some(Location::Coordinates { latitude, longitude }) => {
                geo_types::Coordinate {
                    x: operations::to_tile_scale(longitude),
                    y: operations::to_tile_scale(latitude)
                }
            },
            Some(Location::Center) => {
                warn!("Finding center location of geojson file not supported yet");
                geo_types::Coordinate { x: 0, y: 0 }
            },
            None => geo_types::Coordinate { x: 0, y: 0 }
        };
        Ok(OSMGeoMapper { data_structure, coordinates, radius })
    }

    pub fn from_geojson_file(geojson_file: String, location: Option<Location>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>> {
        OSMGeoMapper::from_geojson_file_with_radius(geojson_file, 0, location)
    }

    pub fn from_address(address: String, radius: Option<u32>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>> {
        let (latitude, longitude) = nominatim::get_address_lat_lon(address)?;
        OSMGeoMapper::from_lat_lon(latitude, longitude, radius)
    }

    pub fn from_lat_lon(latitude: f64, longitude: f64, radius: Option<u32>) -> Result<OSMGeoMapper, Box<dyn std::error::Error>> {
        let rad = radius.unwrap_or(200);
        let radiusf = operations::from_tile_scale(rad as i32);
        let geojson_file = operations::get_geojson_file_by_lat_lon(latitude, longitude, radiusf)?;
        OSMGeoMapper::from_geojson_file_with_radius(
            geojson_file,
            rad,
            Some(
                Location::Coordinates {
                    latitude,
                    longitude
                }
            )
        )
    }

    pub fn load_more_from_lat_lon(&mut self, latitude: f64, longitude: f64, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
        let radiusf = operations::from_tile_scale(radius.unwrap_or(200) as i32);
        let geojson_file = operations::get_geojson_file_by_lat_lon(latitude, longitude, radiusf)?;
        let geojson = operations::parse_geojson_file(&geojson_file.to_string());
        operations::process_geojson_with_data_structure(&geojson, self.data_structure.clone());
        Ok(())
    }

    pub fn load_more_from_address(&mut self, address: String, radius: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
        let (latitude, longitude) = nominatim::get_address_lat_lon(address)?;
        self.load_more_from_lat_lon(latitude, longitude, radius)
    }

    pub fn load_more_from_geojson_file(&mut self, geojson_file: String) -> Result<(), Box<dyn std::error::Error>> {
        let geojson = operations::parse_geojson_file(&geojson_file.to_string());
        operations::process_geojson_with_data_structure(&geojson, self.data_structure.clone());
        Ok(())
    }

    pub fn get(&self, lat: i32, lon: i32) -> Option<features::GeoTile> {
        let locked_data_structure = self.data_structure.read().unwrap();
        let option_arc_geo_tile = locked_data_structure.get(&geo_types::Coordinate { x: lon, y: lat });
        if let Some(arc_geo_tile) = option_arc_geo_tile {
            Some(arc_geo_tile.as_ref().clone())
        } else {
            None
        }
    }

    pub fn get_real(&self, lat: f64, lon: f64) -> Option<features::GeoTile> {
        let lat = operations::to_tile_scale(lat);
        let lon = operations::to_tile_scale(lon);
        self.get(lat, lon)
    }

    pub fn atomic_clone(&self) -> OSMGeoMapper {
        OSMGeoMapper {
            data_structure: self.data_structure.clone(),
            coordinates: self.coordinates,
            radius: self.radius,
        }
    }
}