use geo_types as gt;
use geojson as gj;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
use log::warn;

use crate::{
    features::{Address, GeoTileProperties, GeoTilesDataStructure},
    operations::{
        line_string_operations::{draw_line_string, line_string_feature_to_geo_tile},
        point_operations::{draw_point, point_feature_to_geo_tile},
        polygon_operations::{draw_polygon, polygon_feature_to_geo_tile},
    },
};

pub mod line_string_operations;
pub mod point_operations;
pub mod polygon_operations;

pub fn property_to_option_string(props: &GeoTileProperties, key: &str) -> Option<String> {
    match props.get(key) {
        Some(value) => Some(value.as_str().unwrap().to_string()),
        _ => None,
    }
}

pub fn address_from_properties(props: &GeoTileProperties) -> Option<Address> {
    if props.contains_key("addr:housenumber")
        || props.contains_key("addr:unit")
        || props.contains_key("addr:street")
        || props.contains_key("addr:postcode")
    {
        let house_number = match props.get("addr:housenumber") {
            Some(value) => Some(String::from(value.as_str().unwrap_or_default())),
            _ => None,
        };
        let unit = match props.get("addr:unit") {
            Some(value) => Some(String::from(value.as_str().unwrap_or_default())),
            _ => None,
        };
        let street = match props.get("addr:street") {
            Some(value) => Some(String::from(value.as_str().unwrap_or_default())),
            _ => None,
        };
        let postal_code = match props.get("addr:postcode") {
            Some(value) => Some(String::from(value.as_str().unwrap_or_default())),
            _ => None,
        };
        Some(Address {
            house_number,
            unit,
            street,
            postal_code,
        })
    } else {
        None
    }
}

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

pub fn process_geojson(geojson: &gj::GeoJson) -> GeoTilesDataStructure {
    let mut data_structure = GeoTilesDataStructure::new();
    match *geojson {
        gj::GeoJson::FeatureCollection(ref ctn) => {
            for feature in &ctn.features {
                // Only process features that have properties and a geometry.
                if feature.properties.is_some() && feature.geometry.is_some() {
                    process_feature(
                        &feature.properties.as_ref().unwrap(),
                        &feature.geometry.as_ref().unwrap(),
                        &mut data_structure,
                    )
                } else {
                    warn!("Found feature from features without properties or geometry");
                }
            }
        }
        gj::GeoJson::Feature(ref feature) => {
            // Only process features that have properties and a geometry.
            if feature.properties.is_some() && feature.geometry.is_some() {
                process_feature(
                    &feature.properties.as_ref().unwrap(),
                    &feature.geometry.as_ref().unwrap(),
                    &mut data_structure,
                )
            } else {
                warn!("Found feature without properties or geometry");
            }
        }
        gj::GeoJson::Geometry(_) => {
            // For now, ignore hanging geometry types.
            //match_geometry(geometry, terrain_type, terrain_manager)
            warn!("Found top-level geometry")
        }
    }
    data_structure
}

fn process_feature(
    properties: &GeoTileProperties,
    geometry: &gj::Geometry,
    data_structure: &mut GeoTilesDataStructure,
) {
    match geometry.value {
        gj::Value::Polygon(_) => {
            let poly: gt::Polygon<f64> =
                TryInto::<gt::Polygon<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Rc::new(polygon_feature_to_geo_tile(properties, poly.clone()));
            draw_polygon(&poly, geo_tile, data_structure, false, false, false, false);
        }
        gj::Value::MultiPolygon(_) => {
            let multi_polygon: gt::MultiPolygon<f64> =
                TryInto::<gt::MultiPolygon<f64>>::try_into(geometry.value.clone()).unwrap();
            for polygon in multi_polygon {
                let poly: gt::Polygon<f64> =
                    TryInto::<gt::Polygon<f64>>::try_into(polygon).unwrap();
                let geo_tile = Rc::new(polygon_feature_to_geo_tile(properties, poly.clone()));
                draw_polygon(&poly, geo_tile, data_structure, false, false, false, false);
            }
        }
        gj::Value::GeometryCollection(ref gc) => {
            for geom in gc {
                process_feature(properties, geom, data_structure)
            }
        }
        gj::Value::LineString(_) => {
            let line_string: gt::LineString<f64> =
                TryInto::<gt::LineString<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Rc::new(line_string_feature_to_geo_tile(properties, line_string));
            draw_line_string(geo_tile, data_structure);
        }
        gj::Value::MultiLineString(_) => {
            let multi_line_string: gt::MultiLineString<f64> =
                TryInto::<gt::MultiLineString<f64>>::try_into(geometry.value.clone()).unwrap();
            for line_string in multi_line_string {
                let line_string: gt::LineString<f64> =
                    TryInto::<gt::LineString<f64>>::try_into(line_string).unwrap();
                let geo_tile = Rc::new(line_string_feature_to_geo_tile(properties, line_string));
                draw_line_string(geo_tile, data_structure);
            }
        }
        gj::Value::Point(_) => {
            let point: gt::Point<f64> =
                TryInto::<gt::Point<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Rc::new(point_feature_to_geo_tile(properties, point));
            draw_point(&point, geo_tile, data_structure);
        }
        gj::Value::MultiPoint(_) => {
            let multi_point: gt::MultiPoint<f64> =
                TryInto::<gt::MultiPoint<f64>>::try_into(geometry.value.clone()).unwrap();
            for point in multi_point {
                let point: gt::Point<f64> = TryInto::<gt::Point<f64>>::try_into(point).unwrap();
                let geo_tile = Rc::new(point_feature_to_geo_tile(properties, point));
                draw_point(&point, geo_tile, data_structure);
            }
        }
    }
}
