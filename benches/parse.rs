#![feature(test)]

extern crate geojson;
extern crate test;

#[bench]
fn bench_read(bencher: &mut test::Bencher) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");

    bencher.iter(|| {
        test::black_box(geojson_str.parse::<geojson::GeoJson>());
    });
}
