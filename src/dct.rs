use std::{fs::File, io::{Read, Write}};
use serde_json;
use crate::file_manipulator;

pub struct DialogEntry {
    pub hashed_key: u32,
    pub text: String
}

impl DialogEntry {
    pub fn new(hashed_key: u32, text: String) -> DialogEntry {
        DialogEntry {
            hashed_key,
            text
        }
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("hashed_key".to_owned(), serde_json::Value::Number(serde_json::Number::from(self.hashed_key)));
        dict.insert("text".to_owned(), serde_json::Value::String(self.text.clone()));
        return serde_json::Value::Object(dict);
    }

    pub fn from_dict(dict: serde_json::Value) -> DialogEntry {
        let hashed_key = dict["hashed_key"].as_u64().unwrap() as u32;
        let text = dict["text"].as_str().unwrap().to_owned();
        return DialogEntry::new(hashed_key, text);
    }
}

pub struct FooterEntry {
    pub number: u32,
    pub text: String
}

impl FooterEntry {
    pub fn new(number: u32, text: String) -> FooterEntry {
        FooterEntry {
            number,
            text
        }
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("number".to_owned(), serde_json::Value::Number(serde_json::Number::from(self.number)));
        dict.insert("text".to_owned(), serde_json::Value::String(self.text.clone()));
        return serde_json::Value::Object(dict);
    }

    pub fn from_dict(dict: serde_json::Value) -> FooterEntry {
        let number = dict["number"].as_u64().unwrap() as u32;
        let text = dict["text"].as_str().unwrap().to_owned();
        return FooterEntry::new(number, text);
    }
}

pub struct DCT {
    pub magic: String,
    pub version1: u32,
    pub hash_seed: u32,
    pub version2: u32,
    pub dialog_entries: Vec<DialogEntry>,
    pub footer_entries: Vec<FooterEntry>
}

impl DCT {
    pub fn new(magic: String, version1: u32, hash_seed: u32, version2: u32, dialog_entries: Vec<DialogEntry>, footer_entries: Vec<FooterEntry>) -> DCT {
        DCT {
            magic,
            version1,
            hash_seed,
            version2,
            dialog_entries,
            footer_entries
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.magic = fm.r_str(4);
        self.version1 = fm.r_u32();
        self.hash_seed = fm.r_u32();
        self.version2 = fm.r_u32();

        let num_dialog_entries = fm.r_u32();

        fm.move_pos(4);

        let footer_offset = fm.tell() as u32 + fm.r_u32() + 9;

        let mut has_footer = false;
        let footer_switch = fm.r_u32();
        if footer_switch == 1 {
            has_footer = true;
        }

        let mut current_data_offset = fm.tell();

        self.dialog_entries = Vec::new();
        for _ in 0..num_dialog_entries {
            let hashed_key = fm.r_u32();
            if hashed_key == 0 {
                fm.move_pos(8);
                // add empty dialog entry
                self.dialog_entries.push(DialogEntry::new(0, "".to_owned()));
                current_data_offset = fm.tell();
                continue;
            }
            let line_offset = fm.tell() as u32 + fm.r_u32() + 1;
            let line_zero = fm.r_u32();
            current_data_offset = fm.tell();
            fm.seek(line_offset as usize);
            let line_text = fm.r_str_null();
            self.dialog_entries.push(DialogEntry::new(hashed_key, line_text));
            fm.seek(current_data_offset);
        }
        self.footer_entries = Vec::new();
        if has_footer {
            while fm.tell() < footer_offset as usize {
                let footer_line_offset = fm.tell() as u32 + fm.r_u32() + 1;
                let footer_line_id = fm.r_u32();
                current_data_offset = fm.tell();
                fm.seek(footer_line_offset as usize);
                let footer_line_text = fm.r_str_null();
                self.footer_entries.push(FooterEntry::new(footer_line_id, footer_line_text));
                fm.seek(current_data_offset);
            }
        }
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), file_manipulator::EndianType::LITTLE, file_manipulator::WriteMode::OVERWRITE);

        let num_dialog_entries = self.dialog_entries.len() as u32;
        let num_footer_entries = self.footer_entries.len() as u32;
        let end_offset = (num_dialog_entries * 12) + (num_footer_entries * 8) - 1;

        fm.w_str(&self.magic);
        fm.w_u32(self.version1);
        fm.w_u32(self.hash_seed);
        fm.w_u32(self.version2);

        fm.w_u32(num_dialog_entries);

        fm.w_u32(1);
        fm.w_u32(end_offset);

        if num_footer_entries > 0 {
            fm.w_u32(1);
        } else {
            fm.w_u32(0);
        }

        let mut current_data_offset = fm.tell();
        let mut current_line_offset = end_offset + 50;

        for dialog_entry in &self.dialog_entries {
            fm.seek(current_data_offset);
            if dialog_entry.hashed_key == 0 {
                fm.w_u32(0);
                fm.w_u32(0);
                fm.w_u32(0);
                current_data_offset = fm.tell();
                continue;
            }
            fm.w_u32(dialog_entry.hashed_key);
            fm.w_u32(current_line_offset - fm.tell() as u32 - 1);
            fm.w_u32(0);
            current_data_offset = fm.tell();
            fm.seek(current_line_offset as usize);
            
            fm.w_str_null(&dialog_entry.text);
            current_line_offset = fm.tell() as u32;
            fm.seek(current_data_offset);
        }

