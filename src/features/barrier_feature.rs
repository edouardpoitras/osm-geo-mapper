use crate::{
    features::{BarrierType, GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_barrier_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let barrier_type_str = props.fetch("barrier").unwrap();
    let barrier_type = extract_type_from_string!(barrier_type_str<props> => BarrierType [Block, Bollard, BorderControl, BumpGate, BusTrap, CableBarrier, CattleGrid, Chain, CityWall, CycleBarrier, Debris, Ditch, Entrance, Fence, FullHeightTurnstile, Gate, GuardRail, HampshireGate, Handrail, Hedge, HeightRestrictor, HorseStile, JerseyBarrier, Kerb, KissingGate, LiftGate, Log, MotorcycleBarrier, RetainingWall, Rope, SallyPort, Spikes, Stile, SumpBuster, SwingGate, TollBooth, Turnstile, Unclassified, Wall]);
    geotile_from_properties!(geometry<props> => Barrier<barrier_type> [access, bicycle, fee, foot, two_sided, handrail, height, highway, historic, intermittent, lanes, locked, maxheight, maxwidth, motor_vehicle, operator, wheelchair, width]);
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
