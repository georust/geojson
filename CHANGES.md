# Changes

## Unreleased

* Added conversion from `Vec<Feature>` to `GeoJson`.
* Added `GeoJson::to_string_pretty` as convenience wrappers around the same `serde_json` methods.

## 0.24.1

* Modified conversion from JSON to reject zero- and one-dimensional positions.
  * PR: <https://github.com/georust/geojson/pull/225>

## 0.24.0

* Added `geojson::{ser, de}` helpers to convert your custom struct to and from GeoJSON. 
  * For external geometry types like geo-types, use the `serialize_geometry`/`deserialize_geometry` helpers.
  * Example:
    ```
    #[derive(Serialize, Deserialize)]
    struct MyStruct {
        #[serde(serialize_with = "serialize_geometry", deserialize_with = "deserialize_geometry")]
        geometry: geo_types::Point<f64>,
        name: String,
        age: u64,
    }

    // read your input
    let my_structs: Vec<MyStruct> = geojson::de::deserialize_feature_collection(geojson_reader).unwrap();

    // do some processing
    process(&mut my_structs);

    // write back your results
    geojson::ser::to_feature_collection_string(&my_structs).unwrap();
    ```
  * PR: <https://github.com/georust/geojson/pull/199>
* Added `geojson::{FeatureReader, FeatureWriter}` to stream the reading/writing of your custom struct to and from GeoJSON, greatly reducing the memory required to process a FeatureCollection.
  * PR: <https://github.com/georust/geojson/pull/199>
  * PR: <https://github.com/georust/geojson/pull/205>
  * PR: <https://github.com/georust/geojson/pull/206>
* Added IntoIter implementation for FeatureCollection.
  * <https://github.com/georust/geojson/pull/196>
* Added `geojson::Result<T>`.
  * <https://github.com/georust/geojson/pull/198>
* Added `TryFrom<&geometry::Value>` for geo_type variants.
  * <https://github.com/georust/geojson/pull/202>
* Changed the Display string of the error produced when converting a geometry to an incompatible type - e.g. a LineString into a Point.
  * <https://github.com/georust/geojson/pull/203>
* Fix: FeatureIterator errors when reading "features" field before "type" field.
  * <https://github.com/georust/geojson/pull/200>
* BREAKING: Change the Result type of FeatureIterator from io::Result to crate::Result
  * <https://github.com/georust/geojson/pull/199>

## 0.23.0

* Enable optional geo-types integration by default.
  * <https://github.com/georust/geojson/pull/189>
* FIX: converting a single GeometryCollection Feature to geo_types
  * <https://github.com/georust/geojson/pull/194>

## 0.22.4

* Allow parsing `Feature`/`FeatureCollection` that are missing a `"properties"` key.
  * <https://github.com/georust/geojson/pull/182>
* Overhauled front page documentation.
  * <https://github.com/georust/geojson/pull/183>
* Parse `Geometry`/`Feature`/`FeatureCollection` directly from str rather than
  via `GeoJson` when you know what you're expecting.
  * <https://github.com/georust/geojson/pull/188>
* `Feature` now derives `Default`
  * <https://github.com/georust/geojson/pull/190>
* Reexport `JsonObject` and `JsonValue` from `serde_json`.
  * <https://github.com/georust/geojson/pull/191>

## 0.22.3

* Added `FromIterator<Feature>` impl for `FeatureCollection`
  * <https://github.com/georust/geojson/pull/171>
* Added `'FeatureIterator` streaming feature collection deserializer
  * <https://github.com/georust/geojson/pull/181>

## 0.22.2

* Added convenience methods to convert from geo_types::Geometry directly to GeoJson
  * <https://github.com/georust/geojson/pull/164>

## 0.22.1

* Added convenience methods to convert from Geometry and Value to Feature
  * <https://github.com/georust/geojson/pull/162>

## 0.22.0

* Update `geo-types` to 0.7.0

## 0.21.0

