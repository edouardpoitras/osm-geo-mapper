use crate::{
    operations,
    features::{
        aeroway_feature::{draw_aeroway_line_string, get_aeroway_geo_tile},
        amenity_feature::{draw_amenity_line_string, get_amenity_geo_tile},
        barrier_feature::{draw_barrier_line_string, get_barrier_geo_tile},
        highway_feature::{draw_highway_line_string, get_highway_geo_tile},
        landuse_feature::{draw_landuse_line_string, get_landuse_geo_tile},
        leisure_feature::{draw_leisure_line_string, get_leisure_geo_tile},
        natural_feature::{draw_natural_line_string, get_natural_geo_tile},
        route_feature::{draw_route_line_string, get_route_geo_tile},
        GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, TILE_SCALE,
    }
};

use geo_types as gt;
use log::warn;
use std::rc::Rc;

pub fn draw_line_string(geo_tile: Rc<GeoTile>, data_structure: &mut GeoTilesDataStructure) {
    match (*geo_tile).clone() {
        GeoTile::Aeroway {
            aeroway_type,
            geometry,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("aeroway should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("aeroway should not be dealing with a polygon"),
            };
            draw_aeroway_line_string(geo_tile, data_structure, aeroway_type, line_string)
        }
        GeoTile::Amenity {
            amenity_type,
            geometry,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("amenity should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("amenity should not be dealing with a polygon"),
            };
            draw_amenity_line_string(geo_tile, data_structure, amenity_type, line_string)
        }
        GeoTile::Barrier {
            barrier_type,
            geometry,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("barrier should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("barrier should not be dealing with a polygon"),
            };
            draw_barrier_line_string(geo_tile, data_structure, barrier_type, line_string)
        }
        GeoTile::Building { .. } => {
            warn!("buildings should not be dealing with a line string");
        }
        GeoTile::Highway {
            highway_type,
            geometry,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("highways should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("highways should not be dealing with a polygon"),
            };
            draw_highway_line_string(geo_tile, data_structure, highway_type, line_string)
        }
        GeoTile::Landuse {
            geometry,
            landuse_type,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("landuse should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("landuse should not be dealing with a polygon"),
            };
            draw_landuse_line_string(geo_tile, data_structure, landuse_type, line_string)
        }
        GeoTile::Leisure {
            geometry,
            leisure_type,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("leisure should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("leisure should not be dealing with a polygon"),
            };
            draw_leisure_line_string(geo_tile, data_structure, leisure_type, line_string)
        }
        GeoTile::Natural {
            geometry,
            natural_type,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("natural should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("natural should not be dealing with a polygon"),
            };
            draw_natural_line_string(geo_tile, data_structure, natural_type, line_string)
        }
        GeoTile::Route {
            geometry,
            route_type,
            ..
        } => {
            let line_string = match geometry {
                Geometry::LineString(ls) => ls,
                Geometry::Point(_) => panic!("route should not be dealing with a point"),
                Geometry::Polygon(_) => panic!("route should not be dealing with a polygon"),
            };
            draw_route_line_string(geo_tile, data_structure, route_type, line_string)
        }
        GeoTile::Unclassified { .. } => {
            warn!("Trying to draw a line string for an unclassified feature: {:?}", geo_tile)
        }
        _ => warn!("Trying to draw line for a feature not yet implemented: {:?}", geo_tile),
    }
}

pub fn line_string_feature_to_geo_tile(
    properties: &GeoTileProperties,
    line_string: gt::LineString<f64>,
) -> GeoTile {
    let line_string = Geometry::LineString(line_string);
    if properties.contains_key("highway") {
        get_highway_geo_tile(properties, line_string, false)
    } else if properties.contains_key("route") {
        get_route_geo_tile(properties, line_string)
    } else if properties.contains_key("aeroway") {
        get_aeroway_geo_tile(properties, line_string)
    } else if properties.contains_key("amenity") {
        get_amenity_geo_tile(properties, line_string)
    } else if properties.contains_key("leisure") {
        get_leisure_geo_tile(properties, line_string)
    } else if properties.contains_key("landuse") {
        get_landuse_geo_tile(properties, line_string, false)
    } else if properties.contains_key("landcover") {
        get_landuse_geo_tile(properties, line_string, true)
    } else if properties.contains_key("barrier") {
        get_barrier_geo_tile(properties, line_string)
    } else if properties.contains_key("natural") {
        get_natural_geo_tile(properties, line_string)
    // Weird corner-cases.
    } else if properties.contains_key("service") && properties["service"] == "driveway" {
        // Driveways are treated as service roads.
        get_highway_geo_tile(properties, line_string, true)
    } else {
        warn!(
            "Unclassified line string feature geo tile found: {:?}",
            properties
        );
        let osm_id = properties["id"].to_string();
        GeoTile::Unclassified {
            geometry: line_string,
            osm_id,
        }
    }
}

