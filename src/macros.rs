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

macro_rules! expect_type {
    ($value:expr) => (
        expect_string!(expect_property!($value, "type", "Missing 'type' field"))
    )
}

macro_rules! expect_string {
    ($value:expr) => (try!(
        match $value.as_string() {
            Some(v) => Ok(v),
            None => Err({use Error; Error::new("Expected string value")})
        }
    ))
}

macro_rules! expect_f64 {
    ($value:expr) => (try!(
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err({use Error; Error::new("Expected f64 value")})
        }
    ))
}

macro_rules! expect_array {
    ($value:expr) => (try!(
        match $value.as_array() {
            Some(v) => Ok(v),
            None => Err({use Error; Error::new("Expected array value")})
        }
    ))
}

macro_rules! expect_object {
    ($value:expr) => (try!(
        match $value.as_object() {
            Some(v) => Ok(v),
            None => Err({use Error; Error::new("Expected object value")})
        }
    ))
}

macro_rules! expect_property {
    ($obj:expr, $name:expr, $desc:expr) => (
        match $obj.get($name) {
            Some(v) => v,
            None => return Err({use Error; Error::new($desc)}),
        };
    )
}
