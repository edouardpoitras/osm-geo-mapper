use crate::{
    features::{BarrierType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::extract_type_from_string;
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_barrier_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let barrier_type_str = props["barrier"].as_str().unwrap();
    let barrier_type = extract_type_from_string!(barrier_type_str<props> => BarrierType [Block, Bollard, BorderControl, BumpGate, BusTrap, CableBarrier, CattleGrid, Chain, CityWall, CycleBarrier, Debris, Ditch, Entrance, Fence, FullHeightTurnstile, Gate, GuardRail, HampshireGate, Handrail, Hedge, HeightRestrictor, HorseStile, JerseyBarrier, Kerb, KissingGate, LiftGate, Log, MotorcycleBarrier, RetainingWall, Rope, SallyPort, Spikes, Stile, SumpBuster, SwingGate, TollBooth, Turnstile, Unclassified, Wall]);
    let address = address_from_properties(props);
    let access = property_to_option_string(props, "access");
    let bicycle = property_to_option_string(props, "bicycle");
    let fee = property_to_option_string(props, "fee");
    let foot = property_to_option_string(props, "foot");
    let two_sided = property_to_option_string(props, "two_sided");
    let handrail = property_to_option_string(props, "handrail");
    let height = property_to_option_string(props, "height");
    let highway = property_to_option_string(props, "highway");
    let historic = property_to_option_string(props, "historic");
    let intermittent = property_to_option_string(props, "intermittent");
    let lanes = property_to_option_string(props, "lanes");
    let locked = property_to_option_string(props, "locked");
    let maxheight = property_to_option_string(props, "maxheight");
    let maxwidth = property_to_option_string(props, "maxwidth");
    let motor_vehicle = property_to_option_string(props, "motor_vehicle");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let wheelchair = property_to_option_string(props, "wheelchair");
    let width = property_to_option_string(props, "width");
    GeoTile::Barrier {
        address,
        access,
        barrier_type,
        bicycle,
        fee,
        foot,
        geometry,
        two_sided,
        handrail,
        height,
        highway,
        historic,
        intermittent,
        lanes,
        locked,
        maxheight,
        maxwidth,
        motor_vehicle,
        operator,
        osm_id,
        wheelchair,
        width,
    }
}

pub fn draw_barrier_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _barrier_type: BarrierType,
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
