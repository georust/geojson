use super::PointType;
use geo_traits::{Dimensions, PointTrait};

impl PointTrait for PointType {
    type T = f64;
    type CoordType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        Some(self)
    }

    fn dim(&self) -> Dimensions {
        <Self as geo_traits::CoordTrait>::dim(self)
    }
}

impl geo_traits::PointTrait for &PointType {
    type T = f64;
    type CoordType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        PointType::coord(self)
    }

    fn dim(&self) -> Dimensions {
        PointType::dim(self)
    }
}
