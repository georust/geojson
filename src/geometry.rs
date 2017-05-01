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

use json::{Serialize, Deserialize, Serializer, Deserializer, JsonValue, JsonObject};

use {Bbox, Crs, Error, LineStringType, PointType, PolygonType, FromObject, util};


/// The underlying Geometry value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Point
    ///
    /// [GeoJSON Format Specification § 2.1.2]
    /// (http://geojson.org/geojson-spec.html#point)
    Point(PointType),

    /// MultiPoint
    ///
    /// [GeoJSON Format Specification § 2.1.3]
    /// (http://geojson.org/geojson-spec.html#multipoint)
    MultiPoint(Vec<PointType>),

    /// LineString
    ///
    /// [GeoJSON Format Specification § 2.1.4]
    /// (http://geojson.org/geojson-spec.html#linestring)
    LineString(LineStringType),

    /// MultiLineString
    ///
    /// [GeoJSON Format Specification § 2.1.5]
    /// (http://geojson.org/geojson-spec.html#multilinestring)
    MultiLineString(Vec<LineStringType>),

    /// Polygon
    ///
    /// [GeoJSON Format Specification § 2.1.6]
    /// (http://geojson.org/geojson-spec.html#polygon)
    Polygon(PolygonType),

    /// MultiPolygon
    ///
    /// [GeoJSON Format Specification § 2.1.7]
    /// (http://geojson.org/geojson-spec.html#multipolygon)
    MultiPolygon(Vec<PolygonType>),

    /// GeometryCollection
    ///
    /// [GeoJSON Format Specification § 2.1.8]
    /// (http://geojson.org/geojson-spec.html#geometry-collection)
    GeometryCollection(Vec<Geometry>),
}

impl<'a> From<&'a Value> for JsonValue {
    fn from(value: &'a Value) -> JsonValue {
        match *value {
                Value::Point(ref x) => ::serde_json::to_value(x),
                Value::MultiPoint(ref x) => ::serde_json::to_value(x),
                Value::LineString(ref x) => ::serde_json::to_value(x),
                Value::MultiLineString(ref x) => ::serde_json::to_value(x),
                Value::Polygon(ref x) => ::serde_json::to_value(x),
                Value::MultiPolygon(ref x) => ::serde_json::to_value(x),
                Value::GeometryCollection(ref x) => ::serde_json::to_value(x),
            }
            .unwrap()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        JsonValue::from(self).serialize(serializer)
    }
}

/// Geometry Objects
///
/// [GeoJSON Format Specification § 2.1]
/// (http://geojson.org/geojson-spec.html#geometry-objects)
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    pub bbox: Option<Bbox>,
    pub value: Value,
    pub crs: Option<Crs>,
    pub foreign_members: Option<JsonObject>
}

impl Geometry {
    /// Returns a new `Geometry` with the specified `value`. `bbox` and `crs` will be set to
    /// `None`.
    pub fn new(value: Value) -> Self {
        Geometry {
            bbox: None,
            value: value,
            crs: None,
            foreign_members: None
        }
    }
}

impl<'a> From<&'a Geometry> for JsonObject {
    fn from(geometry: &'a Geometry) -> JsonObject {
        let mut map = JsonObject::new();
        if let Some(ref crs) = geometry.crs {
            map.insert(String::from("crs"), ::serde_json::to_value(crs).unwrap());
        }
        if let Some(ref bbox) = geometry.bbox {
            map.insert(String::from("bbox"), ::serde_json::to_value(bbox).unwrap());
        }

        let ty = String::from(match geometry.value {
            Value::Point(..) => "Point",
            Value::MultiPoint(..) => "MultiPoint",
            Value::LineString(..) => "LineString",
            Value::MultiLineString(..) => "MultiLineString",
            Value::Polygon(..) => "Polygon",
            Value::MultiPolygon(..) => "MultiPolygon",
            Value::GeometryCollection(..) => "GeometryCollection",
        });

        map.insert(String::from("type"), ::serde_json::to_value(&ty).unwrap());

        map.insert(String::from(match geometry.value {
                       Value::GeometryCollection(..) => "geometries",
                       _ => "coordinates",
                   }),
                   ::serde_json::to_value(&geometry.value).unwrap());
        if let Some(ref foreign_members) = geometry.foreign_members {
            for key in foreign_members.keys() {
                map.insert(key.to_string(), foreign_members.get(key.as_str()).unwrap().clone());
            }
        }
        return map;
    }
}

