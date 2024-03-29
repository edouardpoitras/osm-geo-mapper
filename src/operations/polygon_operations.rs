use crate::{
    features::{
        aerialway_feature::get_aerialway_geo_tile, craft_feature::get_craft_geo_tile,
        aeroway_feature::get_aeroway_geo_tile, amenity_feature::get_amenity_geo_tile,
        boundary_feature::get_boundary_geo_tile, building_feature::get_building_geo_tile,
        landuse_feature::get_landuse_geo_tile, leisure_feature::get_leisure_geo_tile,
        natural_feature::get_natural_geo_tile, highway_feature::get_highway_geo_tile,
        man_made_feature::get_man_made_geo_tile, place_feature::get_place_geo_tile,
        emergency_feature::get_emergency_geo_tile, public_transport_feature::get_public_transport_geo_tile,
        route_feature::get_route_geo_tile, geological_feature::get_geological_geo_tile,
        healthcare_feature::get_healthcare_geo_tile, historic_feature::get_historic_geo_tile,
        military_feature::get_military_geo_tile, office_feature::get_office_geo_tile,
        railway_feature::get_railway_geo_tile, shop_feature::get_shop_geo_tile,
        sport_feature::get_sport_geo_tile, telecom_feature::get_telecom_geo_tile,
        water_feature::get_water_geo_tile, waterway_feature::get_waterway_geo_tile,
        GeoTile, UnclassifiedType, GeoTileProperties, GeoTilesDataStructure, Geometry, geotile_sort, geotile_dedup
    },
    operations::{
        self,
        line_string_operations::line_string_to_i32,
    }
};
use geo::algorithm::bounding_rect::BoundingRect;
use geo_types as gt;
use log::warn;
use std::sync::Arc;

// Inspired and adapted from https://www.alienryderflex.com/polygon_fill/
// TODO: Need to create proper unit tests with custom Polygons.
//       This will confirm whether or not we're missing the surrounding layer of
//       polygons (may need to apply the full linestring exterior of the polygon).
// Also need to think about the interior linestrings and how we'll handle those...
pub fn draw_polygon(
    poly: &gt::Polygon<f64>,
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
) {
    // Establish coordinate system first.
    let bounding_rect = poly.bounding_rect().unwrap();
    let min_x = operations::to_tile_scale(bounding_rect.min().x) - 1;
    let max_x = operations::to_tile_scale(bounding_rect.max().x) + 1;
    let min_y = operations::to_tile_scale(bounding_rect.min().y) - 1;
    let max_y = operations::to_tile_scale(bounding_rect.max().y) + 1;

    let mut previous_poly_corner: Option<gt::Point<f64>> = None; // The trailing corner that was last checked.
    let mut first_poly_corner: gt::Point<f64> = gt::Point::new(0_f64, 0_f64);
    // Iterate through horizontal lines in the polygon.
    let mut locked_data_structure = data_structure.write().unwrap();
    for y in min_y..max_y + 1 {
        let yf64 = operations::from_tile_scale(y);
        let mut x_intersections: Vec<i32> = Vec::new();
        for poly_corner in poly.exterior().points_iter() {
            if let Some(previous_corner) = previous_poly_corner {
                if (poly_corner.y() < yf64 && previous_corner.y() >= yf64) || (previous_corner.y() < yf64 && poly_corner.y() >= yf64) {
                    // The horizontal line is between the two polygon corners (linestring passes through).
                    let x_intersection: i32 = operations::to_tile_scale(poly_corner.x() + (yf64 - poly_corner.y()) / (previous_corner.y() - poly_corner.y()) * (previous_corner.x() - poly_corner.x()));
                    x_intersections.push(x_intersection);
                }
            } else { // First iteration, keep track of the first corner.
                first_poly_corner = poly_corner;
            }
            previous_poly_corner = Some(poly_corner.clone());
        }
        // Need to do one last check from the last poly corner to the first poly corner.
        let previous_corner: gt::Point<f64> = previous_poly_corner.unwrap();
        if (first_poly_corner.y() < yf64 && previous_corner.y() >= yf64) || (previous_corner.y() < yf64 && first_poly_corner.y() >= yf64) {
            // The horizontal line is between the two polygon corners (linestring passes through).
            let x_intersection: i32 = operations::to_tile_scale(first_poly_corner.x() + (yf64 - first_poly_corner.y()) / (previous_corner.y() - first_poly_corner.y()) * (previous_corner.x() - first_poly_corner.x()));
            x_intersections.push(x_intersection);
        }
        // Sort our intersections from left to right.
        if x_intersections.len() < 2 { continue ; }
        x_intersections.sort();
        // Add all points between node pairs.
        for i in (0..x_intersections.len() - 1).step_by(2) {
            let mut corner1 = x_intersections[i];
            let mut corner2 = x_intersections[i + 1];
            if corner1 >= max_x { break; } // Not sure how this could ever happen...
            if corner2 > min_x { // Not sure why this is necessary...
                if corner1 < min_x { corner1 = min_x; }
                if corner2 > max_x { corner2 = max_x; }
                // We have our two corners that need geotiles in-between.
                for x in corner1..corner2 { // Do we need to use (corner2 + 1) here?
                    let vec = locked_data_structure
                        .entry(gt::Coordinate { x, y })
                        .or_insert(Vec::new());
                    vec.push(geo_tile.clone());
                    // TODO: May need to revisit this for performance reasons. Maybe only sort and dedup once all loading is complete?
                    vec.sort_by(geotile_sort);
                    vec.dedup_by(geotile_dedup);
                }
            }
        }
    }
}

