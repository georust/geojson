# Changes

## 0.9.0

* Don't publicize `assert_almost_eq` macro
* Bump rust-geo: 0.4 → 0.6
* Use docs.rs for documentation links

## 0.8.0

* [Remove `geojson::Crs`](https://github.com/georust/rust-geojson/pull/71)
* [Support `foreign_members`](https://github.com/georust/rust-geojson/pull/70)

## 0.7.1

* [Add missing reference to GeometryCollection](https://github.com/georust/rust-geojson/pull/68)

## 0.7.0

* [Upgrade to serde 1.0](https://github.com/georust/rust-geojson/pull/64)

## 0.6.0

* [Upgrade rust-geo dep, use num_traits instead of num](https://github.com/georust/rust-geojson/pull/62)

## 0.5.0

* [Upgrade to serde 0.9, remove rustc-serialize support, make geo-interop feature mandatory,](https://github.com/georust/rust-geojson/pull/60)

## 0.4.3

* [Ability to convert a structure from rust-geojson to rust-geo](https://github.com/georust/rust-geojson/pull/56)

## 0.4.2

* [Ability to convert a structure from rust-geo to rust-geojson](https://github.com/georust/rust-geojson/issues/51)

## 0.4.1

* [Derive `Eq` and `PartialEq` for `Error`.](https://github.com/georust/rust-geojson/issues/51)

## 0.4.0

* [Implement `Display` instead of `ToString` for `GeoJson`.](https://github.com/georust/rust-geojson/pull/46)
* [Upgrade Serde from 0.7 to 0.8](https://github.com/georust/rust-geojson/pull/48)
* [Add a few `convert::From` impls for `GeoJson`.](https://github.com/georust/rust-geojson/pull/45)

## 0.3.0

* [Permit `geometry` field on `feature` objects to be `null`](https://github.com/georust/rust-geojson/issues/42)
