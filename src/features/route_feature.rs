use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, RouteType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_route_geo_tile(props: &GeoTileProperties, geometry: Geometry, route: Option<&str>) -> GeoTile {
    let mut route_type_key = "route";
    if route.is_some() {
        route_type_key = route.unwrap();
    }
    let route_type_str = props[route_type_key].as_str().unwrap();
    let route_type = extract_type_from_string!(route_type_str<props> => RouteType [Bicycle, Bus, Canoe, Detour, Ferry, Foot, Hiking, Horse, IceSkate, InlineSkates, LightRail, MTB, Piste, Power, Railway, Road, Running, Ski, Subway, Train, Tracks, Tram, Trolleybus, Unclassified]);
    let address = address_from_properties(props);
    let area = property_to_option_string(props, "area");
    let bicycle = property_to_option_string(props, "bycicle");
    let colour = property_to_option_string(props, "colour");
    let description = property_to_option_string(props, "description");
    let distance = property_to_option_string(props, "distance");
    let duration = property_to_option_string(props, "duration");
    let fee = property_to_option_string(props, "fee");
    let foot = property_to_option_string(props, "foot");
    let from = property_to_option_string(props, "from");
    let lit = property_to_option_string(props, "lit");
    let name = property_to_option_string(props, "name");
    let network = property_to_option_string(props, "network");
    let oneway = property_to_option_string(props, "oneway");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let piste_difficulty = property_to_option_string(props, "piste:difficulty");
    let piste_type= property_to_option_string(props, "piste:type");
    let roundtrip = property_to_option_string(props, "roundtrip");
    let seasonal = property_to_option_string(props, "seasonal");
    let symbol = property_to_option_string(props, "symbol");
    let to = property_to_option_string(props, "to");
    GeoTile::Route {
        address,
        area,
        bicycle,
        colour,
        description,
        distance,
        duration,
        fee,
        foot,
        from,
        geometry,
        lit,
        name,
        network,
        oneway,
        operator,
        osm_id,
        piste_difficulty,
        piste_type,
        roundtrip,
        route_type,
        seasonal,
        symbol,
        to,
    }
}

pub fn draw_route_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    route_type: RouteType,
    line_string: gt::LineString<f64>,
) {
    let points = line_string.into_points();
    let mut first_iteration = true;
    let mut last_point = points[0];
    let thickness = match route_type {
        RouteType::Bus => 3,
        RouteType::LightRail => 3,
        RouteType::MTB => 3,
        RouteType::Railway => 3,
        RouteType::Road => 3,
        RouteType::Subway => 3,
        RouteType::Train => 3,
        RouteType::Tracks => 3,
        RouteType::Tram => 3,
        RouteType::Trolleybus => 3,
        _ => 1,
    };
    for point in points {
        if first_iteration {
            first_iteration = false;
            continue;
        }
        draw_line(
            &last_point,
            &point,
            thickness,
            geo_tile.clone(),
            data_structure.clone(),
        );
        last_point = point;
    }
}
