use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, PowerType},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_power_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let power = props["power"].as_str().unwrap();
    let power_type = match power {
        "cable" => PowerType::Cable,
        "catenary_mast" => PowerType::CatenaryMast,
        "compensator" => PowerType::Compensator,
        "converter" => PowerType::Converter,
        "generator" => PowerType::Generator,
        "heliostat" => PowerType::Heliostat,
        "insulator" => PowerType::Insulator,
        "line" => PowerType::Line,
        "minor_line" => PowerType::MinorLine,
        "plant" => PowerType::Plant,
        "pole" => PowerType::Pole,
        "portal" => PowerType::Portal,
        "substation" => PowerType::Substation,
        "switch" => PowerType::Switch,
        "switchgear" => PowerType::Switchgear,
        "terminal" => PowerType::Terminal,
        "tower" => PowerType::Tower,
        "transformer" => PowerType::Transformer,
        _ => {
            warn!("Unclassified power type {}: {:?}", power, props);
            PowerType::Unclassified
        }
    };
    let busbar = property_to_option_string(props, "busbar");
    let cables = property_to_option_string(props, "cables");
    let circuits = property_to_option_string(props, "");
    let colour = property_to_option_string(props, "");
    let compensator = property_to_option_string(props, "");
    let design = property_to_option_string(props, "");
    let frequency = property_to_option_string(props, "");
    let height = property_to_option_string(props, "");
    let gas_insulated = property_to_option_string(props, "");
    let landuse = property_to_option_string(props, "");
    let line = property_to_option_string(props, "");
    let line_attachment = property_to_option_string(props, "");
    let line_management = property_to_option_string(props, "");
    let location = property_to_option_string(props, "");
    let manufacturer = property_to_option_string(props, "");
    let material = property_to_option_string(props, "");
    let name = property_to_option_string(props, "");
    let operator = property_to_option_string(props, "");
    let osm_id = props["id"].to_string();
    let phases = property_to_option_string(props, "");
    let poles = property_to_option_string(props, "");
    let start_date = property_to_option_string(props, "");
    let structure = property_to_option_string(props, "");
    let substation = property_to_option_string(props, "");
    let switch = property_to_option_string(props, "");
    let rating = property_to_option_string(props, "");
    let voltage = property_to_option_string(props, "");
    let windings = property_to_option_string(props, "");
    let wires = property_to_option_string(props, "");
    GeoTile::Power {
        busbar,
        cables,
        circuits,
        colour,
        compensator,
        design,
        frequency,
        height,
        gas_insulated,
        geometry,
        landuse,
        line,
        line_attachment,
        line_management,
        location,
        manufacturer,
        material,
        name,
        operator,
        osm_id,
        phases,
        poles,
        power_type,
        start_date,
        structure,
        substation,
        switch,
        rating,
        voltage,
        windings,
        wires,
    }
}

pub fn draw_power_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _power_type: PowerType,
    line_string: gt::LineString<f64>,
) {
    let points = line_string.into_points();
    let mut first_iteration = true;
    let mut last_point = points[0];
    for point in points {
        if first_iteration {
            first_iteration = false;
            continue;
        }
        draw_line(&last_point, &point, 1, geo_tile.clone(), data_structure.clone());
        last_point = point;
    }
}
