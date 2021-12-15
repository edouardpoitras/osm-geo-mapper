use std::{
    collections::{ BTreeMap, HashMap },
    convert::TryInto,
    sync::{ Arc, RwLock },
};
use log::warn;
use geo_types as gt;
use geojson as gj;
use osmpbfreader::objects::{ OsmId, OsmObj };
use osm_xml;

use crate::{
    features::{Address, GeoTileProperties, GeoTilesDataStructure, TILE_SCALE},
    operations::{
        line_string_operations::{draw_line_string, line_string_feature_to_geo_tile},
        point_operations::{draw_point, point_feature_to_geo_tile},
        polygon_operations::{draw_polygon, polygon_feature_to_geo_tile},
    },
    openstreetmap,
    osmtogeojson,
    pbf_parser::HasCoordinates,
};

pub mod line_string_operations;
pub mod point_operations;
pub mod polygon_operations;

// Takes a lat/lon unit (f64) and converts it to a 2d grid coordinate unit using i32.
// This is a lossy operation.
pub fn to_tile_scale(unit: f64) -> i32 {
    return (unit * TILE_SCALE).round() as i32
}

// Takes a tile-scaled i32 unit and converts it back to a lat/lon scale unit (f64).
// This is not technically a lossy operation, but the initial convertion to tile scale would have been,
// therefor you can't expect to be able to convert back-and-forth without losing fidelity.
pub fn from_tile_scale(unit : i32) -> f64 {
    return (unit as f64) / TILE_SCALE;
}

// Same as from_tile_scale(i32) except takes a u8.
pub fn from_tile_scale_u8(unit : u8) -> f64 {
    return (unit as f64) / TILE_SCALE;
}

pub fn property_to_option_string(props: &dyn GeoTileProperties, key: &str) -> Option<String> {
    match props.fetch(key) {
        Some(value) => Some(value.to_string()),
        _ => None,
    }
}

