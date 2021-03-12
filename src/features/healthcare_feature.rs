use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, HealthcareType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

// Driveways are treated specially because some cases only provide the "service" key without the "healthcare" key.
pub fn get_healthcare_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let healthcare_type_str = props["healthcare"].as_str().unwrap();
    let healthcare_type = extract_type_from_string!(healthcare_type_str<props> => HealthcareType [Alternative, Audiologist, BirthingCenter, BloodBank, BloodDonation, Counselling, Dialysis, Hospice, Laboratory, Midwife, Nurse, OccupationalTherapist, Optometrist, Physiotherapist, Podiatrist, Psychotherapist, Rehabilitation, SampleCollection, SpeechTherapist, VaccinationCentre, Unclassified]);
    geotile_from_properties!(geometry<props> => Healthcare<healthcare_type> [name, opening_hours, operator, phone, vaccination, website, wheelchair]);
}

pub fn draw_healthcare_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _healthcare_type: HealthcareType,
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
