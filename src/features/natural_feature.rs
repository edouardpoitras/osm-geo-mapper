use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, NaturalType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_natural_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let natural_type_str = props["natural"].as_str().unwrap();
    let natural_type = extract_type_from_string!(natural_type_str<props> => NaturalType [Wood, TreeRow, Tree, Scrub, Heath, Moor, Grassland, Fell, BareRock, Scree, Shingle, Sand, Mud, Water, Wetland, Glacier, Bay, Strait, Cape, Beach, Coastline, Reef, Spring, HotSpring, Geyser, Blowhole, Peak, Volcano, Valley, Peninsula, Isthmus, Ridge, Arete, Cliff, Saddle, Dune, Rock, Stone, Sinkhole, CaveEntrance, Unclassified]);
    let address = address_from_properties(props);
    let access = property_to_option_string(props, "access");
    let circumference = property_to_option_string(props, "circumference");
    let denotation = property_to_option_string(props, "denotation");
    let direction = property_to_option_string(props, "direction");
    let elevation = property_to_option_string(props, "ele");
    let height = property_to_option_string(props, "height");
    let intermittent = property_to_option_string(props, "intermittent");
    let genus = property_to_option_string(props, "genus");
    let leaf_type = property_to_option_string(props, "leaf_type");
    let leaf_cycle = property_to_option_string(props, "leaf_cycle");
    let managed = property_to_option_string(props, "managed");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let salt = property_to_option_string(props, "salt");
    let species = property_to_option_string(props, "species");
    let surface = property_to_option_string(props, "surface");
    let taxon = property_to_option_string(props, "taxon");
    let width = property_to_option_string(props, "width");
    GeoTile::Natural {
        address,
        access,
        circumference,
        denotation,
        direction,
        elevation,
        height,
        intermittent,
        genus,
        geometry,
        leaf_type,
        leaf_cycle,
        managed,
        natural_type,
        name,
        operator,
        osm_id,
        salt,
        species,
        surface,
        taxon,
        width,
    }
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
