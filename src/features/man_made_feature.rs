use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, ManMadeType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_man_made_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let man_made= props["man_made"].as_str().unwrap();
    let man_made_type = match man_made {
        "adit" => ManMadeType::Adit,
        "beacon" => ManMadeType::Beacon,
        "breakwater" => ManMadeType::Breakwater,
        "bridge" => ManMadeType::Bridge,
        "bunker_silo" => ManMadeType::BunkerSilo,
        "carpet_hanger" => ManMadeType::CarpetHanger,
        "chimney" => ManMadeType::Chimney,
        "communications_tower" => ManMadeType::CommunicationsTower,
        "crane" => ManMadeType::Crane,
        "cross" => ManMadeType::Cross,
        "cutline" => ManMadeType::Cutline,
        "clearcut" => ManMadeType::Clearcut,
        "dovecote" => ManMadeType::Dovecote,
        "dyke" => ManMadeType::Dyke,
        "embankment" => ManMadeType::Embankment,
        "flagpole" => ManMadeType::Flagpole,
        "gasometer" => ManMadeType::Gasometer,
        "goods_conveyor" => ManMadeType::GoodsConveyor,
        "groyne" => ManMadeType::Groyne,
        "kiln" => ManMadeType::Kiln,
        "lighthouse" => ManMadeType::Lighthouse,
        "mast" => ManMadeType::Mast,
        "mineshaft" => ManMadeType::Mineshaft,
        "monitoring_station" => ManMadeType::MonitoringStation,
        "obelisk" => ManMadeType::Obelisk,
        "observatory" => ManMadeType::Observatory,
        "offshore_platform" => ManMadeType::OffshorePlatform,
        "petroleum_well" => ManMadeType::PetroleumWell,
        "pier" => ManMadeType::Pier,
        "pipeline" => ManMadeType::Pipeline,
        "pumping_station" => ManMadeType::PumpingStation,
        "reservoir_covered" => ManMadeType::ReservoirCovered,
        "silo" => ManMadeType::Silo,
        "snow_fence" => ManMadeType::SnowFence,
        "snow_net" => ManMadeType::SnowNet,
        "storage_tank" => ManMadeType::StorageTank,
        "street_cabinet" => ManMadeType::StreetCabinet,
        "surveillance" => ManMadeType::Surveillance,
        "survey_point" => ManMadeType::SurveyPoint,
        "telescope" => ManMadeType::Telescope,
        "tower" => ManMadeType::Tower,
        "wastewater_plant" => ManMadeType::WastewaterPlant,
        "watermill" => ManMadeType::Watermill,
        "water_tower" => ManMadeType::WaterTower,
        "water_well" => ManMadeType::WaterWell,
        "water_tap" => ManMadeType::WaterTap,
        "water_works" => ManMadeType::WaterWorks,
        "wildlife_crossing" => ManMadeType::WildlifeCrossing,
        "windmill" => ManMadeType::Windmill,
        "works" => ManMadeType::Works,
        _ => {
            warn!("Unclassified man made type {}: {:?}", man_made, props);
            ManMadeType::Unclassified
        }
    };
    let address = address_from_properties(props);
    let access = property_to_option_string(props, "access");
    let bridge = property_to_option_string(props, "bridge");
    let capacity = property_to_option_string(props, "capacity");
    let color = property_to_option_string(props, "color");
    let content = property_to_option_string(props, "content");
    let country = property_to_option_string(props, "country");
    let covered = property_to_option_string(props, "covered");
    let cutline = property_to_option_string(props, "cutline");
    let depth = property_to_option_string(props, "depth");
    let direction = property_to_option_string(props, "direction");
    let display = property_to_option_string(props, "display");
    let disused = property_to_option_string(props, "disused");
    let drinking_water = property_to_option_string(props, "drinking_water");
    let elevation = property_to_option_string(props, "ele");
    let floating = property_to_option_string(props, "floating");
    let height = property_to_option_string(props, "height");
    let headframe = property_to_option_string(props, "headframe");
    let inscription = property_to_option_string(props, "inscription");
    let layer = property_to_option_string(props, "layer");
    let landuse = property_to_option_string(props, "landuse");
    let length = property_to_option_string(props, "length");
    let location = property_to_option_string(props, "location");
    let material = property_to_option_string(props, "material");
    let mine = property_to_option_string(props, "mine");
    let mineshaft_type = property_to_option_string(props, "mineshaft_type");
    let monitoring = property_to_option_string(props, "monitoring");
    let mooring = property_to_option_string(props, "mooring");
    let name = property_to_option_string(props, "name");
    let operator = property_to_option_string(props, "operator");
    let osm_id = props["id"].to_string();
    let oven = property_to_option_string(props, "oven");
    let power = property_to_option_string(props, "power");
    let product = property_to_option_string(props, "product");
    let pump = property_to_option_string(props, "pump");
    let pumping_station = property_to_option_string(props, "pumping_station");
    let resource = property_to_option_string(props, "resource");
    let species = property_to_option_string(props, "species");
    let start_date = property_to_option_string(props, "start_date");
    let street_cabinet = property_to_option_string(props, "street_cabinet");
    let submerged = property_to_option_string(props, "submerged");
    let substance = property_to_option_string(props, "substance");
    let support = property_to_option_string(props, "support");
    let surveillance = property_to_option_string(props, "surveillance");
    let survey_point = property_to_option_string(props, "survey_point");
    let tidal = property_to_option_string(props, "tidal");
    let tourism = property_to_option_string(props, "tourism");
    let tunnel = property_to_option_string(props, "tunnel");
    let width = property_to_option_string(props, "width");
    GeoTile::ManMade {
        address,
        access,
        bridge,
        capacity,
        color,
        content,
        country,
        covered,
        cutline,
        depth,
        direction,
        display,
        disused,
        drinking_water,
        elevation,
        floating,
        geometry,
        height,
        headframe,
        inscription,
        layer,
        landuse,
        length,
        location,
        man_made_type,
        material,
        mine,
        mineshaft_type,
        monitoring,
        mooring,
        name,
        operator,
        osm_id,
        oven,
        power,
        product,
        pump,
        pumping_station,
        resource,
        species,
        start_date,
        street_cabinet,
        submerged,
        substance,
        support,
        surveillance,
        survey_point,
        tidal,
        tourism,
        tunnel,
        width,
    }
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
