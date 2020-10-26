use crate::{
    features::{
        aeroway_feature::get_aeroway_geo_tile, amenity_feature::get_amenity_geo_tile,
        boundary_feature::get_boundary_geo_tile, building_feature::get_building_geo_tile,
        landuse_feature::get_landuse_geo_tile, leisure_feature::get_leisure_geo_tile,
        natural_feature::get_natural_geo_tile, GeoTile, GeoTileProperties, GeoTilesDataStructure,
        Geometry, TILE_SCALE,
    },
    operations::line_string_operations::line_string_to_i32,
};
use geo::algorithm::{bounding_rect::BoundingRect, contains::Contains};
use geo_types as gt;
use log::warn;
use std::rc::Rc;

// This function requires the f64 version of polygon because the contains() method does not work with integers.
pub fn draw_polygon(
    poly: &gt::Polygon<f64>,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
    landuse: bool,
    leisure: bool,
    amenity: bool,
    boundary: bool,
) {
    // Hide obnoxious feature types that are mostly redundant.
    if !landuse {
        if let GeoTile::Landuse { .. } = *geo_tile {
            return;
        };
    }
    if !leisure {
        if let GeoTile::Leisure { .. } = *geo_tile {
            return;
        };
    }
    if !amenity {
        if let GeoTile::Amenity { .. } = *geo_tile {
            return;
        };
    }
    if !boundary {
        if let GeoTile::Boundary { .. } = *geo_tile {
            return;
        };
    }
    // Establish coordinate system first.
    let bounding_rect = poly.bounding_rect().unwrap();
    let min_x = (bounding_rect.min().x * TILE_SCALE) as i32;
    let max_x = (bounding_rect.max().x * TILE_SCALE) as i32;
    let min_y = (bounding_rect.min().y * TILE_SCALE) as i32;
    let max_y = (bounding_rect.max().y * TILE_SCALE) as i32;
    // Iterate over the entire bounding rect
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            // Contains method only works with f64 apparently.
            if poly.contains(&gt::Point::new(
                (x as f64) / TILE_SCALE,
                (y as f64) / TILE_SCALE,
            )) {
                data_structure.insert(gt::Coordinate { x, y }, geo_tile.clone());
            }
        }
    }
}

pub fn polygon_feature_to_geo_tile(
    properties: &GeoTileProperties,
    polygon: gt::Polygon<f64>,
) -> GeoTile {
    let polygon = Geometry::Polygon(polygon);
    if properties.contains_key("building") {
        get_building_geo_tile(properties, polygon, false)
    } else if properties.contains_key("natural") {
        get_natural_geo_tile(properties, polygon)
    } else if properties.contains_key("boundary") {
        get_boundary_geo_tile(properties, polygon)
    } else if properties.contains_key("aeroway") {
        get_aeroway_geo_tile(properties, polygon)
    } else if properties.contains_key("leisure") {
        get_leisure_geo_tile(properties, polygon)
    } else if properties.contains_key("landuse") {
        get_landuse_geo_tile(properties, polygon)
    } else if properties.contains_key("amenity") {
        get_amenity_geo_tile(properties, polygon)
    } else if properties.contains_key("building:part") {
        get_building_geo_tile(properties, polygon, true)
    } else {
        warn!("Unclassified polygon geo tile found: {:?}", properties);
        let osm_id = properties["id"].to_string();
        GeoTile::Unclassified {
            geometry: polygon,
            osm_id,
        }
    }
}

pub fn polygon_to_i32(polygon: &gt::Polygon<f64>) -> gt::Polygon<i32> {
    let exterior_points = line_string_to_i32(polygon.exterior());
    let mut interiors_points = Vec::new();
    for interior in polygon.interiors() {
        interiors_points.push(line_string_to_i32(interior));
    }
    gt::Polygon::new(exterior_points, interiors_points)
}
