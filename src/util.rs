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

use json::{JsonObject, JsonValue};

use {Bbox, Error, Feature, FromObject, Geometry, Position};

pub fn expect_type(value: &mut JsonObject) -> Result<String, Error> {
    let prop = expect_property(value, "type")?;
    expect_string(prop)
}

pub fn expect_string(value: JsonValue) -> Result<String, Error> {
    match value {
        JsonValue::String(s) => Ok(s),
        _ => Err(Error::ExpectedStringValue),
    }
}

pub fn expect_f64(value: &JsonValue) -> Result<f64, Error> {
    match value.as_f64() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedF64Value),
    }
}

pub fn expect_array(value: &JsonValue) -> Result<&Vec<JsonValue>, Error> {
    match value.as_array() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedArrayValue),
    }
}

pub fn expect_object(value: &JsonValue) -> Result<&JsonObject, Error> {
    match value.as_object() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedObjectValue),
    }
}

fn expect_property(obj: &mut JsonObject, name: &'static str) -> Result<JsonValue, Error> {
    match obj.remove(name) {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedProperty),
    }
}

fn expect_owned_array(value: JsonValue) -> Result<Vec<JsonValue>, Error> {
    match value {
        JsonValue::Array(v) => Ok(v),
        _ => Err(Error::ExpectedArrayValue),
    }
}

fn expect_owned_object(value: JsonValue) -> Result<JsonObject, Error> {
    match value {
        JsonValue::Object(o) => Ok(o),
        _ => Err(Error::ExpectedObjectValue),
    }
}

pub fn get_coords_value<'a>(object: &mut JsonObject) -> Result<JsonValue, Error> {
    expect_property(object, "coordinates")
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_bbox(object: &mut JsonObject) -> Result<Option<Bbox>, Error> {
    let bbox_json = match object.remove("bbox") {
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
    Ok(Some(bbox))
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_foreign_members(object: &JsonObject) -> Result<Option<JsonObject>, Error> {
    if object.is_empty() {
        Ok(None)
    } else {
        let mut res = JsonObject::new();
        for (key, value) in object {
            res.insert(key.to_owned(), value.to_owned());
        }
        Ok(Some(res))
    }
}

/// Used by Feature
pub fn get_properties(object: &mut JsonObject) -> Result<Option<JsonObject>, Error> {
    let properties = expect_property(object, "properties")?;
    return match properties {
        JsonValue::Object(x) => Ok(Some(x)),
        JsonValue::Null => Ok(None),
        _ => return Err(Error::PropertiesExpectedObjectOrNull),
    };
}

/// Retrieve a single Position from the value of the "coordinates" key
///
/// Used by Value::Point
pub fn get_coords_one_pos(object: &mut JsonObject) -> Result<Position, Error> {
    let coords_json = get_coords_value(object)?;
    return json_to_position(&coords_json);
}

/// Retrieve a one dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPoint and Value::LineString
pub fn get_coords_1d_pos(object: &mut JsonObject) -> Result<Vec<Position>, Error> {
    let coords_json = get_coords_value(object)?;
    return json_to_1d_positions(&coords_json);
}

/// Retrieve a two dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiLineString and Value::Polygon
pub fn get_coords_2d_pos(object: &mut JsonObject) -> Result<Vec<Vec<Position>>, Error> {
    let coords_json = get_coords_value(object)?;
    return json_to_2d_positions(&coords_json);
}

/// Retrieve a three dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPolygon
pub fn get_coords_3d_pos(object: &mut JsonObject) -> Result<Vec<Vec<Vec<Position>>>, Error> {
    let coords_json = get_coords_value(object)?;
    return json_to_3d_positions(&coords_json);
}

/// Used by Value::GeometryCollection
pub fn get_geometries(object: &mut JsonObject) -> Result<Vec<Geometry>, Error> {
    let geometries_json = expect_property(object, "geometries")?;
    let geometries_array = expect_array(&geometries_json)?;
    let mut geometries = Vec::with_capacity(geometries_array.len());
    for json in geometries_array {
        let obj = expect_object(json)?;
        let geometry = Geometry::from_object(obj.clone())?;
        geometries.push(geometry);
    }
    return Ok(geometries);
}

/// Used by Feature
pub fn get_id(object: &mut JsonObject) -> Result<Option<JsonValue>, Error> {
    return Ok(object.remove("id"));
}

/// Used by Feature
pub fn get_geometry(object: &mut JsonObject) -> Result<Option<Geometry>, Error> {
    let geometry = expect_property(object, "geometry")?;
    match geometry {
        JsonValue::Object(x) => {
            let geometry_object = Geometry::from_object(x)?;
            Ok(Some(geometry_object))
        }
        JsonValue::Null => Ok(None),
        _ => Err(Error::FeatureInvalidGeometryValue),
    }
}

/// Used by FeatureCollection
pub fn get_features(object: &mut JsonObject) -> Result<Vec<Feature>, Error> {
    let prop = expect_property(object, "features")?;
    let features_json = expect_owned_array(prop)?;
    let mut features = Vec::with_capacity(features_json.len());
    for feature in features_json {
        let feature = expect_owned_object(feature)?;
        let feature: Feature = Feature::from_object(feature)?;
        features.push(feature);
    }
    return Ok(features);
}

fn json_to_position(json: &JsonValue) -> Result<Position, Error> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for position in coords_array {
        coords.push(expect_f64(position)?);
    }
    return Ok(coords);
}

fn json_to_1d_positions(json: &JsonValue) -> Result<Vec<Position>, Error> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_position(item)?);
    }
    return Ok(coords);
}

fn json_to_2d_positions(json: &JsonValue) -> Result<Vec<Vec<Position>>, Error> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_1d_positions(item)?);
    }
    return Ok(coords);
}

fn json_to_3d_positions(json: &JsonValue) -> Result<Vec<Vec<Vec<Position>>>, Error> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_2d_positions(item)?);
    }
    return Ok(coords);
}
