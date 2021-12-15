use crate::{
    features::{RailwayType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_railway_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let railway_type_str = props.fetch("railway").unwrap();
    let railway_type = extract_type_from_string!(railway_type_str<props> => RailwayType [Abandoned, BufferStop, Construction, Crossing, Derail, Disused, Funicular, Halt, LevelCrossing, LightRail, Miniature, Monorail, NarrowGauge, Platform, Preserved, Rail, RailwayCrossing, Roundhouse, Signal, Station, Subway, SubwayEntrance, Switch, Tram, TramStop, Traverser, Turntable, Unclassified, Wash]);
    geotile_from_properties!(geometry<props> => Railway<railway_type> [access, area, bench, bicycle, bin, bridge, capacity, colour, control, crossing, cutting, disused, electrified, elevator, embankment, embedded_rails, fee, frequency, funicular, gauge, highspeed, incline, layer, length, light_rail, maxspeed, monorail, network, oneway, opening_hours, operator, passenger, public_transport, rack, request_stop, service, shelter, subway, supervised, surface, surveillance, tactile_paving, toilets, tracks, tram, tunnel, usage, voltage, wheelchair, width, workrules]);
}

pub fn draw_railway_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _barrier_type: RailwayType,
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
