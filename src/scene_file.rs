// scene_file.rs
// © 2024 Epic Mickey Library

use std::{io::{Read, Write}, str::FromStr};
use serde_json;
use crate::file_manipulator;

#[derive(Clone)]
pub enum SceneFileVersion {
    Version1 = 1,
    Version2Prototype,
    Version2
}
impl SceneFileVersion {
    pub fn from_u32(version: u32) -> SceneFileVersion {
        match version {
            1 => SceneFileVersion::Version1,
            2 => SceneFileVersion::Version2Prototype,
            3 => SceneFileVersion::Version2,
            _ => panic!("Unknown scene file version: {}", version)
        }
    }
}

pub struct Point2 {
    pub x: f32,
    pub y: f32
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Point2 {
        Point2 {
            x,
            y
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.x = fm.r_float();
        self.y = fm.r_float();
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_float(self.x);
        fm.w_float(self.y);
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("x".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.x as f64).unwrap()));
        dict.insert("y".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.y as f64).unwrap()));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Point2 {
        let x = dict["x"].as_f64().unwrap() as f32;
        let y = dict["y"].as_f64().unwrap() as f32;
        return Point2::new(x, y)
    }
}

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 {
            x,
            y,
            z
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.x = fm.r_float();
        self.y = fm.r_float();
        self.z = fm.r_float();
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_float(self.x);
        fm.w_float(self.y);
        fm.w_float(self.z);
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("x".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.x as f64).unwrap()));
        dict.insert("y".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.y as f64).unwrap()));
        dict.insert("z".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.z as f64).unwrap()));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Point3 {
        let x = dict["x"].as_f64().unwrap() as f32;
        let y = dict["y"].as_f64().unwrap() as f32;
        let z = dict["z"].as_f64().unwrap() as f32;
        return Point3::new(x, y, z)
    }
}

pub struct Matrix3 {
    pub m: [[f32; 3]; 3]
}

impl Matrix3 {
    pub fn new(m: [[f32; 3]; 3]) -> Matrix3 {
        Matrix3 {
            m
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        for i in 0..3 {
            for j in 0..3 {
                self.m[i][j] = fm.r_float();
            }
        }
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        for i in 0..3 {
            for j in 0..3 {
                fm.w_float(self.m[i][j]);
            }
        }
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        let mut m = Vec::new();
        for i in 0..3 {
            let mut row = Vec::new();
            for j in 0..3 {
                row.push(serde_json::Value::Number(serde_json::Number::from_f64(self.m[i][j] as f64).unwrap()));
            }
            m.push(serde_json::Value::Array(row));
        }
        dict.insert("m".to_string(), serde_json::Value::Array(m));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Matrix3 {
        let mut m = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] = dict["m"][i][j].as_f64().unwrap() as f32;
            }
        }
        return Matrix3::new(m)
    }

    pub fn identity() -> Matrix3 {
        let m = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ];
        return Matrix3::new(m)
    }
}

pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorRGB {
    pub fn new(r: f32, g: f32, b: f32) -> ColorRGB {
        ColorRGB {
            r,
            g,
            b
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.r = fm.r_float();
        self.g = fm.r_float();
        self.b = fm.r_float();
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_float(self.r);
        fm.w_float(self.g);
        fm.w_float(self.b);
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("r".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.r as f64).unwrap()));
        dict.insert("g".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.g as f64).unwrap()));
        dict.insert("b".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.b as f64).unwrap()));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> ColorRGB {
        let r = dict["r"].as_f64().unwrap() as f32;
        let g = dict["g"].as_f64().unwrap() as f32;
        let b = dict["b"].as_f64().unwrap() as f32;
        return ColorRGB::new(r, g, b)
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        return (r << 16) | (g << 8) | b
    }

    pub fn from_u32(hex: u32) -> ColorRGB {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        return ColorRGB::new(r, g, b)
    }

    pub fn to_hex_string(&self) -> String {
        let hex = self.to_u32();
        return format!("{:06X}", hex)
    }

    pub fn from_hex_string(hex: &str) -> ColorRGB {
        let hex = u32::from_str_radix(hex, 16).unwrap();
        return ColorRGB::from_u32(hex)
    }
}

pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl ColorRGBA {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> ColorRGBA {
        ColorRGBA {
            r,
            g,
            b,
            a
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.r = fm.r_float();
        self.g = fm.r_float();
        self.b = fm.r_float();
        self.a = fm.r_float();
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_float(self.r);
        fm.w_float(self.g);
        fm.w_float(self.b);
        fm.w_float(self.a);
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("r".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.r as f64).unwrap()));
        dict.insert("g".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.g as f64).unwrap()));
        dict.insert("b".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.b as f64).unwrap()));
        dict.insert("a".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.a as f64).unwrap()));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> ColorRGBA {
        let r = dict["r"].as_f64().unwrap() as f32;
        let g = dict["g"].as_f64().unwrap() as f32;
        let b = dict["b"].as_f64().unwrap() as f32;
        let a = dict["a"].as_f64().unwrap() as f32;
        return ColorRGBA::new(r, g, b, a)
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;
        return (r << 24) | (g << 16) | (b << 8) | a
    }

    pub fn from_u32(hex: u32) -> ColorRGBA {
        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;
        return ColorRGBA::new(r, g, b, a)
    }

    pub fn to_hex_string(&self) -> String {
        let hex = self.to_u32();
        return format!("{:08X}", hex)
    }

    pub fn from_hex_string(hex: &str) -> ColorRGBA {
        let hex = u32::from_str_radix(hex, 16).unwrap();
        return ColorRGBA::from_u32(hex)
    }

    pub fn to_rgb(&self) -> ColorRGB {
        return ColorRGB::new(self.r, self.g, self.b)
    }

    pub fn from_rgb(color: &ColorRGB, a: f32) -> ColorRGBA {
        return ColorRGBA::new(color.r, color.g, color.b, a)
    }
}

pub struct ID {
    pub id: u128
}

impl ID {
    pub fn new(id: u128) -> ID {
        ID {
            id
        }
    }

    pub fn to_string(&self, num_bytes: u8) -> String {
        // convert to hex string with commas every 2 characters
        let mut hex = format!("{:X}", self.id);
        while hex.len() < num_bytes as usize * 2 {
            hex = format!("0{}", hex);
        }
        let mut result = String::new();
        for i in 0..num_bytes as usize {
            if i > 0 {
                result.push_str(",");
            }
            result.push_str(&hex[i*2..i*2+2]);
        }
        return result.to_lowercase()
    }

    pub fn to_string_no_leaders(&self, num_bytes: u8) -> String {
        let string = self.to_string(num_bytes);
        // for part, if the first character is 0, remove it
        let mut parts = Vec::new();
        for part in string.split(",") {
            let part = part.to_string();
            if part.chars().nth(0).unwrap() == '0' {
                parts.push(part.chars().skip(1).collect());
            } else {
                parts.push(part);
            }
        }
        return parts.join(",")
    }

    pub fn from_string(string: &str) -> ID {
        let mut hex = String::new();
        for part in string.split(",") {
            // until its 2 characters long, add 0 in front
            let mut part = part.to_string();
            while part.len() < 2 {
                part = format!("0{}", part);
            }
            hex.push_str(&part);
        }
        let id = u128::from_str_radix(&hex, 16).unwrap();
        return ID::new(id)
    }

    pub fn from_u32(id: u32) -> ID {
        return ID::new(id as u128)
    }

    pub fn to_u32(&self) -> u32 {
        // check if it fits in u32
        if self.id > u32::MAX as u128 {
            panic!("The ID is too large to fit into a 32 bit integer, you have too many objects in your scene!");
        }
        return self.id as u32
    }

    pub fn from_u128(id: u128) -> ID {
        return ID::new(id)
    }

    pub fn to_u128(&self) -> u128 {
        return self.id
    }
}

struct EntityPointer {
    pub id: ID
}

impl EntityPointer {
    pub fn new(id: ID) -> EntityPointer {
        EntityPointer {
            id
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.id = ID::from_u32(fm.r_u32());
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_u32(self.id.to_u32());
        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        return serde_json::Value::Number(serde_json::Number::from(self.id.to_u32() as i64));
    }

    pub fn from_dict(dict: &serde_json::Value) -> EntityPointer {
        let id = ID::from_u32(dict.as_u64().unwrap() as u32);
        return EntityPointer::new(id)
    }
}

// Property value types
// "Boolean" - bool
// "Integer" - s32
// "Unsigned Integer" - u32
// "Short" - s16
// "Unsigned Short" - u16
// "Float" - f32
// "String" - String
// "Point2" - Point2
// "Point3" - Point3
// "Matrix3" - Matrix3
// "Color (RGB)" - ColorRGB
// "Color (RGBA)" - ColorRGBA
// "Entity Pointer" - EntityPointer

pub struct Property {
    pub class_name: String,
    pub name: String,
    pub asset: bool,
    pub palette: bool,
    pub template: bool,
    pub value: serde_json::Value
}

impl Property {
    pub fn new(class_name: String, name: String, asset: bool, palette: bool, template: bool, value: serde_json::Value) -> Property {
        Property {
            class_name,
            name,
            asset,
            palette,
            template,
            value
        }
    }

