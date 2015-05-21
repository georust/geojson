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

use rustc_serialize::json::{Json, Object};

use ::{Bbox, Crs, Error, Feature, FromObject, Geometry, Position};


pub trait ObjectUtils {
    fn get_bbox(&self) -> Result<Option<Bbox>, Error>;
    fn get_crs(&self) -> Result<Option<Crs>, Error>;
    fn get_properties(&self) -> Result<Option<Object>, Error>;
    fn get_coords_one_pos(&self) -> Result<Position, Error>;
    fn get_coords_1d_pos(&self) -> Result<Vec<Position>, Error>;
    fn get_coords_2d_pos(&self) -> Result<Vec<Vec<Position>>, Error>;
    fn get_coords_3d_pos(&self) -> Result<Vec<Vec<Vec<Position>>>, Error>;
    fn get_geometries(&self) -> Result<Vec<Geometry>, Error>;
    fn get_id(&self) -> Result<Option<Json>, Error>;
    fn get_geometry(&self) -> Result<Geometry, Error>;
    fn get_features(&self) -> Result<Vec<Feature>, Error>;
    fn get_coords_value(&self) -> Result<&Json, Error>;
}

impl ObjectUtils for Object {
    fn get_coords_value(&self) -> Result<&Json, Error> {
        return Ok(expect_property!(self, "coordinates", "Encountered Geometry object without 'coordinates' member"));
    }

    /// Used by FeatureCollection, Feature, Geometry
    fn get_bbox(&self) -> Result<Option<Bbox>, Error> {
        let bbox_json = match self.get("bbox") {
            Some(b) => b,
            None => return Ok(None),
        };

        let bbox_array = match bbox_json.as_array() {
            Some(b) => b,
            None => return Err(Error::new("Encountered 'bbox' with non-array value")),
        };

        let mut bbox = vec![];
        for item_json in bbox_array {
            match item_json.as_f64() {
                Some(item_f64) => bbox.push(item_f64),
                None => return Err(Error::new("Encountered non numeric value in 'bbox' array")),
            }
        }

        return Ok(Some(bbox));
    }

    /// Used by FeatureCollection, Feature, Geometry
    fn get_crs(&self) -> Result<Option<Crs>, Error> {
        let crs_json = match self.get("crs") {
            Some(b) => b,
            None => return Ok(None),
        };

        let crs_object = match crs_json.as_object() {
            Some(c) => c,
            None => return Err(Error::new("Encountered 'crs' with non-object value")),
        };

        return Crs::from_object(crs_object).map(Some);
    }

    /// Used by Feature
    fn get_properties(&self) -> Result<Option<Object>, Error> {
        let properties = expect_property!(self, "properties", "missing 'properties' field");
        return match *properties {
            Json::Object(ref x) => Ok(Some(x.clone())),
            Json::Null => Ok(None),
            _ => return Err(Error::new("expected an Object or Null value for feature properties")),
        };
    }

    /// Retrieve a single Position from the value of the "coordinates" key
    ///
    /// Used by Value::Point
    fn get_coords_one_pos(&self) -> Result<Position, Error> {
        let coords_json = try!(self.get_coords_value());
        return json_to_position(&coords_json);
    }

    /// Retrieve a one dimensional Vec of Positions from the value of the "coordinates" key
    ///
    /// Used by Value::MultiPoint and Value::LineString
    fn get_coords_1d_pos(&self) -> Result<Vec<Position>, Error> {
        let coords_json = try!(self.get_coords_value());
        return json_to_1d_positions(&coords_json);
    }

    /// Retrieve a two dimensional Vec of Positions from the value of the "coordinates" key
    ///
    /// Used by Value::MultiLineString and Value::Polygon
    fn get_coords_2d_pos(&self) -> Result<Vec<Vec<Position>>, Error> {
        let coords_json = try!(self.get_coords_value());
        return json_to_2d_positions(&coords_json);
    }

    /// Retrieve a three dimensional Vec of Positions from the value of the "coordinates" key
    ///
    /// Used by Value::MultiPolygon
    fn get_coords_3d_pos(&self) -> Result<Vec<Vec<Vec<Position>>>, Error> {
        let coords_json = try!(self.get_coords_value());
        return json_to_3d_positions(&coords_json);
    }

    /// Used by Value::GeometryCollection
    fn get_geometries(&self) -> Result<Vec<Geometry>, Error> {
        let geometries_json = expect_property!(self, "geometries", "Encountered GeometryCollection without 'geometries' property");
        let geometries_array = expect_array!(geometries_json);
        let mut geometries = vec![];
        for json in geometries_array {
            let obj = expect_object!(json);
            let geometry = try!(Geometry::from_object(obj));
            geometries.push(geometry);
        }
        return Ok(geometries);
    }

    /// Used by Feature
    fn get_id(&self) -> Result<Option<Json>, Error> {
        return Ok(self.get("id").map(Clone::clone));
    }

    /// Used by Feature
    fn get_geometry(&self) -> Result<Geometry, Error> {
        let geometry = expect_object!(expect_property!(self, "geometry", "Missing 'geometry' field"));
        return Geometry::from_object(geometry);
    }

    /// Used by FeatureCollection
    fn get_features(&self) -> Result<Vec<Feature>, Error> {
        let mut features = vec![];
        let features_json = expect_array!(expect_property!(self, "features", "Missing 'features' field"));
        for feature in features_json {
            let feature: &Object = expect_object!(feature);
            let feature: Feature = try!(Feature::from_object(feature));
            features.push(feature);
        }
        return Ok(features);
    }
}


fn json_to_position(json: &Json) -> Result<Position, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for position in coords_array {
        coords.push(expect_f64!(position));
    }
    return Ok(coords);
}

fn json_to_1d_positions(json: &Json) -> Result<Vec<Position>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_position(item)));
    }
    return Ok(coords);
}

fn json_to_2d_positions(json: &Json) -> Result<Vec<Vec<Position>>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_1d_positions(item)));
    }
    return Ok(coords);
}

fn json_to_3d_positions(json: &Json) -> Result<Vec<Vec<Vec<Position>>>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_2d_positions(item)));
    }
    return Ok(coords);
}
