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
use crate::geo::algorithm::orient::{Orient, Direction};
use crate::geometry;
use crate::Error as GJError;
use crate::{LineStringType, PointType, PolygonType};
use num_traits::Float;
use std::convert::From;
use std::convert::TryInto;

fn create_point_type<T>(point: &geo_types::Point<T>) -> PointType
where
    T: Float,
{
    let x: f64 = point.x().to_f64().unwrap();
    let y: f64 = point.y().to_f64().unwrap();

    vec![x, y]
}

fn create_line_string_type<T>(line_string: &geo_types::LineString<T>) -> LineStringType
where
    T: Float,
{
    line_string
        .points_iter()
        .map(|point| create_point_type(&point))
        .collect()
}

fn create_multi_line_string_type<T>(
    multi_line_string: &geo_types::MultiLineString<T>,
) -> Vec<LineStringType>
where
    T: Float,
{
    multi_line_string
        .0
        .iter()
        .map(|line_string| create_line_string_type(line_string))
        .collect()
}

fn create_polygon_type<T>(polygon: &geo_types::Polygon<T>) -> PolygonType
where
    T: Float,
{
    let mut coords = vec![polygon
        .exterior()
        .points_iter()
        .map(|point| create_point_type(&point))
        .collect()];

    coords.extend(
        polygon
            .interiors()
            .iter()
            .map(|line_string| create_line_string_type(&line_string)),
    );

    coords
}

fn create_multi_polygon_type<T>(multi_polygon: &geo_types::MultiPolygon<T>) -> Vec<PolygonType>
where
    T: Float,
{
    multi_polygon
        .0
        .iter()
        .map(|polygon| create_polygon_type(&polygon))
        .collect()
}

fn create_geo_coordinate<T>(point_type: &PointType) -> geo_types::Coordinate<T>
where
    T: Float,
{
    geo_types::Coordinate {
        x: T::from(point_type[0]).unwrap(),
        y: T::from(point_type[1]).unwrap(),
    }
}

fn create_geo_point<T>(point_type: &PointType) -> geo_types::Point<T>
where
    T: Float,
{
    geo_types::Point::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
    )
}

fn create_geo_line_string<T>(line_type: &LineStringType) -> geo_types::LineString<T>
where
    T: Float,
{
    geo_types::LineString(
        line_type
            .iter()
            .map(|point_type| create_geo_coordinate(point_type))
            .collect(),
    )
}

fn create_geo_multi_line_string<T>(
    multi_line_type: &[LineStringType],
) -> geo_types::MultiLineString<T>
where
    T: Float,
{
    geo_types::MultiLineString(
        multi_line_type
            .iter()
            .map(|point_type| create_geo_line_string(&point_type))
            .collect(),
    )
}

fn create_geo_polygon<T>(polygon_type: &PolygonType) -> geo_types::Polygon<T>
where
    T: Float,
{
    let exterior = polygon_type
        .get(0)
        .map(|e| create_geo_line_string(e))
        .unwrap_or_else(|| create_geo_line_string(&vec![]));

    let interiors = if polygon_type.len() < 2 {
        vec![]
    } else {
        polygon_type[1..]
            .iter()
            .map(|line_string_type| create_geo_line_string(line_string_type))
            .collect()
    };

    geo_types::Polygon::new(exterior, interiors).orient(Direction::Default)
}

fn create_geo_multi_polygon<T>(multi_polygon_type: &[PolygonType]) -> geo_types::MultiPolygon<T>
where
    T: Float,
{
    geo_types::MultiPolygon(
        multi_polygon_type
            .iter()
            .map(|polygon_type| create_geo_polygon(&polygon_type))
            .collect(),
    )
}

