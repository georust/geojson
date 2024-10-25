use super::PointType;
use geo_traits::{CoordTrait, Dimensions};

impl geo_traits::CoordTrait for PointType {
    type T = f64;

    fn dim(&self) -> Dimensions {
        match self.0.len() {
            0 | 1 => panic!("Position must have at least 2 dimensions"),
            2 => Dimensions::Xy,
            3 => Dimensions::Xyz,
            _ => Dimensions::Unknown(self.0.len()),
        }
    }

    fn x(&self) -> Self::T {
        self.0[0]
    }

    fn y(&self) -> Self::T {
        self.0[1]
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        self.0[n]
    }
}

impl geo_traits::CoordTrait for &PointType {
    type T = f64;

    fn dim(&self) -> Dimensions {
        PointType::dim(self)
    }

    fn x(&self) -> Self::T {
        PointType::x(self)
    }

    fn y(&self) -> Self::T {
        PointType::y(self)
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        PointType::nth_unchecked(self, n)
    }
}
