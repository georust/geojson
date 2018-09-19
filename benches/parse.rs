#![feature(test)]

#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate geojson;
extern crate test;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = test::black_box(geojson_str.parse::<geojson::GeoJson>());
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);