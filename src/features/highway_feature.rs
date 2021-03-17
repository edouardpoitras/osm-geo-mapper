use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, HighwayType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

// Driveways are treated specially because some cases only provide the "service" key without the "highway" key.
pub fn get_highway_geo_tile(props: &GeoTileProperties, geometry: Geometry, driveway: bool) -> GeoTile {
    let highway_type_str = if driveway {
        "service"
    } else {
        props["highway"].as_str().unwrap()
    };
    let highway_type = extract_type_from_string!(highway_type_str<props> => HighwayType [Bridleway, BusGuideway, BusStop, Construction, Corridor, Crossing, Cycleway, Escape, Footway, LivingStreet, Motorway, MotorwayLink, Path, Pedestrian, Primary, PrimaryLink, Proposed, Raceway, Road, Residential, Secondary, SecondaryLink, Service, Steps, Stop, StreetLamp, Tertiary, TertiaryLink, Track, TrafficSignals, Trunk, TrunkLink, TurningCircle, Unclassified]);
    geotile_from_properties!(geometry<props> => Highway<highway_type> [name, abutters, access, bicycle, bus, destination, expressway, foot, hgv, lanes, lit, maxspeed, motor_vehicle, motorcar, motorroad, oneway, operator, service, shelter, sidewalk, sport, smoothness, surface, tracktype, wheelchair, width]);
}

pub fn draw_highway_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    highway_type: HighwayType,
    line_string: gt::LineString<f64>,
) {
    let points = line_string.into_points();
    let mut first_iteration = true;
    let mut last_point = points[0];
    let thickness = match highway_type {
        HighwayType::Motorway => 11,
        HighwayType::MotorwayLink => 11,
        HighwayType::Trunk => 9,
        HighwayType::TrunkLink => 9,
        HighwayType::Primary => 7,
        HighwayType::PrimaryLink => 7,
        HighwayType::Secondary => 7,
        HighwayType::SecondaryLink => 7,
        HighwayType::Tertiary => 5,
        HighwayType::TertiaryLink => 5,
        HighwayType::Residential => 5,
        HighwayType::LivingStreet => 5,
        HighwayType::Bridleway => 5,
        HighwayType::Road => 5,
        HighwayType::Track => 5,
        HighwayType::Raceway => 5,
        HighwayType::BusGuideway => 5,
        HighwayType::Service => 3,
        HighwayType::Corridor => 3,
        HighwayType::Escape => 3,
        HighwayType::TurningCircle => 3,
        HighwayType::BusStop => 2,
        HighwayType::Crossing => 2,
        HighwayType::Cycleway => 2,
        HighwayType::Pedestrian => 2,
        HighwayType::Footway => 2,
        HighwayType::Steps => 2,
        HighwayType::Path => 1,
        HighwayType::Proposed => 1,
        HighwayType::Construction => 1,
        HighwayType::Stop => 1,
        HighwayType::StreetLamp => 1,
        HighwayType::TrafficSignals => 1,
        HighwayType::Unclassified => 1,
    };
    for point in points {
        if first_iteration {
            first_iteration = false;
            continue;
        }
        draw_line(
            &last_point,
            &point,
            thickness,
            geo_tile.clone(),
            data_structure.clone(),
        );

        

        last_point = point;
    }
}
