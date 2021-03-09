use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, LanduseType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_landuse_geo_tile(props: &GeoTileProperties, geometry: Geometry, landcover: bool) -> GeoTile {
    let landuse_type_str = if landcover {
        props["landcover"].as_str().unwrap()
    } else {
        props["landuse"].as_str().unwrap()
    };
    let landuse_type = extract_type_from_string!(landuse_type_str<props> => LanduseType [Allotments, Basin, Brownfield, Cemetery, Commercial, Conservation, Construction, Depot, Farmland, Farmyard, Flowerbed, Forest, Garages, Grass, Greenfield, GreenhouseHorticulture, Industrial, Landfill, Meadow, Military, Orchard, PeatCutting, PlantNursery, Port, Quarry, Railway, RecreationGround, Religious, Reservoir, Residential, Retail, SaltPond, Unclassified, VillageGreen, Vineyard]);
    let address = address_from_properties(props);
    let barrier = property_to_option_string(props, "barrier");
    let crop = property_to_option_string(props, "crop");
    let denomination = property_to_option_string(props, "denomination");
    let genus = property_to_option_string(props, "genus");
    let industrial = property_to_option_string(props, "industrial");
    let leaf_cycle = property_to_option_string(props, "leaf_cycle");
    let leaf_type = property_to_option_string(props, "leaf_type");
    let meadow = property_to_option_string(props, "meadow");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let plant = property_to_option_string(props, "plant");
    let religion = property_to_option_string(props, "religion");
    let resource = property_to_option_string(props, "resource");
    let species = property_to_option_string(props, "species");
    let trees = property_to_option_string(props, "trees");
    GeoTile::Landuse {
        address,
        barrier,
        crop,
        denomination,
        genus,
        geometry,
        industrial,
        landuse_type,
        leaf_cycle,
        leaf_type,
        meadow,
        name,
        operator,
        osm_id,
        plant,
        religion,
        resource,
        species,
        trees,
    }
}

pub fn draw_landuse_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _landuse_type: LanduseType,
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
