use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, PowerType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_power_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let power_type_str = props.fetch("power").unwrap();
    let power_type = extract_type_from_string!(power_type_str<props> => PowerType [Cable, CatenaryMast, Compensator, Converter, Generator, Heliostat, Insulator, Line, MinorLine, Plant, Pole, Portal, Substation, Switch, Switchgear, Terminal, Tower, Transformer, Unclassified]);
    geotile_from_properties!(geometry<props> => Power<power_type> [name, busbar, cables, circuits, colour, compensator, design, frequency, height, gas_insulated, landuse, line, line_attachment, line_management, location, manufacturer, material, operator, phases, poles, start_date, structure, substation, switch, rating, voltage, windings, wires]);
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
