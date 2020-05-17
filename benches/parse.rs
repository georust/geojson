#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};
use geojson;
extern crate test;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse1", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson>());
        });
    });

    c.bench_function("parse2", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _: Result<geojson::geojson_raw::GeoJson, _> = test::black_box(serde_json::from_str(&geojson_str));
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
