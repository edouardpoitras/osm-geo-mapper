use crate::features::{BoundaryType, GeoTile, GeoTileProperties, Geometry};

pub fn get_boundary_geo_tile(props: &GeoTileProperties, geometry: Geometry) -> GeoTile {
    let boundary = props["boundary"].as_str().unwrap();
    let boundary_type = match boundary {
        "administrative" => BoundaryType::Administrative,
        "political" => BoundaryType::Political,
        //"wood" => NaturalType::Wood,
        _ => panic!("New boundary type"),
    };
    let osm_id = props["id"].to_string();
    GeoTile::Boundary {
        boundary_type,
        geometry,
        osm_id,
    }
}
