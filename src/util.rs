// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::errors::{Error, Result};
use crate::{feature, Bbox, Feature, Geometry, Position, Value};
use crate::{JsonObject, JsonValue};

pub fn expect_type(value: &mut JsonObject) -> Result<String> {
    let prop = expect_property(value, "type")?;
    expect_string(prop)
}

pub fn expect_string(value: JsonValue) -> Result<String> {
    match value {
        JsonValue::String(s) => Ok(s),
        _ => Err(Error::ExpectedStringValue(value)),
    }
}

pub fn expect_f64(value: &JsonValue) -> Result<f64> {
    match value.as_f64() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedF64Value),
    }
}

pub fn expect_array(value: &JsonValue) -> Result<&Vec<JsonValue>> {
    match value.as_array() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedArrayValue("None".to_string())),
    }
}

fn expect_property(obj: &mut JsonObject, name: &'static str) -> Result<JsonValue> {
    match obj.remove(name) {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedProperty(name.to_string())),
    }
}

fn expect_owned_array(value: JsonValue) -> Result<Vec<JsonValue>> {
    match value {
        JsonValue::Array(v) => Ok(v),
        _ => match value {
            // it can never be Array, but that's exhaustive matches for you
            JsonValue::Array(_) => Err(Error::ExpectedArrayValue("Array".to_string())),
            JsonValue::Null => Err(Error::ExpectedArrayValue("Null".to_string())),
            JsonValue::Bool(_) => Err(Error::ExpectedArrayValue("Bool".to_string())),
            JsonValue::Number(_) => Err(Error::ExpectedArrayValue("Number".to_string())),
            JsonValue::String(_) => Err(Error::ExpectedArrayValue("String".to_string())),
            JsonValue::Object(_) => Err(Error::ExpectedArrayValue("Object".to_string())),
        },
    }
}

pub(crate) fn expect_owned_object(value: JsonValue) -> Result<JsonObject> {
    match value {
        JsonValue::Object(o) => Ok(o),
        _ => Err(Error::ExpectedObjectValue(value)),
    }
}

