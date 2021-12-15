use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, LanduseType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_landuse_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry, landcover: bool) -> GeoTile {
    let landuse_type_str = if landcover {
        props.fetch("landcover").unwrap()
    } else {
        props.fetch("landuse").unwrap()
    };
    let landuse_type = extract_type_from_string!(landuse_type_str<props> => LanduseType [Allotments, Basin, Brownfield, Cemetery, Commercial, Conservation, Construction, Depot, Farmland, Farmyard, Flowerbed, Forest, Garages, Grass, Greenfield, GreenhouseHorticulture, Industrial, Landfill, Meadow, Military, Orchard, PeatCutting, PlantNursery, Port, Quarry, Railway, RecreationGround, Religious, Reservoir, Residential, Retail, SaltPond, Unclassified, VillageGreen, Vineyard]);
    geotile_from_properties!(geometry<props> => Landuse<landuse_type> [name, barrier, crop, denomination, genus, industrial, leaf_cycle, leaf_type, meadow, operator, plant, religion, resource, species, trees]);
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
