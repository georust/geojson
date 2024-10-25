use super::GeometryCollectionType;
use geo_traits::{Dimensions, GeometryTrait};

impl geo_traits::GeometryCollectionTrait for GeometryCollectionType {
    type T = f64;
    type GeometryType<'b>
        = &'b crate::Geometry
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.geometry(0).unwrap().dim()
    }

    fn geometries(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::GeometryType<'_>> {
        self.0.iter()
    }

    fn geometry(&self, i: usize) -> Option<Self::GeometryType<'_>> {
        self.0.get(i)
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.0.get_unchecked(i)
    }

    fn num_geometries(&self) -> usize {
        self.0.len()
    }
}

impl<'a> geo_traits::GeometryCollectionTrait for &'a GeometryCollectionType {
    type T = f64;
    type GeometryType<'b>
        = &'b crate::Geometry
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        GeometryCollectionType::dim(self)
    }

    fn geometries(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::GeometryType<'_>> {
        GeometryCollectionType::geometries(self)
    }

    fn geometry(&self, i: usize) -> Option<Self::GeometryType<'_>> {
        GeometryCollectionType::geometry(self, i)
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        GeometryCollectionType::geometry_unchecked(self, i)
    }

    fn num_geometries(&self) -> usize {
        GeometryCollectionType::num_geometries(self)
    }
}
