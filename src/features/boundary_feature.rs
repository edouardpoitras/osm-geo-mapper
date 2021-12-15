use crate::{
    features::{BoundaryType, GeoTile, GeoTileProperties, Geometry},
    operations::{address_from_properties, property_to_option_string},
};
use osm_geo_mapper_macros::{ extract_type_from_string, geotile_from_properties };
use paste::paste; // Required for the extract_type_from_string macro.
use log::warn;

pub fn get_boundary_geo_tile(props: &dyn GeoTileProperties, geometry: Geometry) -> GeoTile {
    let boundary_type_str = props.fetch("boundary").unwrap();
    let boundary_type = extract_type_from_string!(boundary_type_str<props> => BoundaryType [AboriginalLands, Administrative, Maritime, Marker, NationalPark, Political, PostalCode, ProtectedArea, UserDefined, Unclassified]);
    geotile_from_properties!(geometry<props> => Boundary<boundary_type> [name, admin_level, area, border_type, description, format, inscription, material, political_division, population, postal_code, protect_class, protection_title]);
}
