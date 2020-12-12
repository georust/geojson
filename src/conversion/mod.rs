// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::geo_types;
use crate::geo_types::{
    Geometry as GtGeometry, GeometryCollection, LineString as GtLineString,
    MultiLineString as GtMultiLineString, MultiPoint as GtMultiPoint,
    MultiPolygon as GtMultiPolygon, Point as GtPoint, Polygon as GtPolygon,
};
use crate::geojson::GeoJson::{self, Feature, FeatureCollection, Geometry};
use crate::Position;

use crate::geometry::Geometry as GjGeometry;
use crate::Error as GJError;
use crate::Value;
use num_traits::Float;
use std::convert::TryInto;

#[cfg(test)]
macro_rules! assert_almost_eq {
    ($x:expr, $y:expr, $epsilon:expr) => {{
        use num_traits::Zero;
        let a = $x.abs();
        let b = $y.abs();
        let delta = (a - b).abs();

        if a.is_infinite() || a.is_nan() || b.is_infinite() || b.is_nan() {
            panic!(
                "Assertion failed: Non comparable value ({} = {}, {} = {})",
                stringify!($x),
                $x,
                stringify!($y),
                $y
            );
        } else if a.is_zero() || b.is_zero() {
            if delta > $epsilon {
                panic!(
                    "Assertion failed: ({} = {}, {} = {}, delta = {})",
                    stringify!($x),
                    $x,
                    stringify!($y),
                    $y,
                    delta / b
                );
            }
        } else {
            let normalized_delta = delta / b;
            if normalized_delta > $epsilon {
                panic!(
                    "Assertion failed: ({} = {}, {} = {}, delta = {})",
                    stringify!($x),
                    $x,
                    stringify!($y),
                    $y,
                    normalized_delta
                );
            }
        }
    }};
}

pub(crate) mod from_geo_types;
pub(crate) mod to_geo_types;

// Process top-level `GeoJSON` items, returning a geo_types::GeometryCollection or an Error
fn process_geojson<T, Pos: Position>(
    gj: &GeoJson<Pos>,
) -> Result<geo_types::GeometryCollection<T>, GJError<Pos>>
where
    T: Float,
{
    match &*gj {
        FeatureCollection(collection) => Ok(GeometryCollection(
            collection
                .features
                .iter()
                // Only pass on non-empty geometries
                .filter_map(|feature| feature.geometry.as_ref())
                .map(|geometry| process_geometry(&geometry))
                .collect::<Result<_, _>>()?,
        )),
        Feature(feature) => {
            if let Some(geometry) = &feature.geometry {
                Ok(GeometryCollection(vec![process_geometry(&geometry)?]))
            } else {
                Ok(GeometryCollection(vec![]))
            }
        }
        Geometry(geometry) => Ok(GeometryCollection(vec![process_geometry(&geometry)?])),
    }
}

// Process GeoJson Geometry objects, returning their geo_types equivalents, or an error
fn process_geometry<T, Pos: Position>(
    geometry: &GjGeometry<Pos>,
) -> Result<geo_types::Geometry<T>, GJError<Pos>>
where
    T: Float,
{
    match &geometry.value {
        Value::Point(_) => Ok(TryInto::<GtPoint<_>>::try_into(geometry.value.clone())?.into()),
        Value::MultiPoint(_) => {
            Ok(TryInto::<GtMultiPoint<_>>::try_into(geometry.value.clone())?.into())
        }
        Value::LineString(_) => {
            Ok(TryInto::<GtLineString<_>>::try_into(geometry.value.clone())?.into())
        }
        Value::MultiLineString(_) => {
            Ok(TryInto::<GtMultiLineString<_>>::try_into(geometry.value.clone())?.into())
        }
        Value::Polygon(_) => Ok(TryInto::<GtPolygon<_>>::try_into(geometry.value.clone())?.into()),
        Value::MultiPolygon(_) => {
            Ok(TryInto::<GtMultiPolygon<_>>::try_into(geometry.value.clone())?.into())
        }
        Value::GeometryCollection(gc) => {
            let gc = GtGeometry::GeometryCollection(GeometryCollection(
                gc.iter()
                    .map(|geom| process_geometry(&geom))
                    .collect::<Result<Vec<geo_types::Geometry<T>>, GJError<Pos>>>()?,
            ));
            Ok(gc)
        }
    }
}

/// A shortcut for producing `geo_types` [GeometryCollection](../geo_types/struct.GeometryCollection.html) objects
/// from arbitrary valid GeoJSON input.
///
/// This function is primarily intended for easy processing of GeoJSON `FeatureCollection`
/// objects using the `geo` crate, and sacrifices a little performance for convenience.
/// # Example
///
/// ```
/// use geo_types::GeometryCollection;
/// use geojson::{quick_collection, GeoJson};
///
/// let geojson_str = r#"
/// {
///   "type": "FeatureCollection",
///   "features": [
///     {
///       "type": "Feature",
///       "properties": {},
///       "geometry": {
///         "type": "Point",
///         "coordinates": [
///           -0.13583511114120483,
///           51.5218870403801
///         ]
///       }
///     }
///   ]
/// }
/// "#;
/// let geojson = geojson_str.parse::<GeoJson>().unwrap();
/// // Turn the GeoJSON string into a geo_types GeometryCollection
/// let mut collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
pub fn quick_collection<T, Pos: Position>(
    gj: &GeoJson<Pos>,
) -> Result<geo_types::GeometryCollection<T>, GJError<Pos>>
where
    T: Float,
{
    process_geojson(gj)
}