        if num_footer_entries > 0 {
            for footer_entry in &self.footer_entries {
                fm.w_u32(current_line_offset - fm.tell() as u32 - 1);
                fm.w_u32(footer_entry.number);
                current_data_offset = fm.tell();
                fm.seek(current_line_offset as usize);
                fm.w_str_null(&footer_entry.text);
                current_line_offset = fm.tell() as u32;
                fm.seek(current_data_offset);
            }
            fm.write(&[0xDF, 0xFF, 0xFF, 0xFF]);
            fm.w_u32(11);
            fm.w_u32(12);
            fm.w_u32(0);
        }

        // return data
        return fm.get_data().to_vec()
    }

    pub fn to_binary(&self) -> Vec<u8> {
        return self.pack();
    }

    pub fn to_binary_path(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.pack()).unwrap();
    }

    pub fn to_dict(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("magic".to_owned(), serde_json::Value::String(self.magic.clone()));
        dict.insert("version1".to_owned(), serde_json::Value::Number(serde_json::Number::from(self.version1)));
        dict.insert("hash_seed".to_owned(), serde_json::Value::Number(serde_json::Number::from(self.hash_seed)));
        dict.insert("version2".to_owned(), serde_json::Value::Number(serde_json::Number::from(self.version2)));
        let mut dialog_entries = Vec::new();
        for dialog_entry in &self.dialog_entries {
            dialog_entries.push(dialog_entry.to_dict());
        }
        dict.insert("dialog_entries".to_owned(), serde_json::Value::Array(dialog_entries));
        let mut footer_entries = Vec::new();
        for footer_entry in &self.footer_entries {
            footer_entries.push(footer_entry.to_dict());
        }
        dict.insert("footer_entries".to_owned(), serde_json::Value::Array(footer_entries));
        return serde_json::Value::Object(dict);
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(&self.to_dict()).unwrap();
    }

    pub fn to_json_path(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_json().as_bytes()).unwrap();
    }

    pub fn from_binary(data: Vec<u8>) -> DCT {
        let mut fm = file_manipulator::FileManipulator::new(data, file_manipulator::EndianType::LITTLE, file_manipulator::WriteMode::OVERWRITE);
        let mut dct = DCT::new("".to_owned(), 0, 0, 0, Vec::new(), Vec::new());
        dct.unpack(&mut fm);
        return dct;
    }

    pub fn from_binary_path(path: String) -> DCT {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        return DCT::from_binary(data);
    }

    pub fn from_dict(dict: serde_json::Value) -> DCT {
        let magic = dict["magic"].as_str().unwrap().to_owned();
        let version1 = dict["version1"].as_u64().unwrap() as u32;
        let hash_seed = dict["hash_seed"].as_u64().unwrap() as u32;
        let version2 = dict["version2"].as_u64().unwrap() as u32;
        let mut dialog_entries = Vec::new();
        for dialog_entry in dict["dialog_entries"].as_array().unwrap() {
            dialog_entries.push(DialogEntry::from_dict(dialog_entry.clone()));
        }
        let mut footer_entries = Vec::new();
        for footer_entry in dict["footer_entries"].as_array().unwrap() {
            footer_entries.push(FooterEntry::from_dict(footer_entry.clone()));
        }
        return DCT::new(magic, version1, hash_seed, version2, dialog_entries, footer_entries);
    }

    pub fn set_line_from_hash(&mut self, hashed_key: u32, text: String) {
        // if key is 0, ignore
        if hashed_key == 0 {
            return;
        }
        for dialog_entry in &mut self.dialog_entries {
            if dialog_entry.hashed_key == hashed_key {
                dialog_entry.text = text;
                return;
            }
        }
        self.dialog_entries.push(DialogEntry::new(hashed_key, text));
    }

    pub fn merge_in_dict(&mut self, dict: serde_json::Value) {
        // if there is a magic, overwrite it
        if dict["magic"].is_string() {
            self.magic = dict["magic"].as_str().unwrap().to_owned();
        }
        // if there is a version1, overwrite it
        if dict["version1"].is_u64() {
            self.version1 = dict["version1"].as_u64().unwrap() as u32;
        }
        // if there is a hash_seed, overwrite it
        if dict["hash_seed"].is_u64() {
            self.hash_seed = dict["hash_seed"].as_u64().unwrap() as u32;
        }
        // if there is a version2, overwrite it
        if dict["version2"].is_u64() {
            self.version2 = dict["version2"].as_u64().unwrap() as u32;
        }
        // if there are dialog_entries, overwrite any existing ones, or add new ones
        if dict["dialog_entries"].is_array() {
            for dialog_entry in dict["dialog_entries"].as_array().unwrap() {
                let hashed_key = dialog_entry["hashed_key"].as_u64().unwrap() as u32;
                let text = dialog_entry["text"].as_str().unwrap().to_owned();
                self.set_line_from_hash(hashed_key, text);
            }
        }
        // if there are footer_entries, append them
        if dict["footer_entries"].is_array() {
            for footer_entry in dict["footer_entries"].as_array().unwrap() {
                let number = footer_entry["number"].as_u64().unwrap() as u32;
                let text = footer_entry["text"].as_str().unwrap().to_owned();
                self.footer_entries.push(FooterEntry::new(number, text));
            }
        }
    }

    pub fn merge_in_json(&mut self, json: String) {
        let dict = serde_json::from_str(&json).unwrap();
        self.merge_in_dict(dict);
    }

    pub fn merge_in_json_path(&mut self, path: String) {
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        self.merge_in_json(json);
    }

    pub fn from_json(json: String) -> DCT {
        let dict = serde_json::from_str(&json).unwrap();
        return DCT::from_dict(dict);
    }

    pub fn from_json_path(path: String) -> DCT {
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        return DCT::from_json(json);
    }
}