pub fn address_from_properties(props: &dyn GeoTileProperties) -> Option<Address> {
    if props.has("addr:housenumber")
        || props.has("addr:unit")
        || props.has("addr:street")
        || props.has("addr:postcode")
    {
        let house_number = match props.fetch("addr:housenumber") {
            Some(value) => Some(String::from(value)),
            _ => None,
        };
        let unit = match props.fetch("addr:unit") {
            Some(value) => Some(String::from(value)),
            _ => None,
        };
        let street = match props.fetch("addr:street") {
            Some(value) => Some(String::from(value)),
            _ => None,
        };
        let postal_code = match props.fetch("addr:postcode") {
            Some(value) => Some(String::from(value)),
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

pub fn get_geojson_file_by_lat_lon(
    lat: f64,
    lon: f64,
    radius: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    let left = lon - radius;
    let bottom = lat - radius;
    let right = lon + radius;
    let top = lat + radius;
    let osm_file = openstreetmap::download_osm_data_by_bbox(left, bottom, right, top)?;
    let geojson_file = format!("{}.geojson", osm_file);
    osmtogeojson::convert_osm_to_geojson(osm_file, geojson_file.clone())?;
    Ok(geojson_file)
}

pub fn process_geojson(geojson: &gj::GeoJson) -> GeoTilesDataStructure {
    let data_structure = GeoTilesDataStructure::new(RwLock::new(HashMap::new()));
    process_geojson_with_data_structure(geojson, data_structure.clone());
    data_structure
}

pub fn process_osm(osm_data: &osm_xml::OSM) -> GeoTilesDataStructure {
    let data_structure = GeoTilesDataStructure::new(RwLock::new(HashMap::new()));
    process_osm_with_data_structure(osm_data, data_structure.clone());
    data_structure
}

pub fn process_pbf(pbf_data: &BTreeMap<OsmId, OsmObj>) -> GeoTilesDataStructure {
    let data_structure = GeoTilesDataStructure::new(RwLock::new(HashMap::new()));
    process_pbf_with_data_structure(pbf_data, data_structure.clone());
    data_structure
}

pub fn process_geojson_with_data_structure(geojson: &gj::GeoJson, data_structure: GeoTilesDataStructure) {
    match *geojson {
        gj::GeoJson::FeatureCollection(ref ctn) => {
            for feature in &ctn.features {
                // Only process features that have properties and a geometry.
                if feature.properties.is_some() && feature.geometry.is_some() {
                    process_feature(
                        feature.properties.as_ref().unwrap(),
                        &feature.geometry.as_ref().unwrap(),
                        data_structure.clone(),
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
                    feature.properties.as_ref().unwrap(),
                    &feature.geometry.as_ref().unwrap(),
                    data_structure,
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
}

pub fn process_osm_with_data_structure(osm_data: &osm_xml::OSM, data_structure: GeoTilesDataStructure) {
    // Nodes
    for (_, node) in osm_data.nodes.iter() {
        let point: gt::Point<f64> = (node.lat, node.lon).try_into().unwrap();
        let geo_tile = Arc::new(point_feature_to_geo_tile(&node.tags, point));
        draw_point(&point, geo_tile, data_structure.clone());
    }
    // Ways
    for (_, way) in osm_data.ways.iter() {
        let mut coordinates: Vec<(f64, f64)> = Vec::new();
        for node in way.nodes.iter() {
            match osm_data.resolve_reference(&node) {
                osm_xml::Reference::Node(n) => coordinates.push((n.lat, n.lon)),
                osm_xml::Reference::Unresolved  |
                osm_xml::Reference::Way(_)      |
                osm_xml::Reference::Relation(_) => {
                    warn!("Found a non-node as part of way {}'s node list: {:?}", way.id, node);
                }
            }
        }
        if way.is_polygon() { // Polygon
            let poly: gt::Polygon<f64> = gt::Polygon::new(coordinates.into(), vec![]);
            let geo_tile = Arc::new(polygon_feature_to_geo_tile(&way.tags, poly.clone()));
            draw_polygon(&poly, geo_tile, data_structure.clone());
        } else { // LineString
            let line_string: gt::LineString<f64> = coordinates.into();
            let geo_tile = Arc::new(line_string_feature_to_geo_tile(&way.tags, line_string));
            draw_line_string(geo_tile, data_structure.clone());
        }
    }
    // Relations
    // TODO: INCOMPLETE - not sure how to handle this scenario yet.
}

pub fn process_pbf_with_data_structure(pbf_data: &BTreeMap<OsmId, OsmObj>, data_structure: GeoTilesDataStructure) {
    for obj in pbf_data.values() {
        let mut tags = obj.tags().clone();
        tags.insert("id".to_string(), obj.id().inner_id().to_string());
        match obj {
            OsmObj::Node(obj) => {
                let point: gt::Point<f64> = (obj.lat(), obj.lon()).try_into().unwrap();
                let geo_tile = Arc::new(point_feature_to_geo_tile(&tags, point));
                draw_point(&point, geo_tile, data_structure.clone());
            }
            OsmObj::Way(obj) => {
                let coordinates = obj.get_coordinates(&pbf_data);
                if obj.is_open() { // LineString
                    let line_string: gt::LineString<f64> = coordinates.into();
                    let geo_tile = Arc::new(line_string_feature_to_geo_tile(&tags, line_string));
                    draw_line_string(geo_tile, data_structure.clone());
                } else { // Polygon
                    let poly: gt::Polygon<f64> = gt::Polygon::new(coordinates.into(), vec![]);
                    let geo_tile = Arc::new(polygon_feature_to_geo_tile(&tags, poly.clone()));
                    draw_polygon(&poly, geo_tile, data_structure.clone());
                }
            }
            OsmObj::Relation(_obj) => {
                //let coordinates = obj.get_coordinates(&pbf_data, &mut vec![]);
                // TODO: INCOMPLETE - not sure how to handle this scenario yet.
            }
        }
    }
}

fn process_feature(
    properties: &dyn GeoTileProperties,
    geometry: &gj::Geometry,
    data_structure: GeoTilesDataStructure,
) {
    match geometry.value {
        gj::Value::Polygon(_) => {
            let poly: gt::Polygon<f64> =
                TryInto::<gt::Polygon<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Arc::new(polygon_feature_to_geo_tile(properties, poly.clone()));
            draw_polygon(&poly, geo_tile, data_structure);
        }
        gj::Value::MultiPolygon(_) => {
            let multi_polygon: gt::MultiPolygon<f64> =
                TryInto::<gt::MultiPolygon<f64>>::try_into(geometry.value.clone()).unwrap();
            for polygon in multi_polygon {
                let poly: gt::Polygon<f64> =
                    TryInto::<gt::Polygon<f64>>::try_into(polygon).unwrap();
                let geo_tile = Arc::new(polygon_feature_to_geo_tile(properties, poly.clone()));
                draw_polygon(&poly, geo_tile, data_structure.clone());
            }
        }
        gj::Value::GeometryCollection(ref gc) => {
            for geom in gc {
                process_feature(properties, geom, data_structure.clone())
            }
        }
        gj::Value::LineString(_) => {
            let line_string: gt::LineString<f64> =
                TryInto::<gt::LineString<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Arc::new(line_string_feature_to_geo_tile(properties, line_string));
            draw_line_string(geo_tile, data_structure);
        }
        gj::Value::MultiLineString(_) => {
            let multi_line_string: gt::MultiLineString<f64> =
                TryInto::<gt::MultiLineString<f64>>::try_into(geometry.value.clone()).unwrap();
            for line_string in multi_line_string {
                let line_string: gt::LineString<f64> =
                    TryInto::<gt::LineString<f64>>::try_into(line_string).unwrap();
                let geo_tile = Arc::new(line_string_feature_to_geo_tile(properties, line_string));
                draw_line_string(geo_tile, data_structure.clone());
            }
        }
        gj::Value::Point(_) => {
            let point: gt::Point<f64> =
                TryInto::<gt::Point<f64>>::try_into(geometry.value.clone()).unwrap();
            let geo_tile = Arc::new(point_feature_to_geo_tile(properties, point));
            draw_point(&point, geo_tile, data_structure);
        }
        gj::Value::MultiPoint(_) => {
            let multi_point: gt::MultiPoint<f64> =
                TryInto::<gt::MultiPoint<f64>>::try_into(geometry.value.clone()).unwrap();
            for point in multi_point {
                let point: gt::Point<f64> = TryInto::<gt::Point<f64>>::try_into(point).unwrap();
                let geo_tile = Arc::new(point_feature_to_geo_tile(properties, point));
                draw_point(&point, geo_tile, data_structure.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geojson_parser;

    #[test]
    fn test_parse_and_process_geojson_file() {
        let geojson = geojson_parser::parse_geojson_file("resources/ottawa.xml.geojson");
        process_geojson(&geojson);
    }
}