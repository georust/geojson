use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use tinyvec::TinyVec;

/// Positions
///
/// [GeoJSON Format Specification § 3.1.1](https://tools.ietf.org/html/rfc7946#section-3.1.1)
///
/// ## Examples
/// ```
/// use geojson::Position;
/// let position_1 = Position::from([1.0, 2.0]);
/// assert_eq!(position_1[0], 1.0);
/// assert_eq!(position_1.as_slice(), &[1.0, 2.0]);
///
/// let position_2 = Position::from(vec![3.0, 4.0]);
/// assert_eq!(position_2[1], 4.0);
/// assert_eq!(position_2.as_slice(), &[3.0, 4.0]);
/// ```
///
/// As always, an out of bound access will panic.
/// ```
/// use geojson::Position;
/// let position_2d = Position::from([1.0, 2.0]);
/// // panics!
/// // let z = position_2d[2];
/// let position_3d = Position::from(vec![1.0, 2.0, 3.0]);
/// let z = position_3d[2];
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Position<const INLINE_SIZE: usize = 2>(tinyvec::TinyVec<[f64; INLINE_SIZE]>);

impl<const INLINE_SIZE: usize> Position<INLINE_SIZE> {
    pub fn as_slice(&self) -> &[f64] {
        self.0.as_slice()
    }

    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        self.0.as_mut_slice()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn from_values(floats: tinyvec::TinyVec<[f64; INLINE_SIZE]>) -> Self {
        Self(floats)
    }
}

impl<I: SliceIndex<[f64]>, const INLINE_SIZE: usize> Index<I> for Position<INLINE_SIZE> {
    type Output = <I as SliceIndex<[f64]>>::Output;
    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I: SliceIndex<[f64]>, const INLINE_SIZE: usize> IndexMut<I> for Position<INLINE_SIZE> {
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<TinyVec<[f64; 2]>> for Position {
    fn from(value: TinyVec<[f64; 2]>) -> Self {
        Self(value)
    }
}

impl From<Vec<f64>> for Position {
    fn from(value: Vec<f64>) -> Self {
        Self(TinyVec::Heap(value))
    }
}

impl From<[f64; 2]> for Position {
    fn from(value: [f64; 2]) -> Self {
        Self(TinyVec::Inline(value.into()))
    }
}

impl From<(f64, f64)> for Position {
    fn from(value: (f64, f64)) -> Self {
        Self::from([value.0, value.1])
    }
}

impl From<[f64; 3]> for Position {
    fn from(value: [f64; 3]) -> Self {
        Self(TinyVec::Heap(value.into()))
    }
}

impl From<(f64, f64, f64)> for Position {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::from([value.0, value.1, value.2])
    }
}

impl From<[f64; 4]> for Position {
    fn from(value: [f64; 4]) -> Self {
        Self(TinyVec::Heap(value.into()))
    }
}

impl From<(f64, f64, f64, f64)> for Position {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self::from([value.0, value.1, value.2, value.3])
    }
}
