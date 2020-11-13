use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, NaturalType},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn get_natural_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let natural = props["natural"].as_str().unwrap();
    let natural_type = match natural {
        "wood" => NaturalType::Wood,
        "tree_row" => NaturalType::TreeRow,
        "tree" => NaturalType::Tree,
        "scrub" => NaturalType::Scrub,
        "heath" => NaturalType::Heath,
        "moor" => NaturalType::Moor,
        "grassland" => NaturalType::Grassland,
        "fell" => NaturalType::Fell,
        "bare_rock" => NaturalType::BareRock,
        "scree" => NaturalType::Scree,
        "shingle" => NaturalType::Shingle,
        "sand" => NaturalType::Sand,
        "mud" => NaturalType::Mud,
        "water" => NaturalType::Water,
        "wetland" => NaturalType::Wetland,
        "glacier" => NaturalType::Glacier,
        "bay" => NaturalType::Bay,
        "strait" => NaturalType::Strait,
        "cape" => NaturalType::Cape,
        "beach" => NaturalType::Beach,
        "coastline" => NaturalType::Coastline,
        "reef" => NaturalType::Reef,
        "spring" => NaturalType::Spring,
        "hot_spring" => NaturalType::HotSpring,
        "geyser" => NaturalType::Geyser,
        "blowhole" => NaturalType::Blowhole,
        "peak" => NaturalType::Peak,
        "volcano" => NaturalType::Volcano,
        "valley" => NaturalType::Valley,
        "peninsula" => NaturalType::Peninsula,
        "isthmus" => NaturalType::Isthmus,
        "ridge" => NaturalType::Ridge,
        "arete" => NaturalType::Arete,
        "cliff" => NaturalType::Cliff,
        "saddle" => NaturalType::Saddle,
        "dune" => NaturalType::Dune,
        "rock" => NaturalType::Rock,
        "stone" => NaturalType::Stone,
        "sinkhole" => NaturalType::Sinkhole,
        "cave_entrance" => NaturalType::CaveEntrance,
        _ => {
            warn!("Unclassified natural type {}: {:?}", natural, props);
            NaturalType::Unclassified
        }
    };
    let access = property_to_option_string(props, "access");
    let circumference = property_to_option_string(props, "circumference");
    let denotation = property_to_option_string(props, "denotation");
    let direction = property_to_option_string(props, "direction");
    let ele = property_to_option_string(props, "ele");
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
        access,
        circumference,
        denotation,
        direction,
        ele,
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
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
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
        draw_line(&last_point, &point, 1, geo_tile.clone(), data_structure);
        last_point = point;
    }
}
