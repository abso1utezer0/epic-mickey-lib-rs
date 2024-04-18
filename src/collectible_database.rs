// collectible_database.rs
// © 2024 Epic Mickey Library

use std::{fs::File, io::{Read, Write}};
use serde_json;
use crate::file_manipulator;

pub struct Collectible {
    pub type_: String,
    pub dev_name: String,
    pub icon_path: String,
}

impl Collectible {
    pub fn new(type_: String, dev_name: String, icon_path: String) -> Collectible {
        return Collectible {
            type_,
            dev_name,
            icon_path,
        }
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_str_jps(&self.type_);
        fm.w_str_jps(&self.dev_name);
        fm.w_str_jps(&self.icon_path);
        return fm.get_data().clone();
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.type_ = fm.r_str_jps();
        self.dev_name = fm.r_str_jps();
        self.icon_path = fm.r_str_jps();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("type".to_string(), serde_json::Value::String(self.type_.clone()));
        map.insert("dev_name".to_string(), serde_json::Value::String(self.dev_name.clone()));
        map.insert("icon_path".to_string(), serde_json::Value::String(self.icon_path.clone()));
        return serde_json::Value::Object(map);
    }

    pub fn from_dict(dict: serde_json::Value) -> Collectible {
        let type_ = dict["type"].as_str().unwrap().to_string();
        let dev_name = dict["dev_name"].as_str().unwrap().to_string();
        let icon_path = dict["icon_path"].as_str().unwrap().to_string();
        return Collectible::new(type_, dev_name, icon_path);
    }

}

pub struct Extra {
    pub global_state: String,
    pub type_: String,
    pub thumbnail_path: String,
    pub asset_path: String,
}

impl Extra {
    pub fn new(global_state: String, type_: String, thumbnail_path: String, asset_path: String) -> Extra {
        return Extra {
            global_state,
            type_,
            thumbnail_path,
            asset_path,
        }
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type, file_manipulator::WriteMode::OVERWRITE);
        fm.w_str_jps(&self.global_state);
        fm.w_str_jps(&self.type_);
        fm.w_str_jps(&self.thumbnail_path);
        fm.w_str_jps(&self.asset_path);
        return fm.get_data().clone();
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.global_state = fm.r_str_jps();
        self.type_ = fm.r_str_jps();
        self.thumbnail_path = fm.r_str_jps();
        self.asset_path = fm.r_str_jps();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("global_state".to_string(), serde_json::Value::String(self.global_state.clone()));
        map.insert("type".to_string(), serde_json::Value::String(self.type_.clone()));
        map.insert("thumbnail_path".to_string(), serde_json::Value::String(self.thumbnail_path.clone()));
        map.insert("asset_path".to_string(), serde_json::Value::String(self.asset_path.clone()));
        return serde_json::Value::Object(map);
    }

    pub fn from_dict(dict: serde_json::Value) -> Extra {
        let global_state = dict["global_state"].as_str().unwrap().to_string();
        let type_ = dict["type"].as_str().unwrap().to_string();
        let thumbnail_path = dict["thumbnail_path"].as_str().unwrap().to_string();
        let asset_path = dict["asset_path"].as_str().unwrap().to_string();
        return Extra::new(global_state, type_, thumbnail_path, asset_path);
    }
}

pub struct CollectibleDatabase {
    pub version: u32,
    pub collectibles: Vec<Collectible>,
    pub extras: Vec<Extra>,
}

impl CollectibleDatabase {
    pub fn new(version: u32) -> CollectibleDatabase {
        return CollectibleDatabase {
            version,
            collectibles: Vec::new(),
            extras: Vec::new(),
        }
    }

    pub fn get_version(&self) -> u32 {
        return self.version;
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn get_collectibles(&self) -> &Vec<Collectible> {
        return &self.collectibles;
    }

    pub fn get_extras(&self) -> &Vec<Extra> {
        return &self.extras;
    }

    pub fn add_collectible(&mut self, collectible: Collectible) {
        self.collectibles.push(collectible);
    }

    pub fn add_extra(&mut self, extra: Extra) {
        self.extras.push(extra);
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let endian_type_clone = endian_type.clone();
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type_clone, file_manipulator::WriteMode::OVERWRITE);
        fm.w_u32(self.version);
        fm.w_u32(self.collectibles.len() as u32);
        for collectible in &self.collectibles {
            fm.write(&collectible.pack(endian_type.clone()));
        }
        fm.w_u32(self.extras.len() as u32);
        for extra in &self.extras {
            fm.write(&extra.pack(endian_type.clone()));
        }
        return fm.get_data().clone();
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.version = fm.r_u32();
        let collectibles_len = fm.r_u32();
        for _ in 0..collectibles_len {
            let mut collectible = Collectible::new(String::new(), String::new(), String::new());
            collectible.unpack(fm);
            self.collectibles.push(collectible);
        }
        let extras_len = fm.r_u32();
        for _ in 0..extras_len {
            let mut extra = Extra::new(String::new(), String::new(), String::new(), String::new());
            extra.unpack(fm);
            self.extras.push(extra);
        }
    }

    pub fn to_binary(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        return self.pack(endian_type);
    }

    pub fn to_binary_path(&self, path: String, endian_type: file_manipulator::EndianType) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.to_binary(endian_type)).unwrap();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(self.version)));
        let mut collectibles = Vec::new();
        for collectible in &self.collectibles {
            collectibles.push(collectible.to_dict());
        }
        map.insert("collectibles".to_string(), serde_json::Value::Array(collectibles));
        let mut extras = Vec::new();
        for extra in &self.extras {
            extras.push(extra.to_dict());
        }
        map.insert("extras".to_string(), serde_json::Value::Array(extras));
        return serde_json::Value::Object(map);
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(&self.to_dict()).unwrap();
    }

    pub fn to_json_path(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_json().as_bytes()).unwrap();
    }

    pub fn from_binary(data: Vec<u8>, endian_type: file_manipulator::EndianType) -> CollectibleDatabase {
        let mut fm = file_manipulator::FileManipulator::new(data, endian_type, file_manipulator::WriteMode::OVERWRITE);
        let mut collectible_database = CollectibleDatabase::new(0);
        collectible_database.unpack(&mut fm);
        return collectible_database;
    }

    pub fn from_binary_path(path: String, endian_type: file_manipulator::EndianType) -> CollectibleDatabase {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        return CollectibleDatabase::from_binary(data, endian_type);
    }

    pub fn from_dict(dict: serde_json::Value) -> CollectibleDatabase {
        let version = dict["version"].as_u64().unwrap() as u32;
        let mut collectible_database = CollectibleDatabase::new(version);
        for collectible in dict["collectibles"].as_array().unwrap() {
            collectible_database.add_collectible(Collectible::from_dict(collectible.clone()));
        }
        for extra in dict["extras"].as_array().unwrap() {
            collectible_database.add_extra(Extra::from_dict(extra.clone()));
        }
        return collectible_database;
    }

    pub fn from_json(json: &str) -> CollectibleDatabase {
        return CollectibleDatabase::from_dict(serde_json::from_str(json).unwrap());
    }

    pub fn from_json_path(path: String) -> CollectibleDatabase {
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        return CollectibleDatabase::from_json(&json);
    }
    
}

