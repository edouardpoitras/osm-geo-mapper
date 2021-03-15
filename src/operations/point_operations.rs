use crate::{
    operations,
    features::{
        barrier_feature::get_barrier_geo_tile, building_feature::get_building_geo_tile,
        highway_feature::get_highway_geo_tile, natural_feature::get_natural_geo_tile,
        man_made_feature::get_man_made_geo_tile, leisure_feature::get_leisure_geo_tile,
        aerialway_feature::get_aerialway_geo_tile, place_feature::get_place_geo_tile,
        amenity_feature::get_amenity_geo_tile, power_feature::get_power_geo_tile,
        craft_feature::get_craft_geo_tile, public_transport_feature::get_public_transport_geo_tile,
        tourism_feature::get_tourism_geo_tile, emergency_feature::get_emergency_geo_tile,
        geological_feature::get_geological_geo_tile, healthcare_feature::get_healthcare_geo_tile,
        historic_feature::get_historic_geo_tile, military_feature::get_military_geo_tile,
        office_feature::get_office_geo_tile, railway_feature::get_railway_geo_tile,
        shop_feature::get_shop_geo_tile, telecom_feature::get_telecom_geo_tile,
        sport_feature::get_sport_geo_tile, water_feature::get_water_geo_tile,
        waterway_feature::get_waterway_geo_tile,
        GeoTile, UnclassifiedType, GeoTileProperties, GeoTilesDataStructure, Geometry,
    }
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn draw_point(
    point: &gt::Point<f64>,
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
) {
    let coord = point_to_coordinates(point);
    data_structure.write().unwrap().insert(coord, geo_tile);
}

pub fn point_feature_to_geo_tile(properties: &GeoTileProperties, point: gt::Point<f64>) -> GeoTile {
    let point = Geometry::Point(point);
    if properties.contains_key("aerialway") {
        get_aerialway_geo_tile(properties, point)
    } else if properties.contains_key("amenity") {
        get_amenity_geo_tile(properties, point)
    } else if properties.contains_key("barrier") {
        get_barrier_geo_tile(properties, point)
    } else if properties.contains_key("craft") {
        get_craft_geo_tile(properties, point)
    } else if properties.contains_key("leisure") {
        get_leisure_geo_tile(properties, point)
    } else if properties.contains_key("man_made") {
        get_man_made_geo_tile(properties, point)
    } else if properties.contains_key("military") {
        get_military_geo_tile(properties, point)
    } else if properties.contains_key("natural") {
        get_natural_geo_tile(properties, point)
    } else if properties.contains_key("office") {
        get_office_geo_tile(properties, point)
    } else if properties.contains_key("place") {
        get_place_geo_tile(properties, point)
    } else if properties.contains_key("healthcare") {
        get_healthcare_geo_tile(properties, point)
    } else if properties.contains_key("historic") {
        get_historic_geo_tile(properties, point)
    } else if properties.contains_key("emergency") {
        get_emergency_geo_tile(properties, point)
    } else if properties.contains_key("power") {
        get_power_geo_tile(properties, point)
    } else if properties.contains_key("public_transport") {
        get_public_transport_geo_tile(properties, point)
    } else if properties.contains_key("railway") {
        get_railway_geo_tile(properties, point)
    } else if properties.contains_key("shop") {
        get_shop_geo_tile(properties, point)
    } else if properties.contains_key("sport") {
        get_sport_geo_tile(properties, point)
    } else if properties.contains_key("telecom") {
        get_telecom_geo_tile(properties, point)
    } else if properties.contains_key("tourism") {
        get_tourism_geo_tile(properties, point)
    } else if properties.contains_key("water") {
        get_water_geo_tile(properties, point)
    } else if properties.contains_key("waterway") {
        get_waterway_geo_tile(properties, point)
    } else if properties.contains_key("geological") {
        get_geological_geo_tile(properties, point)
    // Less common corner cases.
    } else if properties.contains_key("highway") {
        get_highway_geo_tile(properties, point, false)
    // We need to check for office before we check for addr::* because all office
    // features should have the addr::* properties, like any other building.
    //} else if properties.contains_key("office") {
        //get_office_geo_tile(properties, polygon)
    } else if properties.contains_key("addr:housenumber") {
        get_building_geo_tile(properties, point, "yes")
    } else {
        warn!(
            "Unclassified point feature geo tile found: {:?}",
            properties
        );
        let osm_id = properties["id"].to_string();
        GeoTile::Unclassified {
            unclassified_type: UnclassifiedType::Unclassified,
            address: None,
            geometry: point,
            osm_id,
        }
    }
}

pub fn point_to_i32(point: &gt::Point<f64>) -> gt::Point<i32> {
    gt::Point::new(
        operations::to_tile_scale(point.x()),
        operations::to_tile_scale(point.y()),
    )
}

fn point_to_coordinates(point: &gt::Point<f64>) -> gt::Coordinate<i32> {
    gt::Coordinate {
        x: operations::to_tile_scale(point.x()),
        y: operations::to_tile_scale(point.y()),
    }
}
