use std::{fs::File, io::{Read, Write}};
use serde_json;

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

use crate::file_manipulator;
use crate::scene_file;
use crate::dct;
use crate::collectible_database;

pub struct EndianDependentString {
    pub text: String
}

// endian dependent string -- a 4 byte string that is written in reverse order if the endian is little
// if the length of the string is less than 4, it is padded with null bytes
impl EndianDependentString {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        // read a 4 byte string
        let mut buffer = [0; 4];
        fm.read(&mut buffer);
        self.text = String::from_utf8(buffer.to_vec()).unwrap();

        // if the endian is little, reverse the string
        if *fm.get_endian() == file_manipulator::EndianType::LITTLE {
            self.text = self.text.chars().rev().collect();
        }
        // remove null bytes
        self.text = self.text.trim_matches(char::from(0)).to_string();
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type.clone(), file_manipulator::WriteMode::OVERWRITE);
        let mut string_to_write = self.text.clone();
        // pad the string with null bytes
        while string_to_write.len() < 4 {
            string_to_write.push(char::from(0));
        }
        // if the endian is little, reverse the string
        if endian_type.clone() == file_manipulator::EndianType::LITTLE {
            string_to_write = string_to_write.chars().rev().collect();
        }
        fm.write(string_to_write.as_bytes());
        return fm.get_data().to_vec();
    }

    pub fn clone(&self) -> Self {
        return EndianDependentString::new(self.text.clone());
    }
}

pub struct VirtualFile {
    pub type_: EndianDependentString,
    pub compress: bool,
    pub compression_level: u16,
    pub path: String,
    pub data: Vec<u8>
}

impl VirtualFile {
    pub fn new(type_: EndianDependentString, compress: bool, compression_level: u16, path: String, data: Vec<u8>) -> Self {
        Self {
            type_,
            compress,
            compression_level,
            path,
            data
        }
    }

    pub fn get_compressed_data(&self) -> Vec<u8> {
        let mut data = self.data.clone();
        if self.compress {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(self.compression_level as u32));
            encoder.write_all(&self.data).unwrap();
            data = encoder.finish().unwrap();
        }
        return data;
    }

    pub fn get_assembled_data(&self) -> Vec<u8> {
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), file_manipulator::EndianType::LITTLE, file_manipulator::WriteMode::OVERWRITE);
        fm.write(&self.get_compressed_data());
        while fm.size() % 32 != 0 {
            fm.write(&vec![0]);
        }
        return fm.get_data().to_vec();
    }

    pub fn get_real_data_size(&self) -> u32 {
        return self.data.len() as u32;
    }

    pub fn get_compressed_data_size(&self) -> u32 {
        return self.get_compressed_data().len() as u32;
    }

    pub fn get_aligned_data_size(&self) -> u32 {
        return self.get_assembled_data().len() as u32;
    }

    pub fn get_split_path(&self) -> Vec<String> {
        // split the path into 2 parts: the directory and the file name
        let mut split_path = self.path.split("/").collect::<Vec<&str>>();
        let file_name = split_path.pop().unwrap();
        let directory = split_path.join("/");
        return vec![directory, file_name.to_string()];
    }

    pub fn to_dict_stripped(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("type".to_string(), serde_json::Value::String(self.type_.text.clone()));
        dict.insert("compress".to_string(), serde_json::Value::Bool(self.compress));
        dict.insert("compression_level".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.compression_level as f64).unwrap()));
        dict.insert("path".to_string(), serde_json::Value::String(self.path.clone()));
        return serde_json::Value::Object(dict);
    }
}

pub struct Packfile {
    magic: EndianDependentString,
    version: u32,
    files: Vec<VirtualFile>
}

impl Packfile {
    pub fn new(magic: EndianDependentString, version: u32, files: Vec<VirtualFile>) -> Self {
        Self {
            magic,
            version,
            files
        }
    }