    fn read_value_for_type(fm: &mut file_manipulator::FileManipulator, value_type: &str) -> serde_json::Value {
        match value_type {
            "Boolean" => {
                return serde_json::Value::Bool(fm.r_bool());
            },
            "Integer" => {
                return serde_json::Value::Number(serde_json::Number::from(fm.r_s32()));
            },
            "Unsigned Integer" => {
                return serde_json::Value::Number(serde_json::Number::from(fm.r_u32()));
            },
            "Short" => {
                return serde_json::Value::Number(serde_json::Number::from(fm.r_s16_jps()));
            },
            "Unsigned Short" => {
                return serde_json::Value::Number(serde_json::Number::from(fm.r_u16_jps()));
            },
            "Float" => {
                return serde_json::Value::Number(serde_json::Number::from_f64(fm.r_float() as f64).unwrap());
            },
            "String" => {
                // read pointer
                let pointer = fm.r_u32();
                let pos = fm.tell();
                // seek to pointer
                fm.seek(pointer as usize);
                // read string
                let string = fm.r_str_jps();
                // seek back
                fm.seek(pos);
                return serde_json::Value::String(string);
            },
            "Point2" => {
                let mut point = Point2::new(0.0, 0.0);
                point.unpack(fm);
                return point.to_dict();
            },
            "Point3" => {
                let mut point = Point3::new(0.0, 0.0, 0.0);
                point.unpack(fm);
                return point.to_dict();
            },
            "Matrix3" => {
                let mut matrix = Matrix3::new([[0.0; 3]; 3]);
                matrix.unpack(fm);
                return matrix.to_dict();
            },
            "Color (RGB)" => {
                let mut color = ColorRGB::new(0.0, 0.0, 0.0);
                color.unpack(fm);
                return color.to_dict();
            },
            "Color (RGBA)" => {
                let mut color = ColorRGBA::new(0.0, 0.0, 0.0, 0.0);
                color.unpack(fm);
                return color.to_dict();
            },
            "Entity Pointer" => {
                let mut entity_pointer = EntityPointer::new(ID::new(0));
                entity_pointer.unpack(fm);
                return entity_pointer.to_dict();
            },
            _ => {
                panic!("Unknown property value type: {}", value_type);
            }
        }
    }

    fn write_value_for_type(fm: &mut file_manipulator::FileManipulator, strings_offsets_map: &mut std::collections::HashMap<String, u32>, value: &serde_json::Value, value_type: &str) {
        match value_type {
            "Boolean" => {
                fm.w_bool(value.as_bool().unwrap());
            },
            "Integer" => {
                fm.w_s32(value.as_i64().unwrap() as i32);
            },
            "Unsigned Integer" => {
                fm.w_u32(value.as_u64().unwrap() as u32);
            },
            "Short" => {
                fm.w_s16_jps(value.as_i64().unwrap() as i16, 0);
            },
            "Unsigned Short" => {
                fm.w_u16_jps(value.as_u64().unwrap() as u16, 0);
            },
            "Float" => {
                fm.w_float(value.as_f64().unwrap() as f32);
            },
            "String" => {
                let string = value.as_str().unwrap();
                let offset = strings_offsets_map.get(string).unwrap();
                fm.w_u32(*offset);
            },
            "Point2" => {
                let point = Point2::from_dict(value);
                fm.write(&point.pack(fm.get_endian().clone()));
            },
            "Point3" => {
                let point = Point3::from_dict(value);
                fm.write(&point.pack(fm.get_endian().clone()));
            },
            "Matrix3" => {
                let matrix = Matrix3::from_dict(value);
                fm.write(&matrix.pack(fm.get_endian().clone()));
            },
            "Color (RGB)" => {
                let color = ColorRGB::from_dict(value);
                fm.write(&color.pack(fm.get_endian().clone()));
            },
            "Color (RGBA)" => {
                let color = ColorRGBA::from_dict(value);
                fm.write(&color.pack(fm.get_endian().clone()));
            },
            "Entity Pointer" => {
                let entity_pointer = EntityPointer::from_dict(value);
                fm.write(&entity_pointer.pack(fm.get_endian().clone()));
            },
            _ => {
                panic!("Unknown property value type: {}", value_type);
            }
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator, version: SceneFileVersion) {
        let mut name_offset = fm.r_u32();
        let mut class_name_offset = fm.r_u32();
        // if its version 2 proto or version 2
        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                class_name_offset += 4;
                name_offset += 4;
            },
            _ => {}
        }
        let pos = fm.tell();

        fm.seek(class_name_offset as usize);
        self.class_name = fm.r_str_jps();

        fm.seek(name_offset as usize);
        self.name = fm.r_str_jps();

        fm.seek(pos);

        let data_type = fm.r_u32();

        let mut list_mode = false;

        match data_type {
            0 => {
                self.asset = false;
                self.palette = false;
                self.template = false;
            },
            1 => {
                list_mode = true;
                self.asset = false;
                self.palette = false;
                self.template = false;
            },
            2 => {
                self.asset = true;
                self.palette = false;
                self.template = false;
            },
            3 => {
                list_mode = true;
                self.asset = true;
                self.palette = false;
                self.template = false;
            },
            4 => {
                self.palette = true;
                self.asset = false;
                self.template = false;
            },
            5 => {
                list_mode = true;
                self.template = true;
                self.asset = false;
                self.palette = false;
            }
            _ => {
                panic!("Unknown property data storage type: {}", data_type);
            }
        }

