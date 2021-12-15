use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, ManMadeType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_man_made_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let man_made_type_str= props.fetch("man_made").unwrap();
    let man_made_type = extract_type_from_string!(man_made_type_str<props> => ManMadeType [Adit, Beacon, Breakwater, Bridge, BunkerSilo, CarpetHanger, Chimney, CommunicationsTower, Crane, Cross, Cutline, Clearcut, Dovecote, Dyke, Embankment, Flagpole, Gasometer, GoodsConveyor, Groyne, Kiln, Lighthouse, Mast, Mineshaft, MonitoringStation, Obelisk, Observatory, OffshorePlatform, PetroleumWell, Pier, Pipeline, PumpingStation, ReservoirCovered, Silo, SnowFence, SnowNet, StorageTank, StreetCabinet, Surveillance, SurveyPoint, Telescope, Tower, Unclassified, WastewaterPlant, Watermill, WaterTower, WaterWell, WaterTap, WaterWorks, WildlifeCrossing, Windmill, Works]);
    geotile_from_properties!(geometry<props> => ManMade<man_made_type> [name, access, bridge, capacity, color, content, country, covered, cutline, depth, direction, display, disused, drinking_water, ele, floating, height, headframe, inscription, layer, landuse, length, location, material, mine, mineshaft_type, monitoring, mooring, operator, oven, power, product, pump, pumping_station, resource, species, start_date, street_cabinet, submerged, substance, support, surveillance, survey_point, tidal, tourism, tunnel, width]);
}

pub fn draw_man_made_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _man_made_type: ManMadeType,
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
