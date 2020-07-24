#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};
use geojson;
extern crate test;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse Pos=Vec<f64> (countries.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson<Vec<f64>>>());
        });
    });

    c.bench_function("parse Pos=(f64, f64) (countries.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson<(f64, f64)>>());
        });
    });

    c.bench_function("parse Pos=Vec<f64> (geometry_collection.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/geometry_collection.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson<Vec<f64>>>());
        });
    });

    c.bench_function("parse Pos=(f64, f64) (geometry_collection.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/geometry_collection.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson<(f64, f64)>>());
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
