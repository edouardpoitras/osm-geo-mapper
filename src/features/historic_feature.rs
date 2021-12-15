use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, HistoricType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

// Driveways are treated specially because some cases only provide the "service" key without the "historic" key.
pub fn get_historic_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let historic_type_str = props.fetch("historic").unwrap();
    let historic_type = extract_type_from_string!(historic_type_str<props> => HistoricType [Aircraft, Aqueduct, ArchaeologicalSite, Battlefield, BombCrater, BoundaryStone, Building, Cannon, Castle, CastleWall, CharcoalPile, Church, CityGate, Citywalls, Farm, Fort, Gallows, HighwaterMark, Locomotive, Manor, Memorial, Milestone, Monastery, Monument, OpticalTelegraph, Pillory, RailwayCar, Ruins, RuneStone, Ship, Tank, Tomb, Tower, Unclassified, WaysideCross, WaysideShrine, Wreck]);
    geotile_from_properties!(geometry<props> => Historic<historic_type> [architect, artist_name, bridge, castle_type, collection, date, denomination, depth, description, disused, ele, flood_date, format, gauge, height, heritage, image, inscription, location, manufacturer, material, memorial, moved, name, network, operator, optical_telegraph, railway_car, religion, ruins, site_type, start_date, support, tomb, website, wikipedia, year]);
}

pub fn draw_historic_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _historic_type: HistoricType,
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