        let amount = fm.r_u32();

        self.value = match list_mode {
            true => {
                let mut list = Vec::new();
                for _ in 0..amount {
                    list.push(Property::read_value_for_type(fm, &self.class_name));
                }
                serde_json::Value::Array(list)
            },
            false => {
                Property::read_value_for_type(fm, &self.class_name)
            }
        };
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType, strings_offsets_map: &mut std::collections::HashMap<String, u32>) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);

        // write class name offset
        fm.w_u32(strings_offsets_map[&self.name]);
        fm.w_u32(strings_offsets_map[&self.class_name]);
        

        // write data type
        let list_mode = self.value.is_array();

        let data_type = match (list_mode, self.asset, self.palette, self.template) {
            (false, false, false, false) => 0, // single
            (true, false, false, false) => 1, // list
            (false, true, false, false) => 2, // asset
            (true, true, false, false) => 3, // asset list
            (false, false, true, false) => 4, // palette
            (true, false, false, true) => 5, // template
            _ => panic!("Unknown property data storage type")
        };

        fm.w_u32(data_type);

        // write amount
        let amount = match list_mode {
            true => self.value.as_array().unwrap().len(),
            false => 1
        };
        fm.w_u32(amount as u32);

        let mut values = Vec::new();

        match list_mode {
            true => {
                for value in self.value.as_array().unwrap() {
                    values.push(value);
                }
            },
            false => {
                values.push(&self.value);
            }
        }

        for value in values {
            Property::write_value_for_type(&mut fm, strings_offsets_map, value, &self.class_name);
        }

        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("class_name".to_string(), serde_json::Value::String(self.class_name.clone()));
        dict.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        dict.insert("asset".to_string(), serde_json::Value::Bool(self.asset));
        dict.insert("palette".to_string(), serde_json::Value::Bool(self.palette));
        dict.insert("template".to_string(), serde_json::Value::Bool(self.template));
        dict.insert("value".to_string(), self.value.clone());
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Property {
        let class_name = dict["class_name"].as_str().unwrap().to_string();
        let name = dict["name"].as_str().unwrap().to_string();
        let asset = dict["asset"].as_bool().unwrap();
        let palette = dict["palette"].as_bool().unwrap();
        let template = dict["template"].as_bool().unwrap();
        let value = dict["value"].clone();
        return Property::new(class_name, name, asset, palette, template, value)
    }

    pub fn merge_in_dict(&mut self, dict: &serde_json::Value) {
        // if the value is an array, merge the arrays (add the new values)
        if dict["value"].is_array() {
            for value in dict["value"].as_array().unwrap() {
                self.value.as_array_mut().unwrap().push(value.clone());
            }
        } else {
            self.value = dict["value"].clone();
        }
    }
}

pub struct Component {
    pub class_name: String,
    pub name: String,
    pub template_id: ID,
    pub link_id: ID,
    pub master_link_id: ID,
    pub properties: Vec<Property>
}

impl Component {
    pub fn new(class_name: String, name: String, template_id: ID, link_id: ID, master_link_id: ID, properties: Vec<Property>) -> Component {
        Component {
            class_name,
            name,
            template_id,
            link_id,
            master_link_id,
            properties
        }
    }

    fn get_name_for_class_name(class_name: &str) -> String {
        return match class_name {
            "NiTransformationComponent" => "Transformation".to_string(),
            "JPSTransformationComponent" => "Transformation".to_string(),
            "NiSceneGraphComponent" => "Scene Graph".to_string(),
            "JPSSceneGraphComponent" => "Scene Graph".to_string(),
            "NiLightComponent" => "Light".to_string(),
            "JPSLightComponent" => "Light".to_string(),
            "NiCameraComponent" => "Camera".to_string(),
            "JPSCameraComponent" => "Camera".to_string(),
            _ => "Unknown".to_string()
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator, version: SceneFileVersion) {
        let mut class_name_offset = fm.r_u32();
        let mut template_id_string_offset = fm.r_u32();

        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                class_name_offset += 4;
                template_id_string_offset += 4;
            },
            _ => {}
        }

        let pos = fm.tell();

        fm.seek(class_name_offset as usize);
        self.class_name = fm.r_str_jps();

        fm.seek(template_id_string_offset as usize);
        let template_id_string = fm.r_str_jps();
        self.template_id = ID::from_string(&template_id_string);

        fm.seek(pos);

        self.link_id = ID::from_u32(fm.r_u32());
        self.master_link_id = ID::from_u32(fm.r_u32());

        let amount = fm.r_u32();

        self.properties = Vec::new();

        for _ in 0..amount {
            let mut property = Property::new("".to_string(), "".to_string(), false, false, false, serde_json::Value::Null);
            property.unpack(fm, version.clone()); // Clone the version variable
            self.properties.push(property);
        }