    pub fn unpack(&mut self, fm: &mut file_manipulator::FileManipulator) {
        self.magic = EndianDependentString::new("".to_string());
        self.magic.unpack(fm);
        self.version = fm.r_u32();
        let zero = fm.r_u32();
        let header_size = fm.r_u32();
        let mut data_pointer = fm.r_u32();
        data_pointer += header_size;
        let mut current_data_position = data_pointer;
        fm.seek(header_size.try_into().unwrap());
        let num_files = fm.r_u32();
        let string_pointer = (num_files * 24) + header_size + 4;
        let current_header_position = header_size + 4;

        fm.seek(current_header_position.try_into().unwrap());

        for _ in 0..num_files {
            let real_data_size = fm.r_u32();
            let compressed_data_size = fm.r_u32();
            let aligned_data_size = fm.r_u32();

            let mut folder_pointer = fm.r_u32();

            let mut file_type = EndianDependentString::new("".to_string());
            file_type.unpack(fm);

            let mut file_pointer = fm.r_u32();

            folder_pointer += string_pointer;
            file_pointer += string_pointer;

            let current_header_position = fm.tell() as u32;

            fm.seek(folder_pointer.try_into().unwrap());
            let folder = fm.r_str_null();

            fm.seek(file_pointer.try_into().unwrap());
            let file_name = fm.r_str_null();

            let path: String;
            if folder == "" {
                path = file_name.to_owned();
            } else {
                path = folder.to_owned() + "/" + &file_name;
            }

            fm.seek(current_data_position.try_into().unwrap());


            let mut data = vec![0; compressed_data_size as usize];

            fm.read(&mut data);

            let mut compress = false;
            if compressed_data_size != real_data_size {
                compress = true;
                // decompress the data
                let mut decoder = ZlibDecoder::new(data.as_slice());
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data).unwrap();
                data = decompressed_data;
            }

            let virtual_file = VirtualFile::new(file_type, compress, 6, path.to_owned(), data);
            self.files.push(virtual_file);

            current_data_position += aligned_data_size;
            fm.seek(current_header_position.try_into().unwrap());
        }
    }

    pub fn pack(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        let mut endian_type_clone = endian_type.clone();
        let mut fm = file_manipulator::FileManipulator::new(Vec::new(), endian_type.clone(), file_manipulator::WriteMode::OVERWRITE);
        fm.write(&self.magic.pack(endian_type));
        fm.w_u32(self.version);
        fm.w_u32(0);
        let header_size = 32;
        fm.w_u32(header_size);

        let mut path_partition_fm = file_manipulator::FileManipulator::new(Vec::new(), file_manipulator::EndianType::LITTLE, file_manipulator::WriteMode::OVERWRITE);
        // foldername_pointers dictionary
        let mut folder_pointers = std::collections::HashMap::new();
        // filename_pointers dictionary
        let mut filename_pointers = std::collections::HashMap::new();

        for virtual_file in &self.files {
            let split_path = virtual_file.get_split_path();
            let foldername = split_path[0].clone();
            let filename = split_path[1].clone();
            if !folder_pointers.contains_key(&foldername) {
                // folder_pointers[foldername] = len(path_partition)
                folder_pointers.insert(foldername.clone(), path_partition_fm.size() as u32);
                path_partition_fm.w_str_null(&foldername.clone());
            }
            if !filename_pointers.contains_key(&filename) {
                // filename_pointers[filename] = len(path_partition)
                filename_pointers.insert(filename.clone(), path_partition_fm.size() as u32);
                path_partition_fm.w_str_null(&filename.clone());
            }
        }
        let mut data_pointer = (header_size + path_partition_fm.size() as u32 + (self.files.len() as u32 * 24) + 4) as u32;
        while data_pointer % 32 != 0 {
            data_pointer += 1;
        }
        fm.w_u32(data_pointer - header_size);

        fm.seek(header_size.try_into().unwrap());

        fm.w_u32(self.files.len() as u32);

        // loop through the files
        for virtual_file in &self.files {
            let split_path = virtual_file.get_split_path();
            let foldername = split_path[0].clone();
            let filename = split_path[1].clone();
            let real_data_size = virtual_file.get_real_data_size();
            let compressed_data_size = virtual_file.get_compressed_data_size();
            let aligned_data_size = virtual_file.get_aligned_data_size();
            let file_type = virtual_file.type_.clone();
            let path = virtual_file.path.clone();

            let folder_pointer = folder_pointers.get(&foldername).unwrap();
            let file_pointer = filename_pointers.get(&filename).unwrap();

            fm.w_u32(real_data_size);
            fm.w_u32(compressed_data_size);
            fm.w_u32(aligned_data_size);
            fm.w_u32(*folder_pointer);
            fm.write(&file_type.pack(endian_type_clone.clone()));
            fm.w_u32(*file_pointer);
        }
        // write the path partition
        fm.write(&path_partition_fm.get_data());
        fm.seek(data_pointer.try_into().unwrap());
        // pad to 32 bytes
        while fm.size() % 32 != 0 {
            fm.write(&vec![0]);
        }
        for virtual_file in &self.files {
            fm.write(&virtual_file.get_assembled_data());
        }
        return fm.get_data().to_vec();
        
    }

    pub fn extract(&self, path: String) {
        for virtual_file in &self.files {
            // if the directory does not exist, create it
            let split_path = virtual_file.get_split_path();
            let directory = path.clone() + "/" + &split_path[0];
            if !std::path::Path::new(&directory).exists() {
                std::fs::create_dir_all(&directory).unwrap();
            }
            let mut file = File::create(path.clone() + "/" + &virtual_file.path.clone()).unwrap();

            file.write_all(virtual_file.data.clone().as_slice()).unwrap();
        }
    }

    pub fn extract_decompiled(&self, path: String, endian_type: file_manipulator::EndianType, overwrite: bool) {
        for virtual_file in &self.files {
            let split_path = virtual_file.get_split_path();
            let directory = path.clone() + "/" + &split_path[0];
            let mut file_path = path.clone() + "/" + &virtual_file.path.clone();
            
            // dct, bin, or clb should be decompiled and saved as json
            let extension = virtual_file.path.split(".").collect::<Vec<&str>>().pop().unwrap().to_lowercase();
            let mut data = Vec::new();
            match extension.as_str() {
                "dct" => {
                    let dct = dct::DCT::from_binary(virtual_file.data.clone());
                    data = dct.to_json().as_bytes().to_vec();
                    file_path += ".json";
                },
                "bin" => {
                    let scene_file = scene_file::SceneFile::from_binary(&virtual_file.data, endian_type.clone());
                    data = scene_file.to_json().as_bytes().to_vec();
                    file_path += ".json";
                },
                "clb" => {
                    let collectible_database = collectible_database::CollectibleDatabase::from_binary(virtual_file.data.clone(), endian_type.clone());
                    data = collectible_database.to_json().as_bytes().to_vec();
                    file_path += ".json";
                },
                _ => {
                    data = virtual_file.data.clone();
                }
            }
            // if overwrite is false and the file already exists, skip it
            if !overwrite && std::path::Path::new(&file_path).exists() {
                continue;
            }
            // if the directory does not exist, create it
            if !std::path::Path::new(&directory).exists() {
                std::fs::create_dir_all(&directory).unwrap();
            }
            let mut file = File::create(file_path).unwrap();
            file.write_all(&data).unwrap();
        }
    }

    pub fn to_binary(&self, endian_type: file_manipulator::EndianType) -> Vec<u8> {
        return self.pack(endian_type);
    }

    pub fn to_binary_path(&self, path: String, endian_type: file_manipulator::EndianType) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.to_binary(endian_type)).unwrap();
    }

    pub fn to_dict_stripped(&self) -> serde_json::Value {
        let mut dict = serde_json::Map::new();
        dict.insert("magic".to_string(), serde_json::Value::String(self.magic.text.clone()));
        dict.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.version as f64).unwrap()));
        let mut files = Vec::new();
        for virtual_file in &self.files {
            files.push(virtual_file.to_dict_stripped());
        }
        dict.insert("files".to_string(), serde_json::Value::Array(files));
        return serde_json::Value::Object(dict);
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(&self.to_dict_stripped()).unwrap();
    }

    pub fn to_json_path(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_json().as_bytes()).unwrap();
    }

    pub fn from_binary(data: Vec<u8>) -> Self {
        // if the first 4 bytes are "PAK ", then the endian is little, otherwise it is big
        // get the first 4 bytes
        let first_4_bytes = data[0..4].to_vec();
        let mut endian_type = file_manipulator::EndianType::BIG;
        if first_4_bytes == "PAK ".as_bytes() {
            endian_type = file_manipulator::EndianType::LITTLE;
        }

        let mut fm = file_manipulator::FileManipulator::new(
            data,
            endian_type,
            file_manipulator::WriteMode::OVERWRITE
        );

        let mut packfile = Packfile::new(EndianDependentString::new("".to_string()), 0, vec![]);
        packfile.unpack(&mut fm);
        return packfile;
    }

    pub fn from_binary_path(path:String) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        return Packfile::from_binary(data);
    }

    pub fn get_data_from_path(&self, path: String) -> Vec<u8> {
        let fixed_path = path.replace("\\", "/").to_lowercase();
        for virtual_file in &self.files {
            if virtual_file.path.to_lowercase() == fixed_path {
                return virtual_file.data.clone();
            }
        }
        return Vec::new();
    }

    pub fn set_data_from_path(&mut self, path: String, data: Vec<u8>) {
        let fixed_path = path.replace("\\", "/").to_lowercase();
        for virtual_file in &mut self.files {
            if virtual_file.path.to_lowercase() == fixed_path {
                virtual_file.data = data.clone();
            }
        }
    }
}