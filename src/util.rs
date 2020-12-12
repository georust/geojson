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

use crate::errors::Error;
use crate::json::{JsonObject, JsonValue};
use crate::{feature, Bbox, Feature, Geometry, Position, Value};

pub fn expect_type<Pos: Position>(value: &mut JsonObject) -> Result<String, Error<Pos>> {
    let prop = expect_property(value, "type")?;
    expect_string(prop)
}

pub fn expect_string<Pos: Position>(value: JsonValue) -> Result<String, Error<Pos>> {
    match value {
        JsonValue::String(s) => Ok(s),
        _ => Err(Error::ExpectedStringValue(value)),
    }
}

pub fn expect_f64<Pos: Position>(value: &JsonValue) -> Result<f64, Error<Pos>> {
    match value.as_f64() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedF64Value),
    }
}

pub fn expect_array<Pos: Position>(value: &JsonValue) -> Result<&Vec<JsonValue>, Error<Pos>> {
    match value.as_array() {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedArrayValue("None".to_string())),
    }
}

fn expect_property<Pos: Position>(
    obj: &mut JsonObject,
    name: &'static str,
) -> Result<JsonValue, Error<Pos>> {
    match obj.remove(name) {
        Some(v) => Ok(v),
        None => Err(Error::ExpectedProperty(name.to_string())),
    }
}

fn expect_owned_array<Pos: Position>(value: JsonValue) -> Result<Vec<JsonValue>, Error<Pos>> {
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

fn expect_owned_object<Pos: Position>(value: JsonValue) -> Result<JsonObject, Error<Pos>> {
    match value {
        JsonValue::Object(o) => Ok(o),
        _ => Err(Error::ExpectedObjectValue(value)),
    }
}

pub fn get_coords_value<Pos: Position>(object: &mut JsonObject) -> Result<JsonValue, Error<Pos>> {
    expect_property(object, "coordinates")
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_bbox<Pos: Position>(object: &mut JsonObject) -> Result<Option<Bbox>, Error<Pos>> {
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
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(bbox))
}

/// Used by FeatureCollection, Feature, Geometry
pub fn get_foreign_members<Pos: Position>(
    object: JsonObject,
) -> Result<Option<JsonObject>, Error<Pos>> {
    if object.is_empty() {
        Ok(None)
    } else {
        Ok(Some(object))
    }
}

/// Used by Feature
pub fn get_properties<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Option<JsonObject>, Error<Pos>> {
    let properties = expect_property(object, "properties")?;
    match properties {
        JsonValue::Object(x) => Ok(Some(x)),
        JsonValue::Null => Ok(None),
        _ => Err(Error::PropertiesExpectedObjectOrNull(properties)),
    }
}

/// Retrieve a single Position from the value of the "coordinates" key
///
/// Used by Value::Point
pub fn get_coords_one_pos<Pos: Position>(object: &mut JsonObject) -> Result<Pos, Error<Pos>> {
    let coords_json = get_coords_value(object)?;
    Position::from_json_value(&coords_json)
}

/// Retrieve a one dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPoint and Value::LineString
pub fn get_coords_1d_pos<Pos: Position>(object: &mut JsonObject) -> Result<Vec<Pos>, Error<Pos>> {
    let coords_json = get_coords_value(object)?;
    json_to_1d_positions(&coords_json)
}

/// Retrieve a two dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiLineString and Value::Polygon
pub fn get_coords_2d_pos<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Vec<Vec<Pos>>, Error<Pos>> {
    let coords_json = get_coords_value(object)?;
    json_to_2d_positions(&coords_json)
}

/// Retrieve a three dimensional Vec of Positions from the value of the "coordinates" key
///
/// Used by Value::MultiPolygon
pub fn get_coords_3d_pos<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Vec<Vec<Vec<Pos>>>, Error<Pos>> {
    let coords_json = get_coords_value(object)?;
    json_to_3d_positions(&coords_json)
}

/// Used by Value::GeometryCollection
pub fn get_geometries<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Vec<Geometry<Pos>>, Error<Pos>> {
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
pub fn get_id<Pos: Position>(object: &mut JsonObject) -> Result<Option<feature::Id>, Error<Pos>> {
    match object.remove("id") {
        Some(JsonValue::Number(x)) => Ok(Some(feature::Id::Number(x))),
        Some(JsonValue::String(s)) => Ok(Some(feature::Id::String(s))),
        Some(v) => Err(Error::FeatureInvalidIdentifierType(v)),
        None => Ok(None),
    }
}

/// Used by Geometry, Value
pub fn get_value<Pos: Position>(object: &mut JsonObject) -> Result<Value<Pos>, Error<Pos>> {
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
pub fn get_geometry<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Option<Geometry<Pos>>, Error<Pos>> {
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
pub fn get_features<Pos: Position>(
    object: &mut JsonObject,
) -> Result<Vec<Feature<Pos>>, Error<Pos>> {
    let prop = expect_property(object, "features")?;
    let features_json = expect_owned_array(prop)?;
    let mut features = Vec::with_capacity(features_json.len());
    for feature in features_json {
        let feature = expect_owned_object(feature)?;
        let feature: Feature<Pos> = Feature::from_json_object(feature)?;
        features.push(feature);
    }
    Ok(features)
}

fn json_to_1d_positions<Pos: Position>(json: &JsonValue) -> Result<Vec<Pos>, Error<Pos>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(Pos::from_json_value(item)?);
    }
    Ok(coords)
}

fn json_to_2d_positions<Pos: Position>(json: &JsonValue) -> Result<Vec<Vec<Pos>>, Error<Pos>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_1d_positions(item)?);
    }
    Ok(coords)
}

fn json_to_3d_positions<Pos: Position>(json: &JsonValue) -> Result<Vec<Vec<Vec<Pos>>>, Error<Pos>> {
    let coords_array = expect_array(json)?;
    let mut coords = Vec::with_capacity(coords_array.len());
    for item in coords_array {
        coords.push(json_to_2d_positions(item)?);
    }
    Ok(coords)
}
