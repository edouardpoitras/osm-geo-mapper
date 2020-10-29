use criterion::{criterion_group, criterion_main, Criterion};

use osm_geo_mapper::operations;

fn bench_process_geojson() {
    let geojson = operations::parse_geojson_file("resources/ottawa.xml.geojson");
    operations::process_geojson(&geojson);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_geojson");
    group.sample_size(10);
    group.bench_function("process_geojson", |b| b.iter(|| bench_process_geojson()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);