* `Display` implementation of `geojson::Value` prints` the GeoJSON string
  * <https://github.com/georust/geojson/issues/149>

## 0.20.0
* Switch to thiserror
* Add more granular errors
  * `GeoJsonUnknownType` has been split into `NotAFeature` and `EmptyType`
* Add additional Value context to errors where possible
* Add conversions from Geo-Types Line, Triangle, Rect and GeometryCollection

## 0.19.0

* Update `geo-types` to 0.6.0
* Remove unnecessary allocations when parsing `GeometryCollection`
  * <https://github.com/georust/geojson/pull/128>

## 0.18.0
* Update `geo-types` to 0.5.0
* Update docs
* Add quick_collection function
  * <https://github.com/georust/geojson/pull/122>
* Add TryFrom impls for JsonObject and JsonValue
  * <https://github.com/georust/geojson/pull/120>
* Add from_json_value! macro
  * <https://github.com/georust/geojson/pull/119>

## 0.17.0

* Add `TryFrom` impls for `JsonObject` and `JsonValue`
  * <https://github.com/georust/geojson/pull/120>
* Add `from_json_value` for `GeoJson` enum
  * <https://github.com/georust/geojson/pull/119>

## 0.16.0

* Switch to Rust 2018 Edition
  * <https://github.com/georust/geojson/pull/111>
* Switch to `std::TryFrom` trait from custom in-crate `TryFrom` trait
  * <https://github.com/georust/geojson/pull/111>
* Implement `Display` for `Feature`, `Geometry`, and `FeatureCollection`
  * <https://github.com/georust/geojson/pull/113>
  * <https://github.com/georust/geojson/pull/114>
* Make the `geo-types` conversion functionality opt-in
  * <https://github.com/georust/geojson/pull/115>

## 0.15.0

* Bump geo-types to 0.4.0.
  * <https://github.com/georust/geojson/commit/c1681347b4bc49c9085ac3f86fe0488849063913>

## 0.14.0

* Bump geo-types to 0.3.0.
  * <https://github.com/georust/geojson/pull/109>

## 0.13.0

* Feature::id should either be a string or number; introduce `feature::Id`
  * <https://github.com/georust/geojson/pull/107>
* Fix broken GeoJSON links in docs
  * <https://github.com/georust/geojson/pull/105>
* Improve error message for mismatched type
  * <https://github.com/georust/geojson/commit/1c5d174>
* Performance improvements

## 0.12.0

* Bump geo-types to 0.2.0.
  * <https://github.com/georust/geojson/pull/100>

## 0.11.1

* Don't inject empty interior rings when converting to geo Polygons
  * <https://github.com/georust/geojson/pull/99>

## 0.11.0

* Switch 'geo' dependency to 'geo-types'
  * <https://github.com/georust/geojson/pull/93>

## 0.10.0

* Deserialize Optimizations
  * <https://github.com/georust/geojson/pull/82>
* Expand docs with parsing examples and corner cases, and enable conversion docs
  * <https://github.com/georust/geojson/pull/85>
* Update GeoJSON spec links to point to published standard
  * <https://github.com/georust/geojson/pull/87>
* Bump geo and num-traits crates.
  * <https://github.com/georust/geojson/pull/89>
* Bump geo dependency: 0.7 -> 0.8.
  * <https://github.com/georust/geojson/pull/91>

## 0.9.0

* Don't publicize `assert_almost_eq` macro
* Bump geo: 0.4 → 0.6
* Use docs.rs for documentation links

## 0.8.0

* [Remove `geojson::Crs`](https://github.com/georust/geojson/pull/71)
* [Support `foreign_members`](https://github.com/georust/geojson/pull/70)

## 0.7.1

* [Add missing reference to GeometryCollection](https://github.com/georust/geojson/pull/68)

## 0.7.0

* [Upgrade to serde 1.0](https://github.com/georust/geojson/pull/64)

## 0.6.0

* [Upgrade rust-geo dep, use num_traits instead of num](https://github.com/georust/geojson/pull/62)

## 0.5.0

* [Upgrade to serde 0.9, remove rustc-serialize support, make geo-interop feature mandatory,](https://github.com/georust/geojson/pull/60)

## 0.4.3

* [Ability to convert a structure from rust-geojson to rust-geo](https://github.com/georust/geojson/pull/56)

## 0.4.2

* [Ability to convert a structure from rust-geo to rust-geojson](https://github.com/georust/geojson/issues/51)

## 0.4.1

* [Derive `Eq` and `PartialEq` for `Error`.](https://github.com/georust/geojson/issues/51)

## 0.4.0

* [Implement `Display` instead of `ToString` for `GeoJson`.](https://github.com/georust/geojson/pull/46)
* [Upgrade Serde from 0.7 to 0.8](https://github.com/georust/geojson/pull/48)
* [Add a few `convert::From` impls for `GeoJson`.](https://github.com/georust/geojson/pull/45)

## 0.3.0

* [Permit `geometry` field on `feature` objects to be `null`](https://github.com/georust/geojson/issues/42)