pub fn polygon_feature_to_geo_tile(
    properties: &dyn GeoTileProperties,
    polygon: gt::Polygon<f64>,
) -> GeoTile {
    let polygon = Geometry::Polygon(polygon);
    if properties.has("building") {
        get_building_geo_tile(properties, polygon, properties.fetch("building").unwrap())
    } else if properties.has("natural") {
        get_natural_geo_tile(properties, polygon)
    } else if properties.has("boundary") {
        get_boundary_geo_tile(properties, polygon)
    } else if properties.has("craft") {
        get_craft_geo_tile(properties, polygon)
    } else if properties.has("aeroway") {
        get_aeroway_geo_tile(properties, polygon)
    } else if properties.has("aerialway") {
        get_aerialway_geo_tile(properties, polygon)
    } else if properties.has("leisure") {
        get_leisure_geo_tile(properties, polygon)
    } else if properties.has("emergency") {
        get_emergency_geo_tile(properties, polygon)
    } else if properties.has("landuse") {
        get_landuse_geo_tile(properties, polygon, false)
    } else if properties.has("amenity") {
        get_amenity_geo_tile(properties, polygon)
    } else if properties.has("highway") {
        get_highway_geo_tile(properties, polygon, false)
    } else if properties.has("healthcare") {
        get_healthcare_geo_tile(properties, polygon)
    } else if properties.has("historic") {
        get_historic_geo_tile(properties, polygon)
    } else if properties.has("man_made") {
        get_man_made_geo_tile(properties, polygon)
    } else if properties.has("military") {
        get_military_geo_tile(properties, polygon)
    } else if properties.has("office") {
        get_office_geo_tile(properties, polygon)
    } else if properties.has("place") {
        get_place_geo_tile(properties, polygon)
    } else if properties.has("railway") {
        get_railway_geo_tile(properties, polygon)
    } else if properties.has("route") {
        get_route_geo_tile(properties, polygon, None)
    } else if properties.has("shop") {
        get_shop_geo_tile(properties, polygon)
    } else if properties.has("sport") {
        get_sport_geo_tile(properties, polygon)
    } else if properties.has("telecom") {
        get_telecom_geo_tile(properties, polygon)
    } else if properties.has("water") {
        get_water_geo_tile(properties, polygon)
    } else if properties.has("waterway") {
        get_waterway_geo_tile(properties, polygon)
    } else if properties.has("geological") {
        get_geological_geo_tile(properties, polygon)
    } else if properties.has("public_transport") {
        get_public_transport_geo_tile(properties, polygon)
    // Less common corner cases.
    } else if properties.has("building:part") {
        get_building_geo_tile(properties, polygon, properties.fetch("building:part").unwrap())
    // We need to check for office before we check for addr::* because all office
    // features should have the addr::* properties, like any other building.
    //} else if properties.has("office") {
        //get_office_geo_tile(properties, polygon)
    } else if properties.has("addr:housenumber") {
        get_building_geo_tile(properties, polygon, "yes")
    } else if properties.has("landcover") {
        get_landuse_geo_tile(properties, polygon, true)
    } else if properties.has("piste:type") {
        get_route_geo_tile(properties, polygon, Some("piste:type"))
    } else {
        warn!("Unclassified polygon geo tile found: {}", properties.print_debug());
        let osm_id = properties.fetch("id").unwrap_or_default().to_string();
        GeoTile::Unclassified {
            unclassified_type: UnclassifiedType::Unclassified,
            address: None,
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
