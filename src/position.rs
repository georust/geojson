use crate::json::JsonValue;
use crate::{util, Error};
use std::fmt::Debug;

/// Positions
///
/// [GeoJSON Format Specification § 3.1.1](https://tools.ietf.org/html/rfc7946#section-3.1.1)
pub trait Position: Sized + Clone + Debug + serde::Serialize {
    type Z;

    fn from_json_value(json: &JsonValue) -> Result<Self, Error<Self>>;
    fn from_x_y(x: f64, y: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> Self::Z;
}
// TODO: should this derivce serialize unconditionally?

impl Position for Vec<f64> {
    type Z = Option<f64>;

    fn from_json_value(json: &JsonValue) -> Result<Self, Error<Self>> {
        let coords_array = util::expect_array(json)?;
        let mut coords = Vec::with_capacity(coords_array.len());
        for position in coords_array {
            coords.push(util::expect_f64(position)?);
        }
        Ok(coords)
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        vec![x, y]
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn z(&self) -> Self::Z {
        self.get(2).copied()
    }
}

impl Position for (f64, f64) {
    type Z = ();

    fn from_json_value(json: &JsonValue) -> Result<Self, Error<Self>> {
        let coords_array = util::expect_array(json)?;
        if coords_array.len() != 2 {
            unimplemented!()
        }
        Ok((
            util::expect_f64(&coords_array[0])?,
            util::expect_f64(&coords_array[1])?,
        ))
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        (x, y)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> Self::Z {}
}

impl Position for (f64, f64, f64) {
    type Z = f64;

    fn from_json_value(json: &JsonValue) -> Result<Self, Error<Self>> {
        let coords_array = util::expect_array(json)?;
        if coords_array.len() != 3 {
            unimplemented!()
        }
        Ok((
            util::expect_f64(&coords_array[0])?,
            util::expect_f64(&coords_array[1])?,
            util::expect_f64(&coords_array[2])?,
        ))
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        (x, y, 0.)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> Self::Z {
        self.2
    }
}

impl Position for (f64, f64, Option<f64>) {
    type Z = Option<f64>;

    fn from_json_value(json: &JsonValue) -> Result<Self, Error<Self>> {
        let coords_array = util::expect_array(json)?;
        if coords_array.len() == 2 {
            Ok((
                util::expect_f64(&coords_array[0])?,
                util::expect_f64(&coords_array[1])?,
                None,
            ))
        } else if coords_array.len() == 2 {
            Ok((
                util::expect_f64(&coords_array[0])?,
                util::expect_f64(&coords_array[1])?,
                Some(util::expect_f64(&coords_array[2])?),
            ))
        } else {
            unimplemented!()
        }
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        (x, y, None)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> Self::Z {
        self.2
    }
}