pub fn line_string_to_i32(line_string: &gt::LineString<f64>) -> gt::LineString<i32> {
    let mut points: Vec<gt::Coordinate<i32>> = Vec::new();
    for point in line_string.points_iter() {
        points.push(gt::Coordinate {
            x: operations::to_tile_scale(point.x()),
            y: operations::to_tile_scale(point.y()),
        });
    }
    points.into()
}

pub fn draw_line(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    thickness: u8,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    if thickness < 1 {
        return;
    }
    let end_x = end.x() * TILE_SCALE;
    let end_y = end.y() * TILE_SCALE;
    let start_x = start.x() * TILE_SCALE;
    let start_y = start.y() * TILE_SCALE;
    if thickness > 1 {
        // Find which direction the line goes to reach end, then adjust thickness horizontally or vertically.
        if end_x / end_y >= 1.0 {
            // Duplicate lines vertically
            expand_line(
                start,
                end,
                thickness,
                true,
                geo_tile.clone(),
                data_structure,
            );
        } else {
            // Duplicate lines horizontally
            expand_line(
                start,
                end,
                thickness,
                false,
                geo_tile.clone(),
                data_structure,
            );
        }
    }
    // Now that thickness has been handled above, do the center line.
    let number_of_points = ((end_x - start_x).powf(2.0) + (end_y - start_y).powf(2.0))
        .sqrt()
        .floor();
    let step_x = (end_x - start_x) / number_of_points;
    let step_y = (end_y - start_y) / number_of_points;
    for i in 0..number_of_points as u32 {
        let coord = gt::Coordinate {
            x: start_x as i32 + (step_x * (i as f64)) as i32,
            y: start_y as i32 + (step_y * (i as f64)) as i32,
        };
        data_structure.insert(coord, geo_tile.clone());
    }
}

// Does NOT draw the middle line provided, only draws the expanded lines.
fn expand_line(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    thickness: u8,
    vertical: bool,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    for i in 1..thickness {
        let distance = (i as f32 / 2.0).ceil() as u8;
        // If it's an even thickness, we expand the road north or east, if it's an odd thickness we expand south or west.
        if i % 2 == 0 {
            if vertical {
                draw_line_north_of(start, end, distance, geo_tile.clone(), data_structure);
            } else {
                draw_line_east_of(start, end, distance, geo_tile.clone(), data_structure);
            }
        } else if vertical {
            draw_line_south_of(start, end, distance, geo_tile.clone(), data_structure);
        } else {
            draw_line_west_of(start, end, distance, geo_tile.clone(), data_structure);
        }
    }
}

fn draw_line_north_of(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    distance: u8,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    draw_line(
        &gt::Point::new(start.x(), start.y() - operations::from_tile_scale_u8(distance)),
        &gt::Point::new(end.x(), end.y() - operations::from_tile_scale_u8(distance)),
        1,
        geo_tile,
        data_structure,
    );
}

fn draw_line_south_of(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    distance: u8,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    draw_line(
        &gt::Point::new(start.x(), start.y() + operations::from_tile_scale_u8(distance)),
        &gt::Point::new(end.x(), end.y() + operations::from_tile_scale_u8(distance)),
        1,
        geo_tile,
        data_structure,
    );
}

fn draw_line_east_of(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    distance: u8,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    draw_line(
        &gt::Point::new(start.x() + operations::from_tile_scale_u8(distance), start.y()),
        &gt::Point::new(end.x() + operations::from_tile_scale_u8(distance), end.y()),
        1,
        geo_tile,
        data_structure,
    );
}

fn draw_line_west_of(
    start: &gt::Point<f64>,
    end: &gt::Point<f64>,
    distance: u8,
    geo_tile: Rc<GeoTile>,
    data_structure: &mut GeoTilesDataStructure,
) {
    draw_line(
        &gt::Point::new(start.x() - operations::from_tile_scale_u8(distance), start.y()),
        &gt::Point::new(end.x() - operations::from_tile_scale_u8(distance), end.y()),
        1,
        geo_tile,
        data_structure,
    );
}
