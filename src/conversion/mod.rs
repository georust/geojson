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

use geo_types::CoordFloat;

use crate::geojson::GeoJson;

use crate::Result;
use std::convert::TryFrom;

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

macro_rules! try_from_owned_value {
    ($to:ty) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
        impl<T: CoordFloat> TryFrom<geometry::Value> for $to {
            type Error = Error;

            fn try_from(value: geometry::Value) -> Result<Self> {
                (&value).try_into()
            }
        }
    };
}

pub(crate) mod from_geo_types;
pub(crate) mod to_geo_types;

/// A shortcut for producing `geo_types` [GeometryCollection](../geo_types/struct.GeometryCollection.html) objects
/// from arbitrary valid GeoJSON input.
///
/// This function is primarily intended for easy processing of GeoJSON `FeatureCollection`
/// objects using the `geo` crate, and sacrifices a little performance for convenience.
/// # Example
///
/// ```
/// use geo_types::{Geometry, GeometryCollection, Point};
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
///         "coordinates": [-1.0, 2.0]
///       }
///     }
///   ]
/// }
/// "#;
/// let geojson = geojson_str.parse::<GeoJson>().unwrap();
/// // Turn the GeoJSON string into a geo_types GeometryCollection
/// let mut collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();
/// assert_eq!(collection[0], Geometry::Point(Point::new(-1.0, 2.0)))
/// ```
#[deprecated(
    since = "0.24.1",
    note = "use `geo_types::GeometryCollection::try_from(&geojson)` instead"
)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
pub fn quick_collection<T>(gj: &GeoJson) -> Result<geo_types::GeometryCollection<T>>
where
    T: CoordFloat,
{
    geo_types::GeometryCollection::try_from(gj)
}
