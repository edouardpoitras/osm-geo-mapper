use crate::{
    features::{BoundaryType, GeoTile, GeoTileProperties, Geometry},
    operations::property_to_option_string,
};
use log::warn;

pub fn get_boundary_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let boundary = props["boundary"].as_str().unwrap();
    let boundary_type = match boundary {
        "aboriginal_lands" => BoundaryType::AboriginalLands,
        "administrative" => BoundaryType::Administrative,
        "maritime" => BoundaryType::Maritime,
        "marker" => BoundaryType::Marker,
        "national_park" => BoundaryType::NationalPark,
        "political" => BoundaryType::Political,
        "postal_code" => BoundaryType::PostalCode,
        "protected_area" => BoundaryType::ProtectedArea,
        "user_defined" => BoundaryType::UserDefined,
        _ => {
            warn!("Unclassified boundary type {}: {:?}", boundary, props);
            BoundaryType::Unclassified
        }
    };
    let admin_level = property_to_option_string(props, "admin_level");
    let area = property_to_option_string(props, "area");
    let border_type = property_to_option_string(props, "border_type");
    let description = property_to_option_string(props, "description");
    let format = property_to_option_string(props, "format");
    let inscription = property_to_option_string(props, "inscription");
    let material = property_to_option_string(props, "material");
    let name = property_to_option_string(props, "name");
    let osm_id = props["id"].to_string();
    let political_division = property_to_option_string(props, "political_division");
    let population = property_to_option_string(props, "population");
    let postal_code = property_to_option_string(props, "postal_code");
    let protect_class = property_to_option_string(props, "protect_class");
    let protection_title = property_to_option_string(props, "protection_title");
    GeoTile::Boundary {
        admin_level,
        area,
        border_type,
        boundary_type,
        description,
        format,
        geometry,
        inscription,
        material,
        name,
        osm_id,
        political_division,
        population,
        postal_code,
        protect_class,
        protection_title,
    }
}
