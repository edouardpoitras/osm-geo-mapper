use std::{
    collections::BTreeMap,
    fs::File,
};

use geo::prelude::ConvexHull;
use geo_types::MultiPoint;
use osmpbfreader::{
    objects::{ OsmId, OsmObj, Relation, RelationId, Way },
    OsmPbfReader,
};

pub trait HasCoordinates {
    fn get_coordinates(&self, objs: &BTreeMap<OsmId, OsmObj>) -> Vec<(f64, f64)>;
}

pub trait HasCycleCoordinates {
    fn get_coordinates(
        &self,
        objs: &BTreeMap<OsmId, OsmObj>,
        visited: &mut Vec<RelationId>,
    ) -> Vec<(f64, f64)>;
}

impl HasCoordinates for Way {
    fn get_coordinates(&self, objs: &BTreeMap<OsmId, OsmObj>) -> Vec<(f64, f64)> {
        self.nodes
            .iter()
            .filter_map(|&id| {
                let obj = objs.get(&id.into())?;
                let node = obj.node()?;
                Some((node.lon(), node.lat()))
            })
            .collect()
    }
}

impl HasCycleCoordinates for Relation {
    fn get_coordinates(
        &self,
        objs: &BTreeMap<OsmId, OsmObj>,
        visited: &mut Vec<RelationId>,
    ) -> Vec<(f64, f64)> {
        if visited.contains(&self.id) {
            return vec![];
        }
        visited.push(self.id);
        let coordinates: Vec<(f64, f64)> = self
            .refs
            .iter()
            .filter_map(|osm_ref| {
                let obj = objs.get(&osm_ref.member)?;
                let coordinates = match obj {
                    OsmObj::Node(node) => vec![(node.lon(), node.lat())],
                    OsmObj::Way(way) => way.get_coordinates(objs),
                    OsmObj::Relation(rel) => rel.get_coordinates(objs, visited),
                };
                Some(coordinates)
            })
            .flatten()
            .collect();

        let multi_points: MultiPoint<_> = coordinates.into();
        let convex_hull = multi_points.convex_hull();
        convex_hull
            .exterior()
            .points_iter()
            .map(|p| (p.lng(), p.lat()))
            .collect()
    }
}

pub fn parse_pbf_file(filename: String) -> Result<BTreeMap<OsmId, OsmObj>, osmpbfreader::Error> {
    let file = File::open(filename.as_str()).unwrap();
    let mut pbf = OsmPbfReader::new(file);
    pbf.get_objs_and_deps(|_| true)
}