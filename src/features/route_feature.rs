use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, RouteType},
    operations::{line_string_operations::draw_line, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn get_route_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let route = props["route"].as_str().unwrap();
    let route_type = match route {
        "bicycle" => RouteType::Bicycle,
        "bus" => RouteType::Bus,
        "canoe" => RouteType::Canoe,
        "detour" => RouteType::Detour,
        "ferry" => RouteType::Ferry,
        "foot" => RouteType::Foot,
        "hiking" => RouteType::Hiking,
        "horse" => RouteType::Horse,
        "inline_skates" => RouteType::InlineSkates,
        "light_rail" => RouteType::LightRail,
        "mtb" => RouteType::MTB,
        "piste" => RouteType::Piste,
        "power" => RouteType::Power,
        "railway" => RouteType::Railway,
        "road" => RouteType::Road,
        "running" => RouteType::Running,
        "ski" => RouteType::Ski,
        "subway" => RouteType::Subway,
        "train" => RouteType::Train,
        "tracks" => RouteType::Tracks,
        "tram" => RouteType::Tram,
        "trolleybus" => RouteType::Trolleybus,
        _ => {
            warn!("Unclassified route type {}", route);
            RouteType::Unclassified
        }
    };
    let bicycle = property_to_option_string(props, "bycicle");
    let colour = property_to_option_string(props, "colour");
    let description = property_to_option_string(props, "description");
    let distance = property_to_option_string(props, "distance");
    let duration = property_to_option_string(props, "duration");
    let foot = property_to_option_string(props, "foot");
    let from = property_to_option_string(props, "from");
    let name = property_to_option_string(props, "name");
    let network = property_to_option_string(props, "network");
    let oneway = property_to_option_string(props, "oneway");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let roundtrip = property_to_option_string(props, "roundtrip");
    let symbol = property_to_option_string(props, "symbol");
    let to = property_to_option_string(props, "to");
    GeoTile::Route {
        bicycle,
        colour,
        description,
        distance,
        duration,
        foot,
        from,
        geometry,
        name,
        network,
        oneway,
        operator,
        osm_id,
        roundtrip,
        route_type,
        symbol,
        to,
    }
}

pub fn draw_route_line_string(
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
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
            data_structure,
        );
        last_point = point;
    }
}
