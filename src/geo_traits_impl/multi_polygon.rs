use super::{MultiPolygonType, PolygonType};
use bytemuck::TransparentWrapper;
use geo_traits::{Dimensions, PolygonTrait};

impl geo_traits::MultiPolygonTrait for MultiPolygonType {
    type T = f64;
    type PolygonType<'b>
        = &'b PolygonType
    where
        Self: 'b;

    fn num_polygons(&self) -> usize {
        self.0.len()
    }

    fn dim(&self) -> Dimensions {
        self.polygon(0).map_or(Dimensions::Unknown(0), |p| p.dim())
    }

    fn polygon(&self, i: usize) -> Option<Self::PolygonType<'_>> {
        self.0.get(i).map(PolygonType::wrap_ref)
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        PolygonType::wrap_ref(self.0.get_unchecked(i))
    }

    fn polygons(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::PolygonType<'_>> {
        self.0.iter().map(PolygonType::wrap_ref)
    }
}

impl<'a> geo_traits::MultiPolygonTrait for &'a MultiPolygonType {
    type T = f64;
    type PolygonType<'b>
        = &'b PolygonType
    where
        Self: 'b;

    fn num_polygons(&self) -> usize {
        MultiPolygonType::num_polygons(self)
    }

    fn dim(&self) -> Dimensions {
        MultiPolygonType::dim(self)
    }

    fn polygon(&self, i: usize) -> Option<Self::PolygonType<'_>> {
        MultiPolygonType::polygon(self, i)
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        MultiPolygonType::polygon_unchecked(self, i)
    }

    fn polygons(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::PolygonType<'_>> {
        MultiPolygonType::polygons(self)
    }
}
