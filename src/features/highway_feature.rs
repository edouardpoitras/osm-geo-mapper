use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, HighwayType},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

// Driveways are treated specially because some cases only provide the "service" key without the "highway" key.
pub fn get_highway_geo_tile(
    props: &GeoTileProperties,
    geometry: Geometry,
    driveway: bool,
) -> GeoTile {
    let highway = if driveway {
        "service"
    } else {
        props["highway"].as_str().unwrap()
    };
    let highway_type = match highway {
        "bridleway" => HighwayType::Bridleway,
        "bus_guideway" => HighwayType::BusGuideway,
        "bus_stop" => HighwayType::BusStop,
        "construction" => HighwayType::Construction,
        "corridor" => HighwayType::Corridor,
        "crossing" => HighwayType::Crossing,
        "cycleway" => HighwayType::Cycleway,
        "escape" => HighwayType::Escape,
        "footway" => HighwayType::Footway,
        "living_street" => HighwayType::LivingStreet,
        "motorway" => HighwayType::Motorway,
        "motorway_link" => HighwayType::Motorway,
        "path" => HighwayType::Path,
        "pedestrian" => HighwayType::Pedestrian,
        "primary" => HighwayType::Primary,
        "primary_link" => HighwayType::PrimaryLink,
        "proposed" => HighwayType::Proposed,
        "raceway" => HighwayType::Raceway,
        "residential" => HighwayType::Residential,
        "road" => HighwayType::Road,
        "secondary" => HighwayType::Secondary,
        "secondary_link" => HighwayType::SecondaryLink,
        "service" => HighwayType::Service,
        "steps" => HighwayType::Steps,
        "stop" => HighwayType::Stop,
        "tertiary" => HighwayType::Tertiary,
        "tertiary_link" => HighwayType::TertiaryLink,
        "track" => HighwayType::Track,
        "traffic_signals" => HighwayType::TrafficSignals,
        "trunk" => HighwayType::Trunk,
        "trunk_link" => HighwayType::TrunkLink,
        "turning_circle" => HighwayType::TurningCircle,
        _ => {
            warn!("New highway type {}: {:?}", highway, props);
            HighwayType::Unclassified
        }
    };
    let abutters = property_to_option_string(props, "abutters");
    let access = property_to_option_string(props, "access");
    let bicycle = property_to_option_string(props, "bicycle");
    let bus = property_to_option_string(props, "bus");
    let destination = property_to_option_string(props, "destination");
    let expressway = property_to_option_string(props, "expressway");
    let foot = property_to_option_string(props, "foot");
    let hgv = property_to_option_string(props, "hgv");
    let lanes = property_to_option_string(props, "lanes");
    let lit = property_to_option_string(props, "lit");
    let maxspeed = property_to_option_string(props, "maxspeed");
    let motor_vehicle = property_to_option_string(props, "motor_vehicle");
    let motorcar = property_to_option_string(props, "motorcar");
    let motorroad = property_to_option_string(props, "motorroad");
    let name = property_to_option_string(props, "name");
    let oneway = property_to_option_string(props, "oneway");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let service = property_to_option_string(props, "service");
    let shelter = property_to_option_string(props, "shelter");
    let sidewalk = property_to_option_string(props, "sidewalk");
    let sport = property_to_option_string(props, "sport");
    let smoothness = property_to_option_string(props, "smoothness");
    let surface = property_to_option_string(props, "surface");
    let tracktype = property_to_option_string(props, "tracktype");
    let wheelchair = property_to_option_string(props, "wheelchair");
    let width = property_to_option_string(props, "width");
    GeoTile::Highway {
        abutters,
        access,
        bicycle,
        bus,
        destination,
        expressway,
        foot,
        geometry,
        hgv,
        highway_type,
        lanes,
        lit,
        maxspeed,
        motor_vehicle,
        motorcar,
        motorroad,
        name,
        oneway,
        operator,
        osm_id,
        service,
        shelter,
        sidewalk,
        sport,
        smoothness,
        surface,
        tracktype,
        wheelchair,
        width,
    }
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
