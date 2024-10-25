use super::{MultiPointType, PointType};
use bytemuck::TransparentWrapper;
use geo_traits::{Dimensions, PointTrait};

impl geo_traits::MultiPointTrait for MultiPointType {
    type T = f64;
    type PointType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn num_points(&self) -> usize {
        self.0.len()
    }

    fn dim(&self) -> Dimensions {
        self.point(0).unwrap().dim()
    }

    fn point(&self, i: usize) -> Option<Self::PointType<'_>> {
        self.0.get(i).map(PointType::wrap_ref)
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        PointType::wrap_ref(self.0.get_unchecked(i))
    }

    fn points(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::PointType<'_>> {
        self.0.iter().map(PointType::wrap_ref)
    }
}

impl<'a> geo_traits::MultiPointTrait for &'a MultiPointType {
    type T = f64;
    type PointType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn num_points(&self) -> usize {
        MultiPointType::num_points(self)
    }

    fn dim(&self) -> Dimensions {
        MultiPointType::dim(self)
    }

    fn point(&self, i: usize) -> Option<Self::PointType<'_>> {
        MultiPointType::point(self, i)
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::PointType<'_> {
        MultiPointType::point_unchecked(self, i)
    }

    fn points(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::PointType<'_>> {
        MultiPointType::points(self)
    }
}
