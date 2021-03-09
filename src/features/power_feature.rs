use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, PowerType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_power_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let power_type_str = props["power"].as_str().unwrap();
    let power_type = extract_type_from_string!(power_type_str<props> => PowerType [Cable, CatenaryMast, Compensator, Converter, Generator, Heliostat, Insulator, Line, MinorLine, Plant, Pole, Portal, Substation, Switch, Switchgear, Terminal, Tower, Transformer, Unclassified]);
    let address = address_from_properties(props);
    let busbar = property_to_option_string(props, "busbar");
    let cables = property_to_option_string(props, "cables");
    let circuits = property_to_option_string(props, "circuits");
    let colour = property_to_option_string(props, "colour");
    let compensator = property_to_option_string(props, "compensator");
    let design = property_to_option_string(props, "design");
    let frequency = property_to_option_string(props, "frequency");
    let height = property_to_option_string(props, "height");
    let gas_insulated = property_to_option_string(props, "gas_insulated");
    let landuse = property_to_option_string(props, "landuse");
    let line = property_to_option_string(props, "line");
    let line_attachment = property_to_option_string(props, "line_attachment");
    let line_management = property_to_option_string(props, "line_management");
    let location = property_to_option_string(props, "location");
    let manufacturer = property_to_option_string(props, "manufacturer");
    let material = property_to_option_string(props, "material");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let phases = property_to_option_string(props, "phases");
    let poles = property_to_option_string(props, "poles");
    let start_date = property_to_option_string(props, "start_date");
    let structure = property_to_option_string(props, "structure");
    let substation = property_to_option_string(props, "substation");
    let switch = property_to_option_string(props, "switch");
    let rating = property_to_option_string(props, "rating");
    let voltage = property_to_option_string(props, "voltage");
    let windings = property_to_option_string(props, "windings");
    let wires = property_to_option_string(props, "wires");
    GeoTile::Power {
        address,
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
