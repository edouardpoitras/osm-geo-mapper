use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, NaturalType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_natural_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let natural_type_str = props.fetch("natural").unwrap();
    let natural_type = extract_type_from_string!(natural_type_str<props> => NaturalType [Wood, TreeRow, Tree, Scrub, Heath, Moor, Grassland, Fell, BareRock, Scree, Shingle, Sand, Mud, Water, Wetland, Glacier, Bay, Strait, Cape, Beach, Coastline, Reef, Spring, HotSpring, Geyser, Blowhole, Peak, Volcano, Valley, Peninsula, Isthmus, Ridge, Arete, Cliff, Saddle, Dune, Rock, Stone, Sinkhole, CaveEntrance, Unclassified]);
    geotile_from_properties!(geometry<props> => Natural<natural_type> [name, access, circumference, denotation, direction, ele, height, intermittent, genus, leaf_type, leaf_cycle, managed, operator, salt, species, surface, taxon, width]);
}

pub fn draw_natural_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _natural_type: NaturalType,
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
