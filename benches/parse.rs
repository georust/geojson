#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};
use geojson;
extern crate test;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse (countries.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson>());
        });
    });

    c.bench_function("parse (geometry_collection.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/geometry_collection.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson>());
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
