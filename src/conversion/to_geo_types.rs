use crate::geo_types::{self, CoordFloat};

use crate::geometry;

use crate::Error as GJError;
use crate::{
    quick_collection, Feature, FeatureCollection, GeoJson, Geometry, LineStringType, PointType,
    PolygonType,
};
use std::convert::{TryFrom, TryInto};

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::Point<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::Point(point_type) => Ok(create_geo_point(&point_type)),
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::MultiPoint<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::MultiPoint(multi_point_type) => Ok(geo_types::MultiPoint(
                multi_point_type
                    .iter()
                    .map(|point_type| create_geo_point(&point_type))
                    .collect(),
            )),
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::LineString<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::LineString(multi_point_type) => {
                Ok(create_geo_line_string(&multi_point_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::MultiLineString<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::MultiLineString(multi_line_string_type) => {
                Ok(create_geo_multi_line_string(&multi_line_string_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::Polygon<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::Polygon(polygon_type) => Ok(create_geo_polygon(&polygon_type)),
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::MultiPolygon<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<geo_types::MultiPolygon<T>, Self::Error> {
        match value {
            geometry::Value::MultiPolygon(multi_polygon_type) => {
                Ok(create_geo_multi_polygon(&multi_polygon_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::GeometryCollection<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
            geometry::Value::GeometryCollection(geometries) => {
                let geojson_geometries = geometries
                    .iter()
                    .map(|geometry| geometry.value.clone().try_into().unwrap())
                    .collect();

                Ok(geo_types::GeometryCollection(geojson_geometries))
            }
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<geometry::Value> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(value: geometry::Value) -> Result<Self, Self::Error> {
        match value {
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
            _ => Err(GJError::InvalidGeometryConversion(value)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<Geometry> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(val: Geometry) -> Result<geo_types::Geometry<T>, Self::Error> {
        val.value.try_into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<Feature> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(val: Feature) -> Result<geo_types::Geometry<T>, Self::Error> {
        match val.geometry {
            None => Err(GJError::FeatureHasNoGeometry(val)),
            Some(geom) => geom.try_into(),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<FeatureCollection> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(val: FeatureCollection) -> Result<geo_types::Geometry<T>, Self::Error> {
        Ok(geo_types::Geometry::GeometryCollection(quick_collection(
            &GeoJson::FeatureCollection(val),
        )?))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<GeoJson> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = GJError;

    fn try_from(val: GeoJson) -> Result<geo_types::Geometry<T>, Self::Error> {
        match val {
            GeoJson::Geometry(geom) => geom.try_into(),
            GeoJson::Feature(feat) => feat.try_into(),
            GeoJson::FeatureCollection(fc) => fc.try_into(),
        }
    }
}

fn create_geo_coordinate<T>(point_type: &PointType) -> geo_types::Coordinate<T>
where
    T: CoordFloat,
{
    geo_types::Coordinate::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
    )
}

fn create_geo_point<T>(point_type: &PointType) -> geo_types::Point<T>
where
    T: CoordFloat,
{
    geo_types::Point::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
    )
}

fn create_geo_line_string<T>(line_type: &LineStringType) -> geo_types::LineString<T>
where
    T: CoordFloat,
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
    T: CoordFloat,
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
    T: CoordFloat,
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

    geo_types::Polygon::new(exterior, interiors)
}

fn create_geo_multi_polygon<T>(multi_polygon_type: &[PolygonType]) -> geo_types::MultiPolygon<T>
where
    T: CoordFloat,
{
    geo_types::MultiPolygon(
        multi_polygon_type
            .iter()
            .map(|polygon_type| create_geo_polygon(&polygon_type))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{Geometry, Value};
    use geo_types;

    use std::convert::TryInto;

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
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
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

    #[test]
    fn geojson_geometry_conversion() {
        let coords = vec![100.0, 0.2];
        let geojson_geometry = Geometry::from(Value::Point(coords.clone()));
        let geo_geometry: geo_types::Geometry<f64> = geojson_geometry
            .try_into()
            .expect("Shoudl be able to convert to geo_types::Geometry");
        let geo_point: geo_types::Point<_> =
            geo_geometry.try_into().expect("this should be a point");
        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }
}
