#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};

extern crate test;

fn parse_benchmark(c: &mut Criterion) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");
    let geojson = geojson_str.parse::<geojson::GeoJson<(f64, f64)>>().unwrap();

    c.bench_function("quick_collection", move |b| {
        b.iter(|| {
            let _: Result<geo_types::GeometryCollection<f64>, _> =
                test::black_box(geojson::quick_collection(&geojson));
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