        self.name = Component::get_name_for_class_name(&self.class_name);
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType, strings_offsets_map: &mut std::collections::HashMap<String, u32>) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);

        // write class name offset
        fm.w_u32(strings_offsets_map[&self.class_name]);
        fm.w_u32(strings_offsets_map[&self.template_id.to_string_no_leaders(4)]);

        // write link id
        fm.w_u32(self.link_id.to_u32());
        fm.w_u32(self.master_link_id.to_u32());

        // write amount
        fm.w_u32(self.properties.len() as u32);

        for property in &self.properties {
            fm.write(&property.pack(endian_type.clone(), strings_offsets_map));
        }

        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("class_name".to_string(), serde_json::Value::String(self.class_name.clone()));
        dict.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        dict.insert("template_id".to_string(), serde_json::Value::String(self.template_id.to_string_no_leaders(4)));
        dict.insert("link_id".to_string(), serde_json::Value::Number(serde_json::Number::from(self.link_id.to_u32() as i64)));
        // if master link id is not 0
        if self.master_link_id.to_u32() != 0 {
            dict.insert("master_link_id".to_string(), serde_json::Value::Number(serde_json::Number::from(self.master_link_id.to_u32() as i64)));
        }
        let mut properties = Vec::new();
        for property in &self.properties {
            properties.push(property.to_dict());
        }
        dict.insert("properties".to_string(), serde_json::Value::Array(properties));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Component {
        let class_name = dict["class_name"].as_str().unwrap().to_string();
        // check if name is present
        let name = match dict.get("name") {
            Some(name) => name.as_str().unwrap().to_string(),
            None => Component::get_name_for_class_name(&class_name)
        };
        let template_id = ID::from_string(&dict["template_id"].as_str().unwrap());
        let link_id = ID::from_u32(dict["link_id"].as_u64().unwrap() as u32);
        // check if master link id is present
        let master_link_id = match dict.get("master_link_id") {
            Some(master_link_id) => ID::from_u32(master_link_id.as_u64().unwrap() as u32),
            None => ID::new(0)
        };
        // check if properties are present
        let properties = match dict.get("properties") {
            Some(properties) => {
                let mut properties_vec = Vec::new();
                for property in properties.as_array().unwrap() {
                    properties_vec.push(Property::from_dict(property));
                }
                properties_vec
            },
            None => Vec::new()
        };
        return Component::new(class_name, name, template_id, link_id, master_link_id, properties)
    }

    pub fn merge_in_dict(&mut self, dict: &serde_json::Value) {
        // if the properties are present, merge them
        if dict.get("properties").is_some() {
            for property in dict["properties"].as_array().unwrap() {
                let mut found = false;
                for self_property in &mut self.properties {
                    if self_property.name == property["name"].as_str().unwrap() {
                        self_property.merge_in_dict(property);
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.properties.push(Property::from_dict(property));
                }
            }
        }
    }

    pub fn get_property(&self, name: &str) -> &Property {
        for property in &self.properties {
            if property.name == name {
                return property;
            }
        }
        panic!("Property not found: {}", name);
    }

    pub fn get_property_value(&self, name: &str) -> &serde_json::Value {
        return &self.get_property(name).value;
    }
}

pub struct Entity {
    pub class_name: String,
    pub name: String,
    pub link_id: ID,
    pub master_link_id: ID,
    pub unknown: u32,
    pub unknown_em2: u32,
    pub components: Vec<Component>
}

impl Entity {
    pub fn new(class_name: String, name: String, link_id: ID, master_link_id: ID, unknown: u32, unknown_em2: u32, components: Vec<Component>) -> Entity {
        Entity {
            class_name,
            name,
            link_id,
            master_link_id,
            unknown,
            unknown_em2,
            components
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator, version: SceneFileVersion) {
        self.class_name = "JPSGeneralEntity".to_string();
        let mut name_offset = fm.r_u32();

        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                name_offset += 4;
            },
            _ => {}
        }

        let pos = fm.tell();

        fm.seek(name_offset as usize);
        self.name = fm.r_str_jps();

        fm.seek(pos);

