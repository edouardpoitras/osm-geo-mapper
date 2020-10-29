use crate::{
    operations,
    features::{
        barrier_feature::get_barrier_geo_tile, highway_feature::get_highway_geo_tile,
        natural_feature::get_natural_geo_tile, GeoTile, GeoTileProperties, GeoTilesDataStructure,
        Geometry,
    }
};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn draw_point(
    point: &gt::Point<f64>,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    let coord = point_to_coordinates(point);
    data_structure.insert(coord, geo_tile);
}

pub fn point_feature_to_geo_tile(properties: &GeoTileProperties, point: gt::Point<f64>) -> GeoTile {
    let point = Geometry::Point(point);
    if properties.contains_key("natural") {
        get_natural_geo_tile(properties, point)
    } else if properties.contains_key("highway") {
        get_highway_geo_tile(properties, point, false)
    } else if properties.contains_key("barrier") {
        get_barrier_geo_tile(properties, point)
    } else {
        warn!(
            "Unclassified point feature geo tile found: {:?}",
            properties
        );
        let osm_id = properties["id"].to_string();
        GeoTile::Unclassified {
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
