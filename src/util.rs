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

use std::collections::HashSet;

use json::{JsonValue, JsonObject};

use {Bbox, Crs, Error, Feature, FromObject, Geometry, Position};


pub fn get_coords_value<'a>(object: &JsonObject) -> Result<&JsonValue, Error> {
    return Ok(expect_property!(object,
                               "coordinates",
                               "Encountered Geometry object without 'coordinates' member"));
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_bbox(object: &JsonObject) -> Result<Option<Bbox>, Error> {
    let bbox_json = match object.get("bbox") {
        Some(b) => b,
        None => return Ok(None),
    };

    let bbox_array = match bbox_json.as_array() {
        Some(b) => b,
        None => return Err(Error::BboxExpectedArray),
    };

    let mut bbox = vec![];
    for item_json in bbox_array {
        match item_json.as_f64() {
            Some(item_f64) => bbox.push(item_f64),
            None => return Err(Error::BboxExpectedNumericValues),
        }
    }

    return Ok(Some(bbox));
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_crs(object: &JsonObject) -> Result<Option<Crs>, Error> {
    let crs_json = match object.get("crs") {
        Some(b) => b,
        None => return Ok(None),
    };

    let crs_object = match crs_json.as_object() {
        Some(c) => c,
        None => return Err(Error::CrsExpectedObject),
    };

    return Crs::from_object(crs_object).map(Some);
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_foreign_members(object: &JsonObject, parent: &str) -> Result<Option<JsonObject>, Error> {
    let mut res = JsonObject::new();
    let ref_keys: HashSet<&str> = match parent {
        "Geometry" => [ "type", "bbox", "crs", "coordinates", "geometries" ].iter().cloned().collect(),
        "Feature" =>  [ "type", "bbox", "crs", "properties", "geometry" ].iter().cloned().collect(),
        "FeatureCollection" => [ "type", "bbox", "crs", "features" ].iter().cloned().collect(),
        _ => return Err(Error::GeoJsonUnknownType)
    };
    for (key, value) in object {
        if !ref_keys.contains(&key.as_str()) {
            res.insert(key.to_owned(), value.to_owned());
        }
    }
    if res.is_empty() {
        Ok(None)
    } else {
        Ok(Some(res))
    }
}

/// Used by Feature
pub fn get_properties(object: &JsonObject) -> Result<Option<JsonObject>, Error> {
    let properties = expect_property!(object, "properties", "missing 'properties' field");
    return match *properties {
        JsonValue::Object(ref x) => Ok(Some(x.clone())),
        JsonValue::Null => Ok(None),
        _ => return Err(Error::PropertiesExpectedObjectOrNull),
    };
}

/// Retrieve a single Position from the value of the "coordinates" key
///
/// Used by Value::Point
pub fn get_coords_one_pos(object: &JsonObject) -> Result<Position, Error> {
    let coords_json = try!(get_coords_value(object));
    return json_to_position(&coords_json);
}

/// Retrieve a one dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPoint and Value::LineString
pub fn get_coords_1d_pos(object: &JsonObject) -> Result<Vec<Position>, Error> {
    let coords_json = try!(get_coords_value(object));
    return json_to_1d_positions(&coords_json);
}

/// Retrieve a two dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiLineString and Value::Polygon
pub fn get_coords_2d_pos(object: &JsonObject) -> Result<Vec<Vec<Position>>, Error> {
    let coords_json = try!(get_coords_value(object));
    return json_to_2d_positions(&coords_json);
}

/// Retrieve a three dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPolygon
pub fn get_coords_3d_pos(object: &JsonObject) -> Result<Vec<Vec<Vec<Position>>>, Error> {
    let coords_json = try!(get_coords_value(object));
    return json_to_3d_positions(&coords_json);
}

/// Used by Value::GeometryCollection
pub fn get_geometries(object: &JsonObject) -> Result<Vec<Geometry>, Error> {
    let geometries_json = expect_property!(object,
                                           "geometries",
                                           "Encountered GeometryCollection without 'geometries' \
                                            property");
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
pub fn get_id(object: &JsonObject) -> Result<Option<JsonValue>, Error> {
    return Ok(object.get("id").map(Clone::clone));
}

/// Used by Feature
pub fn get_geometry(object: &JsonObject) -> Result<Option<Geometry>, Error> {
    let geometry = expect_property!(object, "geometry", "Missing 'geometry' field");
    match *geometry {
        JsonValue::Object(ref x) => {
            let geometry_object = try!(Geometry::from_object(x));
            Ok(Some(geometry_object))
        }
        JsonValue::Null => Ok(None),
        _ => Err(Error::FeatureInvalidGeometryValue),
    }
}

/// Used by FeatureCollection
pub fn get_features(object: &JsonObject) -> Result<Vec<Feature>, Error> {
    let mut features = vec![];
    let features_json =
        expect_array!(expect_property!(object, "features", "Missing 'features' field"));
    for feature in features_json {
        let feature = expect_object!(feature);
        let feature: Feature = try!(Feature::from_object(feature));
        features.push(feature);
    }
    return Ok(features);
}


fn json_to_position(json: &JsonValue) -> Result<Position, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for position in coords_array {
        coords.push(expect_f64!(position));
    }
    return Ok(coords);
}

fn json_to_1d_positions(json: &JsonValue) -> Result<Vec<Position>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_position(item)));
    }
    return Ok(coords);
}

fn json_to_2d_positions(json: &JsonValue) -> Result<Vec<Vec<Position>>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_1d_positions(item)));
    }
    return Ok(coords);
}

fn json_to_3d_positions(json: &JsonValue) -> Result<Vec<Vec<Vec<Position>>>, Error> {
    let coords_array = expect_array!(json);
    let mut coords = vec![];
    for item in coords_array {
        coords.push(try!(json_to_2d_positions(item)));
    }
    return Ok(coords);
}