impl<T> TryInto<geo_types::Point<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::Point<T>, Self::Error> {
        match self {
            geometry::Value::Point(point_type) => Ok(create_geo_point(&point_type)),
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::Point<T>> for geometry::Value
where
    T: Float,
{
    fn from(point: &geo_types::Point<T>) -> Self {
        let coords = create_point_type(point);

        geometry::Value::Point(coords)
    }
}

impl<T> TryInto<geo_types::MultiPoint<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::MultiPoint<T>, Self::Error> {
        match self {
            geometry::Value::MultiPoint(multi_point_type) => Ok(geo_types::MultiPoint(
                multi_point_type
                    .iter()
                    .map(|point_type| create_geo_point(&point_type))
                    .collect(),
            )),
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::MultiPoint<T>> for geometry::Value
where
    T: Float,
{
    fn from(multi_point: &geo_types::MultiPoint<T>) -> Self {
        let coords = multi_point
            .0
            .iter()
            .map(|point| create_point_type(point))
            .collect();

        geometry::Value::MultiPoint(coords)
    }
}

impl<T> TryInto<geo_types::LineString<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::LineString<T>, Self::Error> {
        match self {
            geometry::Value::LineString(multi_point_type) => {
                Ok(create_geo_line_string(&multi_point_type))
            }
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::LineString<T>> for geometry::Value
where
    T: Float,
{
    fn from(line_string: &geo_types::LineString<T>) -> Self {
        let coords = create_line_string_type(line_string);

        geometry::Value::LineString(coords)
    }
}

impl<T> TryInto<geo_types::MultiLineString<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::MultiLineString<T>, Self::Error> {
        match self {
            geometry::Value::MultiLineString(multi_line_string_type) => {
                Ok(create_geo_multi_line_string(&multi_line_string_type))
            }
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::MultiLineString<T>> for geometry::Value
where
    T: Float,
{
    fn from(multi_line_string: &geo_types::MultiLineString<T>) -> Self {
        let coords = create_multi_line_string_type(multi_line_string);

        geometry::Value::MultiLineString(coords)
    }
}

impl<T> TryInto<geo_types::Polygon<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::Polygon<T>, Self::Error> {
        match self {
            geometry::Value::Polygon(polygon_type) => Ok(create_geo_polygon(&polygon_type)),
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::Polygon<T>> for geometry::Value
where
    T: Float,
{
    fn from(polygon: &geo_types::Polygon<T>) -> Self {
        let coords = create_polygon_type(polygon);

        geometry::Value::Polygon(coords)
    }
}

impl<T> TryInto<geo_types::MultiPolygon<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::MultiPolygon<T>, Self::Error> {
        match self {
            geometry::Value::MultiPolygon(multi_polygon_type) => {
                Ok(create_geo_multi_polygon(&multi_polygon_type))
            }
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::MultiPolygon<T>> for geometry::Value
where
    T: Float,
{
    fn from(multi_polygon: &geo_types::MultiPolygon<T>) -> Self {
        let coords = create_multi_polygon_type(multi_polygon);

        geometry::Value::MultiPolygon(coords)
    }
}

impl<T> TryInto<geo_types::GeometryCollection<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::GeometryCollection<T>, Self::Error> {
        match self {
            geometry::Value::GeometryCollection(geometries) => {
                let geojson_geometries = geometries
                    .iter()
                    .map(|geometry| geometry.value.clone().try_into().unwrap())
                    .collect();

                Ok(geo_types::GeometryCollection(geojson_geometries))
            }
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<T> TryInto<geo_types::Geometry<T>> for geometry::Value
where
    T: Float,
{
    type Error = GJError;

    fn try_into(self) -> Result<geo_types::Geometry<T>, Self::Error> {
        match self {
            geometry::Value::Point(ref point_type) => {
                Ok(geo_types::Geometry::Point(create_geo_point(point_type)))
            }
            geometry::Value::MultiPoint(ref multi_point_type) => {
                Ok(geo_types::Geometry::MultiPoint(geo_types::MultiPoint(
                    multi_point_type
                        .iter()
                        .map(|point_type| create_geo_point(&point_type))
                        .collect(),
                )))
            }
            geometry::Value::LineString(ref line_string_type) => Ok(
                geo_types::Geometry::LineString(create_geo_line_string(line_string_type)),
            ),
            geometry::Value::MultiLineString(ref multi_line_string_type) => {
                Ok(geo_types::Geometry::MultiLineString(
                    create_geo_multi_line_string(multi_line_string_type),
                ))
            }
            geometry::Value::Polygon(ref polygon_type) => Ok(geo_types::Geometry::Polygon(
                create_geo_polygon(polygon_type),
            )),
            geometry::Value::MultiPolygon(ref multi_polygon_type) => Ok(
                geo_types::Geometry::MultiPolygon(create_geo_multi_polygon(multi_polygon_type)),
            ),
            _ => Err(GJError::GeometryUnknownType),
        }
    }
}

impl<'a, T> From<&'a geo_types::GeometryCollection<T>> for geometry::Value
where
    T: Float,
{
    fn from(geometry_collection: &geo_types::GeometryCollection<T>) -> Self {
        let coords = geometry_collection
            .0
            .iter()
            .map(|geometry| geometry::Geometry::new(geometry::Value::from(geometry)))
            .collect();

        geometry::Value::GeometryCollection(coords)
    }
}

impl<'a, T> From<&'a geo_types::Geometry<T>> for geometry::Value
where
    T: Float,
{
    fn from(geometry: &'a geo_types::Geometry<T>) -> Self {
        match *geometry {
            geo_types::Geometry::Point(ref point) => geometry::Value::from(point),
            geo_types::Geometry::MultiPoint(ref multi_point) => geometry::Value::from(multi_point),
            geo_types::Geometry::LineString(ref line_string) => geometry::Value::from(line_string),
            geo_types::Geometry::MultiLineString(ref multi_line_string) => {
                geometry::Value::from(multi_line_string)
            }
            geo_types::Geometry::Polygon(ref polygon) => geometry::Value::from(polygon),
            geo_types::Geometry::MultiPolygon(ref multi_polygon) => {
                geometry::Value::from(multi_polygon)
            }
            _ => panic!("GeometryCollection not allowed"),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{Geometry, Value};
    use geo_types;
    use geo_types::{
        GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
    };
    use std::convert::TryInto;

    #[test]
    fn geo_point_conversion_test() {
        // Test with f32 coordinates
        let geo_point = Point::new(40.02f32, 116.34f32);
        let geojson_point = Value::from(&geo_point);

        if let Value::Point(c) = geojson_point {
            assert_almost_eq!(geo_point.x(), c[0] as f32, 1e-6);
            assert_almost_eq!(geo_point.y(), c[1] as f32, 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_point);
        }

        // Test with f64 coordinates.
        let geo_point = Point::new(40.02f64, 116.34f64);
        let geojson_point = Value::from(&geo_point);

        if let Value::Point(c) = geojson_point {
            assert_almost_eq!(geo_point.x(), c[0], 1e-6);
            assert_almost_eq!(geo_point.y(), c[1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_point);
        }
    }

    #[test]
    fn geo_multi_point_conversion_test() {
        let p1 = Point::new(40.02f64, 116.34f64);
        let p2 = Point::new(13.02f64, 24.34f64);

        let geo_multi_point = MultiPoint(vec![p1, p2]);
        let geojson_multi_point = Value::from(&geo_multi_point);

        if let Value::MultiPoint(c) = geojson_multi_point {
            assert_almost_eq!(p1.x(), c[0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[1][1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_multi_point);
        }
    }

    #[test]
    fn geo_line_string_conversion_test() {
        let p1 = Point::new(40.02f64, 116.34f64);
        let p2 = Point::new(13.02f64, 24.34f64);

        let geo_line_string = LineString::from(vec![p1, p2]);
        let geojson_line_point = Value::from(&geo_line_string);

        if let Value::LineString(c) = geojson_line_point {
            assert_almost_eq!(p1.x(), c[0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[1][1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_line_point);
        }
    }

    #[test]
    fn geo_multi_line_string_conversion_test() {
        let p1 = Point::new(40.02f64, 116.34f64);
        let p2 = Point::new(13.02f64, 24.34f64);
        let p3 = Point::new(46.84f64, 160.95f64);
        let p4 = Point::new(42.02f64, 96.34f64);

        let geo_line_string1 = LineString::from(vec![p1, p2]);
        let geo_line_string2 = LineString::from(vec![p3, p4]);

        let geo_multi_line_string = MultiLineString(vec![geo_line_string1, geo_line_string2]);
        let geojson_multi_line_point = Value::from(&geo_multi_line_string);

        if let Value::MultiLineString(c) = geojson_multi_line_point {
            assert_almost_eq!(p1.x(), c[0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[0][1][1], 1e-6);
            assert_almost_eq!(p3.x(), c[1][0][0], 1e-6);
            assert_almost_eq!(p3.y(), c[1][0][1], 1e-6);
            assert_almost_eq!(p4.x(), c[1][1][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][1][1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_multi_line_point);
        }
    }

    #[test]
    fn geo_polygon_conversion_test() {
        let p1 = Point::new(100.0f64, 0.0f64);
        let p2 = Point::new(101.0f64, 0.0f64);
        let p3 = Point::new(101.0f64, 1.0f64);
        let p4 = Point::new(104.0f64, 0.2f64);
        let p5 = Point::new(100.9f64, 0.2f64);
        let p6 = Point::new(100.9f64, 0.7f64);

        let geo_line_string1 = LineString::from(vec![p1, p2, p3, p1]);
        let geo_line_string2 = LineString::from(vec![p4, p5, p6, p4]);

        let geo_polygon = Polygon::new(geo_line_string1, vec![geo_line_string2]);
        let geojson_polygon = Value::from(&geo_polygon);

        if let Value::Polygon(c) = geojson_polygon {
            assert_almost_eq!(p1.x(), c[0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[0][1][1], 1e-6);
            assert_almost_eq!(p3.x(), c[0][2][0], 1e-6);
            assert_almost_eq!(p3.y(), c[0][2][1], 1e-6);
            assert_almost_eq!(p4.x(), c[1][0][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][0][1], 1e-6);
            assert_almost_eq!(p5.x(), c[1][1][0], 1e-6);
            assert_almost_eq!(p5.y(), c[1][1][1], 1e-6);
            assert_almost_eq!(p6.x(), c[1][2][0], 1e-6);
            assert_almost_eq!(p6.y(), c[1][2][1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_polygon);
        }
    }

    #[test]
    fn geo_multi_polygon_conversion_test() {
        let p1 = Point::new(102.0f64, 2.0f64);
        let p2 = Point::new(103.0f64, 2.0f64);
        let p3 = Point::new(103.0f64, 3.0f64);
        let p4 = Point::new(100.0f64, 0.0f64);
        let p5 = Point::new(101.0f64, 0.0f64);
        let p6 = Point::new(101.0f64, 1.0f64);

        let geo_line_string1 = LineString::from(vec![p1, p2, p3, p1]);
        let geo_line_string2 = LineString::from(vec![p4, p5, p6, p4]);

        let geo_polygon1 = Polygon::new(geo_line_string1, vec![]);
        let geo_polygon2 = Polygon::new(geo_line_string2, vec![]);
        let geo_multi_polygon = MultiPolygon(vec![geo_polygon1, geo_polygon2]);
        let geojson_multi_polygon = Value::from(&geo_multi_polygon);

        if let Value::MultiPolygon(c) = geojson_multi_polygon {
            assert_almost_eq!(p1.x(), c[0][0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][0][1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[0][0][1][1], 1e-6);
            assert_almost_eq!(p3.x(), c[0][0][2][0], 1e-6);
            assert_almost_eq!(p3.y(), c[0][0][2][1], 1e-6);
            assert_almost_eq!(p4.x(), c[1][0][0][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][0][0][1], 1e-6);
            assert_almost_eq!(p5.x(), c[1][0][1][0], 1e-6);
            assert_almost_eq!(p5.y(), c[1][0][1][1], 1e-6);
            assert_almost_eq!(p6.x(), c[1][0][2][0], 1e-6);
            assert_almost_eq!(p6.y(), c[1][0][2][1], 1e-6);
        } else {
            panic!("Not valid geometry {:?}", geojson_multi_polygon);
        }
    }

    #[test]
    fn geo_geometry_collection_conversion_test() {
        let p1 = Point::new(100.0f64, 0.0f64);
        let p2 = Point::new(100.0f64, 1.0f64);
        let p3 = Point::new(101.0f64, 1.0f64);
        let p4 = Point::new(102.0f64, 0.0f64);
        let p5 = Point::new(101.0f64, 0.0f64);
        let geo_multi_point = MultiPoint(vec![p1, p2]);
        let geo_multi_line_string = MultiLineString(vec![
            LineString::from(vec![p1, p2]),
            LineString::from(vec![p2, p3]),
        ]);
        let geo_multi_polygon = MultiPolygon(vec![
            Polygon::new(LineString::from(vec![p3, p4, p5, p3]), vec![]),
            Polygon::new(LineString::from(vec![p1, p5, p3, p1]), vec![]),
        ]);
        let geo_geometry_collection = GeometryCollection(vec![
            geo_types::Geometry::MultiPoint(geo_multi_point),
            geo_types::Geometry::MultiLineString(geo_multi_line_string),
            geo_types::Geometry::MultiPolygon(geo_multi_polygon),
        ]);

        let geojson_geometry_collection = Value::from(&geo_geometry_collection);

        if let Value::GeometryCollection(geometries) = geojson_geometry_collection {
            let geometry_type = |geometry: &Geometry| match geometry.value {
                Value::Point(..) => "Point",
                Value::MultiPoint(..) => "MultiPoint",
                Value::LineString(..) => "LineString",
                Value::MultiLineString(..) => "MultiLineString",
                Value::Polygon(..) => "Polygon",
                Value::MultiPolygon(..) => "MultiPolygon",
                Value::GeometryCollection(..) => "GeometryCollection",
            };

            assert_eq!(3, geometries.len());
            assert_eq!(geometry_type(&geometries[0]), "MultiPoint");
            assert_eq!(geometry_type(&geometries[1]), "MultiLineString");
            assert_eq!(geometry_type(&geometries[2]), "MultiPolygon");
        } else {
            panic!("Not valid geometry {:?}", geojson_geometry_collection);
        }
    }

    #[test]
    fn geojson_point_conversion_test() {
        let coords = vec![100.0, 0.2];
        let geojson_point = Value::Point(coords.clone());
        let geo_point: geo_types::Point<f64> = geojson_point.try_into().unwrap();

        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_multi_point_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geo_multi_point: geo_types::MultiPoint<f64> = geojson_multi_point.try_into().unwrap();

        assert_almost_eq!(geo_multi_point.0[0].x(), coord1[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[0].y(), coord1[1], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].x(), coord2[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].y(), coord2[1], 1e-6);
    }

    #[test]
    fn geojson_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let geo_line_string: geo_types::LineString<f64> = geojson_line_string.try_into().unwrap();

        assert_almost_eq!(geo_line_string.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].y, coord2[1], 1e-6);
    }

    #[test]
    fn geojson_multi_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![102.0, 0.8];
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geo_multi_line_string: geo_types::MultiLineString<f64> =
            geojson_multi_line_string.try_into().unwrap();

        let ref geo_line_string1 = geo_multi_line_string.0[0];
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);

        let ref geo_line_string2 = geo_multi_line_string.0[1];
        assert_almost_eq!(geo_line_string2.0[0].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord3[1], 1e-6);
    }

    #[test]
    fn geojson_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_multi_line_string_type1 = vec![
            vec![
                coord1.clone(),
                coord2.clone(),
                coord3.clone(),
                coord1.clone(),
            ],
            vec![
                coord4.clone(),
                coord5.clone(),
                coord6.clone(),
                coord4.clone(),
            ],
        ];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let ref geo_line_string2 = geo_polygon.interiors()[0];
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_empty_polygon_conversion_test() {
        let geojson_polygon = Value::Polygon(vec![]);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        assert!(geo_polygon.exterior().0.is_empty());
    }

    #[test]
    fn geojson_polygon_without_interiors_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];

        let geojson_multi_line_string_type1 = vec![vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ]];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        assert_eq!(0, geo_polygon.interiors().len());
    }

    #[test]
    fn geojson_multi_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_line_string_type1 = vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ];

        let geojson_line_string_type2 = vec![
            coord4.clone(),
            coord5.clone(),
            coord6.clone(),
            coord4.clone(),
        ];
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![geojson_line_string_type1],
            vec![geojson_line_string_type2],
        ]);
        let geo_multi_polygon: geo_types::MultiPolygon<f64> =
            geojson_multi_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_multi_polygon.0[0].exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let ref geo_line_string2 = geo_multi_polygon.0[1].exterior();
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_geometry_collection_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![100.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![102.0, 0.0];
        let coord5 = vec![101.0, 0.0];

        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![vec![
                coord3.clone(),
                coord4.clone(),
                coord5.clone(),
                coord3.clone(),
            ]],
            vec![vec![
                coord1.clone(),
                coord5.clone(),
                coord3.clone(),
                coord1.clone(),
            ]],
        ]);

        let geojson_geometry_collection = Value::GeometryCollection(vec![
            Geometry::new(geojson_multi_point),
            Geometry::new(geojson_multi_line_string),
            Geometry::new(geojson_multi_polygon),
        ]);

        let geo_geometry_collection: geo_types::GeometryCollection<f64> =
            geojson_geometry_collection.try_into().unwrap();

        assert_eq!(3, geo_geometry_collection.0.len());
    }
}
