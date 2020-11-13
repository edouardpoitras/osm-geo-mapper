use crate::{
    features::{AerowayType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn get_aeroway_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let aeroway = props["aeroway"].as_str().unwrap();
    let aeroway_type = match aeroway {
        "aerodrome" => AerowayType::Aerodrome,
        "apron" => AerowayType::Apron,
        "gate" => AerowayType::Gate,
        "hangar" => AerowayType::Hangar,
        "helipad" => AerowayType::Helipad,
        "heliport" => AerowayType::Heliport,
        "navigationaid" => AerowayType::Navigationaid,
        "runway" => AerowayType::Runway,
        "spaceport" => AerowayType::Spaceport,
        "taxiway" => AerowayType::Taxiway,
        "terminal" => AerowayType::Terminal,
        "windsock" => AerowayType::Windsock,
        _ => {
            warn!("Unclassified aeroway type {}: {:?}", aeroway, props);
            AerowayType::Unclassified
        }
    };
    let description = property_to_option_string(props, "description");
    let iata = property_to_option_string(props, "iata");
    let icao = property_to_option_string(props, "icao");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let surface = property_to_option_string(props, "surface");
    GeoTile::Aeroway {
        aeroway_type,
        description,
        geometry,
        iata,
        icao,
        name,
        operator,
        osm_id,
        surface,
    }
}

pub fn draw_aeroway_line_string(
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
    _aeroway_type: AerowayType,
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
