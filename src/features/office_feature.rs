use crate::{
    features::{GeoTile, GeoTileProperties, GeoTilesDataStructure, Geometry, OfficeType},
    operations::{line_string_operations::draw_line, address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use geo_types as gt;
use log::warn;
use std::sync::Arc;

pub fn get_office_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let office_type_str = props["office"].as_str().unwrap();
    let office_type = extract_type_from_string!(office_type_str<props> => OfficeType [Accountant, AdvertisingAgency, Architect, Association, Charity, Company, Consulting, Courier, Coworking, Diplomatic, EducationalInstitution, EmploymentAgency, EnergySupplier, Engineer, EstateAgent, Financial, FinancialAdvisor, Forestry, Foundation, Government, Guide, Insurance, It, Lawyer, Logistics, MovingCompany, Newspaper, Ngo, Notary, PoliticalParty, PropertyManagement, Quango, Religion, Research, Surveyor, TaxAdvisor, Telecommunication, Unclassified, Visa, WaterUtility]);
    geotile_from_properties!(geometry<props> => Office<office_type> [admin_level, advertising, association, brand, cargo, club, consulate, consulting, country, denomination, department, diplomatic, email, embassy, faculty, fax, fee, function, government, hgv, industrial, insurance, internet_access, liaison, name, opening_hours, operator, owner, payment, phone, religion, research, social_facility, target, website, wheelchair]);
}

pub fn draw_office_line_string(
    geo_tile: Arc<GeoTile>,
    data_structure: GeoTilesDataStructure,
    _office_type: OfficeType,
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
