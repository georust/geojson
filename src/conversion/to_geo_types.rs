use geo_types::{self, CoordFloat};

use crate::geometry;

use crate::{
    quick_collection, Feature, FeatureCollection, GeoJson, LineStringType, PointType, PolygonType,
};
use crate::{Error, Result};
use std::convert::{TryFrom, TryInto};

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::Point<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::Point(point_type) => Ok(create_geo_point(point_type)),
            other => Err(mismatch_geom_err("Point", other)),
        }
    }
}
try_from_owned_value!(geo_types::Point<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::MultiPoint<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::MultiPoint(multi_point_type) => Ok(geo_types::MultiPoint(
                multi_point_type
                    .iter()
                    .map(|point_type| create_geo_point(point_type))
                    .collect(),
            )),
            other => Err(mismatch_geom_err("MultiPoint", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiPoint<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::LineString<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::LineString(multi_point_type) => {
                Ok(create_geo_line_string(multi_point_type))
            }
            other => Err(mismatch_geom_err("LineString", other)),
        }
    }
}
try_from_owned_value!(geo_types::LineString<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::MultiLineString<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::MultiLineString(multi_line_string_type) => {
                Ok(create_geo_multi_line_string(multi_line_string_type))
            }
            other => Err(mismatch_geom_err("MultiLineString", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiLineString<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::Polygon<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::Polygon(polygon_type) => Ok(create_geo_polygon(polygon_type)),
            other => Err(mismatch_geom_err("Polygon", other)),
        }
    }
}
try_from_owned_value!(geo_types::Polygon<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::MultiPolygon<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<geo_types::MultiPolygon<T>> {
        match value {
            geometry::Value::MultiPolygon(multi_polygon_type) => {
                Ok(create_geo_multi_polygon(multi_polygon_type))
            }
            other => Err(mismatch_geom_err("MultiPolygon", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiPolygon<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::GeometryCollection<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::GeometryCollection(geometries) => {
                let geojson_geometries = geometries
                    .iter()
                    .map(|geometry| (&geometry.value).try_into().unwrap())
                    .collect();

                Ok(geo_types::GeometryCollection(geojson_geometries))
            }
            other => Err(mismatch_geom_err("GeometryCollection", other)),
        }
    }
}
try_from_owned_value!(geo_types::GeometryCollection<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&geometry::Value> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &geometry::Value) -> Result<Self> {
        match value {
            geometry::Value::Point(ref point_type) => {
                Ok(geo_types::Geometry::Point(create_geo_point(point_type)))
            }
            geometry::Value::MultiPoint(ref multi_point_type) => {
                Ok(geo_types::Geometry::MultiPoint(geo_types::MultiPoint(
                    multi_point_type
                        .iter()
                        .map(|point_type| create_geo_point(point_type))
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
            geometry::Value::GeometryCollection(ref gc_type) => {
                let gc = geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(
                    gc_type
                        .iter()
                        .cloned()
                        .map(|geom| geom.try_into())
                        .collect::<Result<Vec<geo_types::Geometry<T>>>>()?,
                ));
                Ok(gc)
            }
        }
    }
}
try_from_owned_value!(geo_types::Geometry<T>);

macro_rules! impl_try_from_geom_value {
    ($($kind:ident),*) => {
        $(
            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<&$crate::Geometry> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: &crate::Geometry) -> Result<Self> {
                    Self::try_from(&geometry.value)
                }
            }

            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<$crate::Geometry> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: crate::Geometry) -> Result<Self> {
                    Self::try_from(geometry.value)
                }
            }

            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<$crate::Feature> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(val: Feature) -> Result<Self> {
                    match val.geometry {
                        None => Err(Error::FeatureHasNoGeometry(val)),
                        Some(geom) => geom.try_into(),
                    }
                }
            }
        )*
    }
}