        self.link_id = ID::from_u32(fm.r_u32());
        self.master_link_id = ID::from_u32(fm.r_u32());
        self.unknown = fm.r_u32();

        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                self.unknown_em2 = fm.r_u32();
            },
            _ => {}
        }

        let amount = fm.r_u32();

        self.components = Vec::new();

        for _ in 0..amount {
            let mut component = Component::new("".to_string(), "".to_string(), ID::new(0), ID::new(0), ID::new(0), Vec::new());
            component.unpack(fm, version.clone()); // Clone the version variable
            self.components.push(component);
        }
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType, strings_offsets_map: &mut std::collections::HashMap<String, u32>, version: SceneFileVersion) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);

        // write name offset
        fm.w_u32(strings_offsets_map[&self.name]);

        // write link id
        fm.w_u32(self.link_id.to_u32());
        fm.w_u32(self.master_link_id.to_u32());

        // write unknown
        fm.w_u32(self.unknown);

        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                fm.w_u32(self.unknown_em2);
            },
            _ => {}
        }

        // write amount
        fm.w_u32(self.components.len() as u32);

        for component in &self.components {
            fm.write(&component.pack(endian_type.clone(), strings_offsets_map));
        }

        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self, version: SceneFileVersion) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("class_name".to_string(), serde_json::Value::String(self.class_name.clone()));
        dict.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        dict.insert("link_id".to_string(), serde_json::Value::Number(serde_json::Number::from(self.link_id.to_u32() as i64)));
        // if master link id is not 0
        if self.master_link_id.to_u32() != 0 {
            dict.insert("master_link_id".to_string(), serde_json::Value::Number(serde_json::Number::from(self.master_link_id.to_u32() as i64)));
        }
        if self.unknown != 0 {
            dict.insert("unknown".to_string(), serde_json::Value::Number(serde_json::Number::from(self.unknown as i64)));
        }
        match version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                if self.unknown_em2 != 0 {
                    dict.insert("unknown_em2".to_string(), serde_json::Value::Number(serde_json::Number::from_str(&self.unknown_em2.to_string()).unwrap()));
                }
            },
            _ => {}
        }
        let mut components = Vec::new();
        for component in &self.components {
            components.push(component.to_dict());
        }
        dict.insert("components".to_string(), serde_json::Value::Array(components));
        return serde_json::Value::Object(dict)
    }

    pub fn from_dict(dict: &serde_json::Value) -> Entity {
        let class_name = match dict.get("class_name") {
            Some(class_name) => class_name.as_str().unwrap().to_string(),
            None => "JPSGeneralEntity".to_string()
        };
        let name = dict["name"].as_str().unwrap().to_string();
        let link_id = ID::from_u32(dict["link_id"].as_u64().unwrap() as u32);
        let master_link_id = match dict.get("master_link_id") {
            Some(master_link_id) => ID::from_u32(master_link_id.as_u64().unwrap() as u32),
            None => ID::new(0)
        };
        // check if unknown is present
        let unknown = match dict.get("unknown") {
            Some(unknown) => unknown.as_u64().unwrap() as u32,
            None => 0
        };
        // check if unknown_em2 is present
        let unknown_em2 = match dict.get("unknown_em2") {
            Some(unknown_em2) => unknown_em2.as_u64().unwrap() as u32,
            None => 0
        };
        // check if components are present
        let components = match dict.get("components") {
            Some(components) => {
                let mut components_vec = Vec::new();
                for component in components.as_array().unwrap() {
                    components_vec.push(Component::from_dict(component));
                }
                components_vec
            },
            None => Vec::new()
        };
        return Entity::new(class_name, name, link_id, master_link_id, unknown, unknown_em2, components)
    }

    pub fn merge_in_dict(&mut self, dict: &serde_json::Value) {
        // if the components are present, merge them
        if dict.get("components").is_some() {
            for component in dict["components"].as_array().unwrap() {
                let mut found = false;
                for self_component in &mut self.components {
                    if self_component.class_name == component["class_name"].as_str().unwrap() {
                        self_component.merge_in_dict(component);
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.components.push(Component::from_dict(component));
                }
            }
        }
    }

    pub fn get_component(&self, class_name: &str) -> &Component {
        for component in &self.components {
            if component.class_name == class_name {
                return component;
            }
        }
        panic!("Component not found: {}", class_name);
    }
}

pub struct SceneFile {
    pub objects: Vec<Entity>,
    pub scene: Vec<ID>,
    pub em2_extra_strings: Vec<String>,
    pub unique_id: ID,
    pub version: SceneFileVersion
}

impl SceneFile {
    pub fn new(objects: Vec<Entity>, scene: Vec<ID>, em2_extra_strings: Vec<String>, unique_id: ID, version: SceneFileVersion) -> SceneFile {
        SceneFile {
            objects,
            scene,
            em2_extra_strings,
            unique_id,
            version
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        // if version 2
        match self.version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                fm.move_pos(4);
            },
            _ => {}
        }

        let mut data_offset = fm.r_u32();

