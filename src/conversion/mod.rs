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

macro_rules! try_from_owned_value {
    ($to:ty) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
        impl<T: CoordFloat> TryFrom<GeometryValue> for $to {
            type Error = Error;

            fn try_from(value: GeometryValue) -> Result<Self> {
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
/// #[allow(deprecated)]
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
/// #[allow(deprecated)]
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