impl_try_from_geom_value![
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    Geometry,
    GeometryCollection
];

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<FeatureCollection> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(val: FeatureCollection) -> Result<geo_types::Geometry<T>> {
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
    type Error = Error;

    fn try_from(val: GeoJson) -> Result<geo_types::Geometry<T>> {
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
    geo_types::Coordinate {
        x: T::from(point_type[0]).unwrap(),
        y: T::from(point_type[1]).unwrap(),
    }
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
            .map(|point_type| create_geo_line_string(point_type))
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
            .map(|polygon_type| create_geo_polygon(polygon_type))
            .collect(),
    )
}

fn mismatch_geom_err(expected_type: &'static str, found: &geometry::Value) -> Error {
    Error::InvalidGeometryConversion {
        expected_type,
        found_type: found.type_name(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Geometry, Value};
    use serde_json::json;
    use tinyvec::tiny_vec;

    use std::convert::TryInto;

    #[test]
    fn geojson_point_conversion_test() {
        let coords = tiny_vec![100.0, 0.2];
        let geojson_point = Value::Point(coords.clone());
        let geo_point: geo_types::Point<f64> = geojson_point.try_into().unwrap();

        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_multi_point_conversion_test() {
        let coord1 = tiny_vec![100.0, 0.2];
        let coord2 = tiny_vec![101.0, 1.0];
        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geo_multi_point: geo_types::MultiPoint<f64> = geojson_multi_point.try_into().unwrap();

        assert_almost_eq!(geo_multi_point.0[0].x(), coord1[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[0].y(), coord1[1], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].x(), coord2[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].y(), coord2[1], 1e-6);
    }

    #[test]
    fn geojson_line_string_conversion_test() {
        let coord1 = tiny_vec![100.0, 0.2];
        let coord2 = tiny_vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let geo_line_string: geo_types::LineString<f64> = geojson_line_string.try_into().unwrap();

        assert_almost_eq!(geo_line_string.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].y, coord2[1], 1e-6);
    }

    #[test]
    fn geojson_multi_line_string_conversion_test() {
        let coord1 = tiny_vec![100.0, 0.2];
        let coord2 = tiny_vec![101.0, 1.0];
        let coord3 = tiny_vec![102.0, 0.8];
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geo_multi_line_string: geo_types::MultiLineString<f64> =
            geojson_multi_line_string.try_into().unwrap();

        let geo_line_string1 = &geo_multi_line_string.0[0];
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);

        let geo_line_string2 = &geo_multi_line_string.0[1];
        assert_almost_eq!(geo_line_string2.0[0].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord3[1], 1e-6);
    }

    #[test]
    fn geojson_polygon_conversion_test() {
        let coord1 = tiny_vec![100.0, 0.0];
        let coord2 = tiny_vec![101.0, 1.0];
        let coord3 = tiny_vec![101.0, 1.0];
        let coord4 = tiny_vec![104.0, 0.2];
        let coord5 = tiny_vec![100.9, 0.2];
        let coord6 = tiny_vec![100.9, 0.7];

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

        let geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let geo_line_string2 = &geo_polygon.interiors()[0];
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
        let coord1 = tiny_vec![100.0, 0.0];
        let coord2 = tiny_vec![101.0, 1.0];
        let coord3 = tiny_vec![101.0, 1.0];

        let geojson_multi_line_string_type1 = vec![vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ]];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let geo_line_string1 = geo_polygon.exterior();
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
        let coord1 = tiny_vec![100.0, 0.0];
        let coord2 = tiny_vec![101.0, 1.0];
        let coord3 = tiny_vec![101.0, 1.0];
        let coord4 = tiny_vec![104.0, 0.2];
        let coord5 = tiny_vec![100.9, 0.2];
        let coord6 = tiny_vec![100.9, 0.7];

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

        let geo_line_string1 = geo_multi_polygon.0[0].exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let geo_line_string2 = geo_multi_polygon.0[1].exterior();
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
        let coord1 = tiny_vec![100.0, 0.0];
        let coord2 = tiny_vec![100.0, 1.0];
        let coord3 = tiny_vec![101.0, 1.0];
        let coord4 = tiny_vec![102.0, 0.0];
        let coord5 = tiny_vec![101.0, 0.0];

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
        let coords = tiny_vec![100.0, 0.2];
        let geojson_geometry = Geometry::from(Value::Point(coords.clone()));
        let geo_geometry: geo_types::Geometry<f64> = geojson_geometry
            .try_into()
            .expect("Should be able to convert to geo_types::Geometry");
        let geo_point: geo_types::Point<_> =
            geo_geometry.try_into().expect("this should be a point");
        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_mismatch_geometry_conversion_test() {
        let coord1 = tiny_vec![100.0, 0.2];
        let coord2 = tiny_vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        use std::convert::TryFrom;
        let error = geo_types::Point::<f64>::try_from(geojson_line_string).unwrap_err();
        assert_eq!(
            "Expected type: `Point`, but found `LineString`",
            format!("{}", error)
        )
    }

    #[test]
    fn feature_collection_with_geom_collection() {
        let geojson_str = json!({
            "type": "FeatureCollection",
            "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "GeometryCollection",
                    "geometries": [
                    {
                        "type": "Polygon",
                        "coordinates": [
                            [
                                [1.0, 1.0],
                                [2.0, 2.0],
                                [3.0, 1.0],
                                [1.0, 1.0]
                            ]
                        ]
                    }
                    ]
                },
                "properties": {}
            }
            ]
        })
        .to_string();
        let geojson: crate::GeoJson = geojson_str.parse().unwrap();
        let mut geojson_feature_collection: crate::FeatureCollection = geojson.try_into().unwrap();
        let feature: crate::Feature = geojson_feature_collection.features.remove(0);

        use std::convert::TryFrom;
        let geo_geom = geo_types::Geometry::try_from(feature).unwrap();

        let expected =
            geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(vec![
                geo_types::Geometry::Polygon(geo_types::Polygon::new(
                    geo_types::LineString::new(vec![
                        geo_types::coord!(x: 1.0, y: 1.0),
                        geo_types::coord!(x: 2.0, y: 2.0),
                        geo_types::coord!(x: 3.0, y: 1.0),
                        geo_types::coord!(x: 1.0, y: 1.0),
                    ]),
                    vec![],
                )),
            ]));
        assert_eq!(geo_geom, expected);
    }

    #[test]
    fn borrowed_value_conversions_test() -> crate::Result<()> {
        let coord1 = tiny_vec![100.0, 0.2];
        let coord2 = tiny_vec![101.0, 1.0];
        let coord3 = tiny_vec![102.0, 0.8];
        let coord4 = tiny_vec![104.0, 0.2];

        let geojson_point = Value::Point(coord1.clone());
        let _: geo_types::Point<f64> = (&geojson_point).try_into()?;

        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let _: geo_types::MultiPoint<f64> = (&geojson_multi_point).try_into()?;

        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let _: geo_types::LineString<f64> = (&geojson_line_string).try_into()?;

        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let _: geo_types::MultiLineString<f64> = (&geojson_multi_line_string).try_into()?;

        let geojson_multi_line_string_type1 = vec![
            vec![
                coord1.clone(),
                coord2.clone(),
                coord3.clone(),
                coord1.clone(),
            ],
            vec![
                coord4.clone(),
                coord1.clone(),
                coord2.clone(),
                coord4.clone(),
            ],
        ];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let _: geo_types::Polygon<f64> = (&geojson_polygon).try_into()?;

        let geojson_line_string_type1 = vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ];

        let geojson_line_string_type2 = vec![
            coord4.clone(),
            coord3.clone(),
            coord2.clone(),
            coord4.clone(),
        ];
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![geojson_line_string_type1],
            vec![geojson_line_string_type2],
        ]);
        let _: geo_types::MultiPolygon<f64> = (&geojson_multi_polygon).try_into()?;

        let geojson_geometry_collection = Value::GeometryCollection(vec![
            Geometry::new(geojson_multi_point),
            Geometry::new(geojson_multi_line_string),
            Geometry::new(geojson_multi_polygon),
        ]);

        let _: geo_types::GeometryCollection<f64> = (&geojson_geometry_collection).try_into()?;

        Ok(())
    }
}
