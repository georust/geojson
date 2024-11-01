use super::{
    GeometryCollectionType, LineStringType, MultiLineStringType, MultiPointType, MultiPolygonType,
    PointType, PolygonType,
};
use bytemuck::TransparentWrapper;
use geo_traits::{
    Dimensions, GeometryCollectionTrait, LineStringTrait, MultiLineStringTrait, MultiPointTrait,
    MultiPolygonTrait, PointTrait, PolygonTrait, UnimplementedLine, UnimplementedRect,
    UnimplementedTriangle,
};

impl geo_traits::GeometryTrait for crate::Value {
    type T = f64;
    type PointType<'a> = PointType;
    type LineStringType<'a> = LineStringType;
    type PolygonType<'a> = PolygonType;
    type MultiPointType<'a> = MultiPointType;
    type MultiLineStringType<'a> = MultiLineStringType;
    type MultiPolygonType<'a> = MultiPolygonType;
    type GeometryCollectionType<'a> = GeometryCollectionType;
    type RectType<'a> = UnimplementedRect<Self::T>;
    type TriangleType<'a> = UnimplementedTriangle<Self::T>;
    type LineType<'a> = UnimplementedLine<Self::T>;

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            crate::Value::Point(p) => geo_traits::GeometryType::Point(PointType::wrap_ref(p)),
            crate::Value::LineString(ls) => {
                geo_traits::GeometryType::LineString(LineStringType::wrap_ref(ls))
            }
            crate::Value::Polygon(p) => geo_traits::GeometryType::Polygon(PolygonType::wrap_ref(p)),
            crate::Value::MultiPoint(mp) => {
                geo_traits::GeometryType::MultiPoint(MultiPointType::wrap_ref(mp))
            }
            crate::Value::MultiLineString(mls) => {
                geo_traits::GeometryType::MultiLineString(MultiLineStringType::wrap_ref(mls))
            }
            crate::Value::MultiPolygon(mp) => {
                geo_traits::GeometryType::MultiPolygon(MultiPolygonType::wrap_ref(mp))
            }
            crate::Value::GeometryCollection(gc) => {
                geo_traits::GeometryType::GeometryCollection(GeometryCollectionType::wrap_ref(gc))
            }
        }
    }

    fn dim(&self) -> Dimensions {
        match self {
            crate::Value::Point(ref p) => PointType::wrap_ref(p).dim(),
            crate::Value::LineString(ref ls) => LineStringType::wrap_ref(ls).dim(),
            crate::Value::Polygon(ref p) => PolygonType::wrap_ref(p).dim(),
            crate::Value::MultiPoint(ref mp) => MultiPointType::wrap_ref(mp).dim(),
            crate::Value::MultiLineString(ref mls) => MultiLineStringType::wrap_ref(mls).dim(),
            crate::Value::MultiPolygon(ref mp) => MultiPolygonType::wrap_ref(mp).dim(),
            crate::Value::GeometryCollection(ref gc) => GeometryCollectionType::wrap_ref(gc).dim(),
        }
    }
}

impl geo_traits::GeometryTrait for &crate::Value {
    type T = f64;
    type PointType<'b>
        = PointType
    where
        Self: 'b;
    type LineStringType<'b>
        = LineStringType
    where
        Self: 'b;
    type PolygonType<'b>
        = PolygonType
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPointType
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineStringType
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygonType
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollectionType
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<Self::T>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<Self::T>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<Self::T>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        crate::Value::dim(self)
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        crate::Value::as_type(self)
    }
}

impl geo_traits::GeometryTrait for crate::Geometry {
    type T = f64;
    type PointType<'b> = PointType;
    type LineStringType<'b> = LineStringType;
    type PolygonType<'b> = PolygonType;
    type MultiPointType<'b> = MultiPointType;
    type MultiLineStringType<'b> = MultiLineStringType;
    type MultiPolygonType<'b> = MultiPolygonType;
    type GeometryCollectionType<'b> = GeometryCollectionType;
    type RectType<'b> = UnimplementedRect<Self::T>;
    type TriangleType<'b> = UnimplementedTriangle<Self::T>;
    type LineType<'b> = UnimplementedLine<Self::T>;

    fn dim(&self) -> Dimensions {
        self.value.dim()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        self.value.as_type()
    }
}

impl geo_traits::GeometryTrait for &crate::Geometry {
    type T = f64;
    type PointType<'b>
        = PointType
    where
        Self: 'b;
    type LineStringType<'b>
        = LineStringType
    where
        Self: 'b;
    type PolygonType<'b>
        = PolygonType
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPointType
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineStringType
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygonType
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollectionType
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<Self::T>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<Self::T>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<Self::T>
    where
        Self: 'b;

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        crate::Geometry::as_type(self)
    }

    fn dim(&self) -> Dimensions {
        crate::Geometry::dim(self)
    }
}

impl geo_traits::GeometryTrait for crate::Feature {
    type T = f64;
    type PointType<'b> = PointType;
    type LineStringType<'b> = LineStringType;
    type PolygonType<'b> = PolygonType;
    type MultiPointType<'b> = MultiPointType;
    type MultiLineStringType<'b> = MultiLineStringType;
    type MultiPolygonType<'b> = MultiPolygonType;
    type GeometryCollectionType<'b> = GeometryCollectionType;
    type RectType<'b> = UnimplementedRect<Self::T>;
    type TriangleType<'b> = UnimplementedTriangle<Self::T>;
    type LineType<'b> = UnimplementedLine<Self::T>;

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self.geometry {
            Some(ref g) => g.as_type(),
            None => panic!("GeoJSON feature has no geometry"),
        }
    }

    fn dim(&self) -> Dimensions {
        match self.geometry {
            Some(ref g) => g.dim(),
            None => panic!("GeoJSON feature has no geometry"),
        }
    }
}

impl geo_traits::GeometryTrait for &crate::Feature {
    type T = f64;
    type PointType<'b>
        = PointType
    where
        Self: 'b;
    type LineStringType<'b>
        = LineStringType
    where
        Self: 'b;
    type PolygonType<'b>
        = PolygonType
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPointType
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineStringType
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygonType
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollectionType
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<Self::T>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<Self::T>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<Self::T>
    where
        Self: 'b;

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        crate::Feature::as_type(self)
    }

    fn dim(&self) -> Dimensions {
        crate::Feature::dim(self)
    }
}

impl geo_traits::GeometryTrait for crate::GeoJson {
    type T = f64;
    type PointType<'b>
        = PointType
    where
        Self: 'b;
    type LineStringType<'b>
        = LineStringType
    where
        Self: 'b;
    type PolygonType<'b>
        = PolygonType
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPointType
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineStringType
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygonType
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollectionType
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<Self::T>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<Self::T>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<Self::T>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        match self {
            crate::GeoJson::Feature(f) => f.dim(),
            crate::GeoJson::FeatureCollection(fc) => fc.dim(),
            crate::GeoJson::Geometry(g) => g.dim(),
        }
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            crate::GeoJson::Feature(f) => f.as_type(),
            crate::GeoJson::FeatureCollection(_fc) => {
                unimplemented!("TODO")
            }
            crate::GeoJson::Geometry(g) => g.as_type(),
        }
    }
}