        match self.version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                data_offset += 8;
            },
            _ => {}
        }

        fm.seek(data_offset as usize);

        match self.version {
            SceneFileVersion::Version1 | SceneFileVersion::Version2Prototype => {
                self.unique_id = ID::from_u128(fm.r_u128());
            },
            _ => {}
        }

        match self.version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                let em2_extra_strings_amount = fm.r_u32();
                self.em2_extra_strings = Vec::new();
                for _ in 0..em2_extra_strings_amount {
                    self.em2_extra_strings.push(fm.r_str_jps());
                }
            },
            _ => {}
        }

        let entity_amount = fm.r_u32();
        let ref_ids_amount = fm.r_u32();

        self.objects = Vec::new();
        for _ in 0..entity_amount {
            let mut entity = Entity::new("".to_string(), "".to_string(), ID::new(0), ID::new(0), 0, 0, Vec::new());
            entity.unpack(fm, self.version.clone()); // Clone the version variable
            self.objects.push(entity);
        }

        self.scene = Vec::new();
        for _ in 0..ref_ids_amount {
            self.scene.push(ID::from_u32(fm.r_u32()));
        }
    }

    fn add_string(fm: &mut file_manipulator::FileManipulator, strings_offsets_map: &mut std::collections::HashMap<String, u32>, start_offset: u32, string: &str) {
        if !strings_offsets_map.contains_key(string) {
            strings_offsets_map.insert(string.to_string(), fm.get_size() as u32 + start_offset);
            fm.w_str_jps(string);
            
        }
    }

    fn build_strings_and_map(&self) -> (Vec<u8>, std::collections::HashMap<String, u32>) {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), file_manipulator::EndianType::BIG, file_manipulator::WriteMode::OVERWRITE);
        let mut strings_offsets_map = std::collections::HashMap::new();
        let start_offset = 4;
        // for entities
        for entity in &self.objects {
            // add name
            SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &entity.name);
            // for components
            for component in &entity.components {
                // add class name
                SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &component.class_name);
                // add template id
                SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &component.template_id.to_string_no_leaders(4));
                // for properties
                for property in &component.properties {
                    // add name
                    SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &property.name);
                    // add class name
                    SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &property.class_name);
                    // if its a string
                    if property.class_name == "String" {
                        // if its a list
                        if property.value.is_array() {
                            for value in property.value.as_array().unwrap() {
                                SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &value.as_str().unwrap());
                            }
                        } else {
                            SceneFile::add_string(&mut fm, &mut strings_offsets_map, start_offset, &property.value.as_str().unwrap());
                        }
                    }
                }
            }
        }
        // return data and map
        return (fm.get_data().to_vec(), strings_offsets_map);
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let (strings_data, mut strings_offsets_map) = self.build_strings_and_map();
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);

        match self.version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                fm.w_u32(0x01000001);
            },
            _ => {}
        }
        // write data offset (length of strings data + 4)
        fm.w_u32(strings_data.len() as u32 + 4);

        // write the strings section
        fm.write(&strings_data);

        match self.version {
            SceneFileVersion::Version2 => {
                fm.w_u32(0x02000002);
            },
            SceneFileVersion::Version2Prototype => {
                fm.w_u32(0x02000001);
            },
            _ => {}
        }

        match self.version {
            SceneFileVersion::Version1 | SceneFileVersion::Version2Prototype => {
                fm.w_u128(self.unique_id.to_u128());
            },
            _ => {}
        }

        match self.version {
            SceneFileVersion::Version2Prototype | SceneFileVersion::Version2 => {
                fm.w_u32(self.em2_extra_strings.len() as u32);
                for string in &self.em2_extra_strings {
                    fm.w_str_jps(string);
                }
            },
            _ => {}
        }

        // write amount of entities
        fm.w_u32(self.objects.len() as u32);
        
        // write amount of ref ids
        fm.w_u32(self.scene.len() as u32);

        for entity in &self.objects {
            fm.write(&entity.pack(endian_type.clone(), &mut strings_offsets_map, self.version.clone()));
        }

        for id in &self.scene {
            fm.w_u32(id.to_u32());
        }

        return fm.get_data().to_vec();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        // if objects are present
        if self.objects.len() > 0 {
            let mut objects = Vec::new();
            for object in &self.objects {
                objects.push(object.to_dict(self.version.clone()));
            }
            dict.insert("objects".to_string(), serde_json::Value::Array(objects));
        }
        // if scene is present
        if self.scene.len() > 0 {
            let mut scene = Vec::new();
            for id in &self.scene {
                scene.push(serde_json::Value::Number(serde_json::Number::from(id.to_u32() as i64)));
            }
            dict.insert("scene".to_string(), serde_json::Value::Array(scene));
        }
        // if em2 extra strings are present
        if self.em2_extra_strings.len() > 0 {
            let mut em2_extra_strings = Vec::new();
            for string in &self.em2_extra_strings {
                em2_extra_strings.push(serde_json::Value::String(string.clone()));
            }
            dict.insert("em2_extra_strings".to_string(), serde_json::Value::Array(em2_extra_strings));
        }
        // if unique id is present
        if self.unique_id.to_u128() != 0 {
            dict.insert("unique_id".to_string(), serde_json::Value::String(self.unique_id.to_string(16)));
        }
        // version, integer
        dict.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(self.version.clone() as u32)));
        return serde_json::Value::Object(dict);
    }

    pub fn to_json(&self) -> String {
        // dont sort keys
        return serde_json::to_string_pretty(&self.to_dict()).unwrap();
    }

    pub fn to_json_path(&self, path: String) {
        let data = self.to_json();
        std::fs::write(path, data).unwrap();
    }

    pub fn to_binary(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        return self.pack(endian_type);
    }

    pub fn to_binary_path(&self, path: String, endian_type: file_manipulator::EndianType) {
        let data = self.to_binary(endian_type);
        std::fs::write(path, data).unwrap();
    }

    pub fn from_dict(dict: &serde_json::Value) -> SceneFile {
        let mut objects = Vec::new();
        let mut scene = Vec::new();
        let mut em2_extra_strings = Vec::new();
        let mut unique_id = ID::new(0);
        let mut version = SceneFileVersion::Version1;
        // if objects are present
        if dict.get("objects").is_some() {
            for object in dict["objects"].as_array().unwrap() {
                objects.push(Entity::from_dict(object));
            }
        }
        // if scene is present
        if dict.get("scene").is_some() {
            for id in dict["scene"].as_array().unwrap() {
                scene.push(ID::from_u32(id.as_u64().unwrap() as u32));
            }
        }
        // if em2 extra strings are present
        if dict.get("em2_extra_strings").is_some() {
            for string in dict["em2_extra_strings"].as_array().unwrap() {
                em2_extra_strings.push(string.as_str().unwrap().to_string());
            }
        }
        // if unique id is present
        if dict.get("unique_id").is_some() {
            unique_id = ID::from_string(dict["unique_id"].as_str().unwrap());
        }
        // if version is present
        if dict.get("version").is_some() {
            version = SceneFileVersion::from_u32(dict["version"].as_u64().unwrap() as u32);
        }
        return SceneFile::new(objects, scene, em2_extra_strings, unique_id, version);
    }

    pub fn from_json(json: &str) -> SceneFile {
        let dict = serde_json::from_str(json).unwrap();
        return SceneFile::from_dict(&dict);
    }

    pub fn from_json_path(path: String) -> SceneFile {
        let data = std::fs::read_to_string(path).unwrap();
        return SceneFile::from_json(&data);
    }

    pub fn from_binary(data: &[u8], endian_type: file_manipulator::EndianType) -> SceneFile {
        let mut fm = file_manipulator::FileManipulator::new(data.to_vec(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        let first_four_bytes = fm.r_u32();
        let version = match first_four_bytes {
            0x01000001 => {
                let offset = fm.r_u32() + 4;
                fm.seek(offset as usize);
                let num = fm.r_u32();
                match num {
                    0x02000002 => SceneFileVersion::Version2,
                    0x02000001 => SceneFileVersion::Version2Prototype,
                    _ => panic!("Unknown version!!!")
                }
            },
            _ => SceneFileVersion::Version1
        };
        fm.seek(0);
        let mut scene_file = SceneFile::new(Vec::new(), Vec::new(), Vec::new(), ID::new(0), version);
        scene_file.unpack(&mut fm);
        return scene_file;
    }

    pub fn from_binary_path(path: String, endian_type: file_manipulator::EndianType) -> SceneFile {
        let data = std::fs::read(path).unwrap();
        return SceneFile::from_binary(&data, endian_type);
    }

    pub fn merge_in_dict(&mut self, dict: &serde_json::Value) {
        // if the objects are present, merge them
        if dict.get("objects").is_some() {
            for object in dict["objects"].as_array().unwrap() {
                let mut found = false;
                for self_object in &mut self.objects {
                    if self_object.name.to_lowercase() == object["name"].as_str().unwrap().to_lowercase() {
                        self_object.merge_in_dict(object);
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.objects.push(Entity::from_dict(object));
                }
            }
        }
        // if the scene is present, merge it
        if dict.get("scene").is_some() {
            for id in dict["scene"].as_array().unwrap() {
                let mut found = false;
                for self_id in &mut self.scene {
                    if self_id.to_u32() == id.as_u64().unwrap() as u32 {
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.scene.push(ID::from_u32(id.as_u64().unwrap() as u32));
                }
            }
        }
        // if the em2 extra strings are present, merge them
        if dict.get("em2_extra_strings").is_some() {
            for string in dict["em2_extra_strings"].as_array().unwrap() {
                let mut found = false;
                for self_string in &mut self.em2_extra_strings {
                    if self_string == string.as_str().unwrap() {
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.em2_extra_strings.push(string.as_str().unwrap().to_string());
                }
            }
        }
        // if the unique id is present, merge it
        if dict.get("unique_id").is_some() {
            self.unique_id = ID::from_string(dict["unique_id"].as_str().unwrap());
        }
        // if the version is present, merge it
        if dict.get("version").is_some() {
            self.version = SceneFileVersion::from_u32(dict["version"].as_u64().unwrap() as u32);
        }
    }

    pub fn merge_in_json(&mut self, json: &str) {
        let dict = serde_json::from_str(json).unwrap();
        self.merge_in_dict(&dict);
    }

    pub fn merge_in_json_path(&mut self, path: String) {
        let data = std::fs::read_to_string(path).unwrap();
        self.merge_in_json(&data);
    }
}