pub fn get_coords_value(object: &mut JsonObject) -> Result<JsonValue> {
    expect_property(object, "coordinates")
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_bbox(object: &mut JsonObject) -> Result<Option<Bbox>> {
    let bbox_json = match object.remove("bbox") {
        Some(b) => b,
        None => return Ok(None),
    };
    let bbox_array = match bbox_json {
        JsonValue::Array(a) => a,
        _ => return Err(Error::BboxExpectedArray(bbox_json)),
    };
    let bbox = bbox_array
        .into_iter()
        .map(|i| i.as_f64().ok_or(Error::BboxExpectedNumericValues(i)))
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(bbox))
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_foreign_members(object: JsonObject) -> Result<Option<JsonObject>> {
    if object.is_empty() {
        Ok(None)
    } else {
        Ok(Some(object))
    }
}

/// Used by Feature
pub fn get_properties(object: &mut JsonObject) -> Result<Option<JsonObject>> {
    let properties = expect_property(object, "properties");
    match properties {
        Ok(JsonValue::Object(x)) => Ok(Some(x)),
        Ok(JsonValue::Null) | Err(Error::ExpectedProperty(_)) => Ok(None),
        Ok(not_a_dictionary) => Err(Error::PropertiesExpectedObjectOrNull(not_a_dictionary)),
        Err(e) => Err(e),
    }
}

/// Retrieve a single Position from the value of the "coordinates" key
///
/// Used by Value::Point
pub fn get_coords_one_pos(object: &mut JsonObject) -> Result<Position> {
    let coords_json = get_coords_value(object)?;
    json_to_position(&coords_json)
}

/// Retrieve a one dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPoint and Value::LineString
pub fn get_coords_1d_pos(object: &mut JsonObject) -> Result<Vec<Position>> {
    let coords_json = get_coords_value(object)?;
    json_to_1d_positions(&coords_json)
}

/// Retrieve a two dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiLineString and Value::Polygon
pub fn get_coords_2d_pos(object: &mut JsonObject) -> Result<Vec<Vec<Position>>> {
    let coords_json = get_coords_value(object)?;
    json_to_2d_positions(&coords_json)
}

/// Retrieve a three dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPolygon
pub fn get_coords_3d_pos(object: &mut JsonObject) -> Result<Vec<Vec<Vec<Position>>>> {
    let coords_json = get_coords_value(object)?;
    json_to_3d_positions(&coords_json)
}

/// Used by Value::GeometryCollection
pub fn get_geometries(object: &mut JsonObject) -> Result<Vec<Geometry>> {
    let geometries_json = expect_property(object, "geometries")?;
    let geometries_array = expect_owned_array(geometries_json)?;
    let mut geometries = Vec::with_capacity(geometries_array.len());
    for json in geometries_array {
        let obj = expect_owned_object(json)?;
        let geometry = Geometry::from_json_object(obj)?;
        geometries.push(geometry);
    }
    Ok(geometries)
}

/// Used by Feature
pub fn get_id(object: &mut JsonObject) -> Result<Option<feature::Id>> {
    match object.remove("id") {
        Some(JsonValue::Number(x)) => Ok(Some(feature::Id::Number(x))),
        Some(JsonValue::String(s)) => Ok(Some(feature::Id::String(s))),
        Some(v) => Err(Error::FeatureInvalidIdentifierType(v)),
        None => Ok(None),
    }
}

/// Used by Geometry, Value
pub fn get_value(object: &mut JsonObject) -> Result<Value> {
    let res = &*expect_type(object)?;
    match res {
        "Point" => Ok(Value::Point(get_coords_one_pos(object)?)),
        "MultiPoint" => Ok(Value::MultiPoint(get_coords_1d_pos(object)?)),
        "LineString" => Ok(Value::LineString(get_coords_1d_pos(object)?)),
        "MultiLineString" => Ok(Value::MultiLineString(get_coords_2d_pos(object)?)),
        "Polygon" => Ok(Value::Polygon(get_coords_2d_pos(object)?)),
        "MultiPolygon" => Ok(Value::MultiPolygon(get_coords_3d_pos(object)?)),
        "GeometryCollection" => Ok(Value::GeometryCollection(get_geometries(object)?)),
        _ => Err(Error::GeometryUnknownType(res.to_string())),
    }
}

/// Used by Feature
pub fn get_geometry(object: &mut JsonObject) -> Result<Option<Geometry>> {
    let geometry = expect_property(object, "geometry")?;
    match geometry {
        JsonValue::Object(x) => {
            let geometry_object = Geometry::from_json_object(x)?;
            Ok(Some(geometry_object))
        }
        JsonValue::Null => Ok(None),
        _ => Err(Error::FeatureInvalidGeometryValue(geometry)),
    }
}

/// Used by FeatureCollection
pub fn get_features(object: &mut JsonObject) -> Result<Vec<Feature>> {
    let prop = expect_property(object, "features")?;
    let features_json = expect_owned_array(prop)?;
    let mut features = Vec::with_capacity(features_json.len());
    for feature in features_json {
        let feature = expect_owned_object(feature)?;
        let feature: Feature = Feature::from_json_object(feature)?;
        features.push(feature);
    }
    Ok(features)
}
use tinyvec::TinyVec;
fn json_to_position(json: &JsonValue) -> Result<Position> {
    let coords_array = expect_array(json)?;
    if coords_array.len() < 2 {
        return Err(Error::PositionTooShort(coords_array.len()));
    }
    let mut coords = TinyVec::with_capacity(coords_array.len());
    for position in coords_array {
        coords.push(expect_f64(position)?);
    }
    Ok(coords)
}

fn json_to_1d_positions(json: &JsonValue) -> Result<Vec<Position>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_position(item)?);
    }
    Ok(coords)
}

fn json_to_2d_positions(json: &JsonValue) -> Result<Vec<Vec<Position>>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_1d_positions(item)?);
    }
    Ok(coords)
}

fn json_to_3d_positions(json: &JsonValue) -> Result<Vec<Vec<Vec<Position>>>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_2d_positions(item)?);
    }
    Ok(coords)
}