impl FromObject for Geometry {
    fn from_object(object: &JsonObject) -> Result<Self, Error> {
        let type_ = expect_type!(object);
        let value = match type_ {
            "Point" => Value::Point(try!(util::get_coords_one_pos(object))),
            "MultiPoint" => Value::MultiPoint(try!(util::get_coords_1d_pos(object))),
            "LineString" => Value::LineString(try!(util::get_coords_1d_pos(object))),
            "MultiLineString" => Value::MultiLineString(try!(util::get_coords_2d_pos(object))),
            "Polygon" => Value::Polygon(try!(util::get_coords_2d_pos(object))),
            "MultiPolygon" => Value::MultiPolygon(try!(util::get_coords_3d_pos(object))),
            "GeometryCollection" => Value::GeometryCollection(try!(util::get_geometries(object))),
            _ => return Err(Error::GeometryUnknownType),
        };

        let bbox = try!(util::get_bbox(object));
        let crs = try!(util::get_crs(object));
        let foreign_members = try!(util::get_foreign_members(object, "Geometry"));

        return Ok(Geometry {
            bbox: bbox,
            value: value,
            crs: crs,
            foreign_members: foreign_members
        });
    }
}

impl Serialize for Geometry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Geometry {
    fn deserialize<D>(deserializer: D) -> Result<Geometry, D::Error>
        where D: Deserializer<'de>
    {
        use std::error::Error as StdError;
        use serde::de::Error as SerdeError;

        let val = try!(JsonObject::deserialize(deserializer));

        Geometry::from_object(&val).map_err(|e| D::Error::custom(e.description()))
    }
}


#[cfg(test)]
mod tests {
    use {GeoJson, Geometry, Value};
    use serde_json;
    use json::JsonObject;

    fn encode(geometry: &Geometry) -> String {
        use serde_json;

        serde_json::to_string(&geometry).unwrap()
    }

    fn decode(json_string: String) -> GeoJson {
        json_string.parse().unwrap()
    }

    #[test]
    fn encode_decode_geometry() {
        let geometry_json_str = "{\"coordinates\":[1.1,2.1],\"type\":\"Point\"}";
        let geometry = Geometry {
            value: Value::Point(vec![1.1, 2.1]),
            crs: None,
            bbox: None,
            foreign_members: None
        };

        // Test encode
        let json_string = encode(&geometry);
        assert_eq!(json_string, geometry_json_str);

        // Test decode
        let decoded_geometry = match decode(json_string) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry);
    }

    #[test]
    fn encode_decode_geometry_and_foreign_member() {
        let geometry_json_str = "{\"coordinates\":[1.1,2.1],\"other_member\":\"some_value\",\"type\":\"Point\"}";
        let mut foreign_members = JsonObject::new();
        foreign_members.insert(String::from("other_member"), serde_json::to_value("some_value").unwrap());
        let geometry = Geometry {
            value: Value::Point(vec![1.1, 2.1]),
            crs: None,
            bbox: None,
            foreign_members: Some(foreign_members)
        };

        // Test encode
        let json_string = encode(&geometry);
        assert_eq!(json_string, geometry_json_str);

        // Test decode
        let decoded_geometry = match decode(geometry_json_str.into()) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry);
    }

    #[test]
    fn encode_decode_geometry_collection() {
        let geometry_collection = Geometry {
                bbox: None,
                value: Value::GeometryCollection(vec![
                    Geometry { bbox: None, value: Value::Point(vec![100.0, 0.0]), crs: None, foreign_members: None },
                    Geometry { bbox: None, value: Value::LineString(vec![vec![101.0, 0.0], vec![102.0, 1.0]]),
                crs: None, foreign_members: None }]),
            crs: None, foreign_members: None };

        let geometry_collection_string = "{\"geometries\":[{\"coordinates\":[100.0,0.0],\"type\":\"Point\"},{\"coordinates\":[[101.0,0.0],[102.0,1.0]],\"type\":\"LineString\"}],\"type\":\"GeometryCollection\"}";
        // Test encode
        let json_string = encode(&geometry_collection);
        assert_eq!(json_string, geometry_collection_string);

        // Test decode
        let decoded_geometry = match decode(geometry_collection_string.into()) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry_collection);
    }
}
