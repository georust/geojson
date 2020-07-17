use crate::geo_types;

use crate::geometry;

use num_traits::Float;
use std::convert::From;

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::Point<T>> for geometry::Value
where
    T: Float,
{
    fn from(point: &geo_types::Point<T>) -> Self {
        let coords = create_point_type(point);

        geometry::Value::Point(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
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

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::LineString<T>> for geometry::Value
where
    T: Float,
{
    fn from(line_string: &geo_types::LineString<T>) -> Self {
        let coords = create_line_string_type(line_string);

        geometry::Value::LineString(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::MultiLineString<T>> for geometry::Value
where
    T: Float,
{
    fn from(multi_line_string: &geo_types::MultiLineString<T>) -> Self {
        let coords = create_multi_line_string_type(multi_line_string);

        geometry::Value::MultiLineString(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::Polygon<T>> for geometry::Value
where
    T: Float,
{
    fn from(polygon: &geo_types::Polygon<T>) -> Self {
        let coords = create_polygon_type(polygon);

        geometry::Value::Polygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::MultiPolygon<T>> for geometry::Value
where
    T: Float,
{
    fn from(multi_polygon: &geo_types::MultiPolygon<T>) -> Self {
        let coords = create_multi_polygon_type(multi_polygon);

        geometry::Value::MultiPolygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
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

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
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

#[cfg(test)]
mod tests {
    use crate::{Geometry, Value};
    use geo_types;
    use geo_types::{
        GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
    };

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
}
