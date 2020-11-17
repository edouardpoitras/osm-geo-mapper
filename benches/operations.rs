use std::thread;
use criterion::{criterion_group, criterion_main, Criterion};

use osm_geo_mapper::operations;

fn bench_process_geojson() {
    let geojson = operations::parse_geojson_file("resources/ottawa.xml.geojson");
    operations::process_geojson(&geojson);
}

fn bench_threaded_process_geojson() {
    let mut handles = Vec::new();
    for _ in 1..10 {
        handles.push(thread::spawn(|| {
            let geojson = operations::parse_geojson_file("resources/ottawa.xml.geojson");
            operations::process_geojson(&geojson);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("process_geojson");
    group.sample_size(10);
    group.bench_function("process_geojson", |b| b.iter(|| bench_process_geojson()));
    group.bench_function("threaded_process_geojson", |b| b.iter(|| bench_threaded_process_geojson()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);