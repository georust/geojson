// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rustc_serialize::json::{Json, ToJson, Object};

use {Poly, GeoJsonResult, FromJson};
use util::new_geometry_object;


/// MultiPolygon
///
/// [GeoJSON Format Specification § 2.1.7](http://geojson.org/geojson-spec.html#multipolygon)
#[derive(RustcEncodable, Clone, Debug)]
pub struct MultiPolygon {
    pub coordinates: Vec<Poly>,
}

impl ToJson for MultiPolygon {
    fn to_json(&self) -> Json {
        new_geometry_object("MultiPolygon", self.coordinates.to_json())
    }
}

impl FromJson for MultiPolygon {
    fn from_json(json_geometry: &Object) -> GeoJsonResult<Self> {
        let mut coordinates = vec![];
        for json_poly in expect_array!(expect_property!(json_geometry, "coordinates", "missing 'coordinates' field")) {
            coordinates.push(try!(Poly::from_json(expect_array!(json_poly))));
        }
        Ok(MultiPolygon{coordinates: coordinates})
    }
}


#[cfg(test)]
mod tests {
    use rustc_serialize::json::{Json, ToJson};
    use {Pos, MultiPolygon, Poly, Ring, FromJson};

    #[test]
    fn test_multi_polygon_to_json() {
        let multi_polygon = MultiPolygon{coordinates: vec![Poly(vec![
            Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
            Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
            ])]};
        let json_string = multi_polygon.to_json().to_string();
        assert_eq!("{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}", json_string);
    }

    #[test]
    fn test_multi_polygon_from_json() {
        let json_string = "{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}";
        let json_doc = Json::from_str(json_string).unwrap();
        let multi_polygon = MultiPolygon::from_json(json_doc.as_object().unwrap()).ok().unwrap();
        assert_eq!(json_string, multi_polygon.to_json().to_string());
    }
}
