// file_manipulator.rs
// © 2024 Epic Mickey Library

use std::io::Write;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, PartialEq)]
pub enum EndianType {
    BIG,
    LITTLE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WriteMode {
    OVERWRITE,
    INSERT,
}

pub struct FileManipulator {
    pub endian: EndianType,
    pub write_mode: WriteMode,
    pub data: Vec<u8>,
    pub pos: usize,
}

impl FileManipulator {
    pub fn new(data: Vec<u8>, endian: EndianType, write_mode: WriteMode) -> FileManipulator {
        FileManipulator {
            endian,
            write_mode,
            data,
            pos: 0,
        }
    }

    pub fn read_backwards(&mut self, length: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for _ in 0..length {
            self.move_pos(-1);
            let byte = self.read_byte();
            data.push(byte);
            self.move_pos(-1);
        }
        data.reverse();
        return data
    }

    pub fn write(&mut self, buffer: &[u8]) {
        match self.write_mode {
            WriteMode::OVERWRITE => {
                let pos = self.pos;
                if pos + buffer.len() > self.data.len() {
                    let padding = pos + buffer.len() - self.data.len();
                    for _ in 0..padding {
                        self.data.push(0);
                    }
                }
                for (i, byte) in buffer.iter().enumerate() {
                    self.data[pos + i] = *byte;
                }
                self.move_pos(buffer.len() as isize);
            }
            WriteMode::INSERT => {
                let pos = self.pos;
                let mut data = self.data.split_off(pos);
                self.data.extend_from_slice(buffer);
                self.data.append(&mut data);
                self.move_pos(buffer.len() as isize);
            }
        }
    }

    pub fn get_struct_order_prefix(&self) -> &str {
        match self.endian {
            EndianType::BIG => ">",
            EndianType::LITTLE => "<",
        }
    }

    pub fn r_u8(&mut self) -> u8 {
        let byte = self.read_byte();
        return byte;
    }

    pub fn r_s8(&mut self) -> i8 {
        let byte = self.read_byte();
        return byte as i8;
    }

    pub fn r_u16(&mut self) -> u16 {
        let mut data = [0; 2];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => u16::from_be_bytes(data),
            EndianType::LITTLE => u16::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_u16_jps(&mut self) -> u16 {
        let value = self.r_u16();
        self.move_pos(2);
        return value
    }

    pub fn r_s16(&mut self) -> i16 {
        let mut data = [0; 2];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => i16::from_be_bytes(data),
            EndianType::LITTLE => i16::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_s16_jps(&mut self) -> i16 {
        let value = self.r_s16();
        self.move_pos(2);
        return value
    }

    pub fn r_u32(&mut self) -> u32 {
        let mut data = [0; 4];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => u32::from_be_bytes(data),
            EndianType::LITTLE => u32::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_s32(&mut self) -> i32 {
        let mut data = [0; 4];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => i32::from_be_bytes(data),
            EndianType::LITTLE => i32::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_u64(&mut self) -> u64 {
        let mut data = [0; 8];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => u64::from_be_bytes(data),
            EndianType::LITTLE => u64::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_s64(&mut self) -> i64 {
        let mut data = [0; 8];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => i64::from_be_bytes(data),
            EndianType::LITTLE => i64::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_u128(&mut self) -> u128 {
        let mut data = [0; 16];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => u128::from_be_bytes(data),
            EndianType::LITTLE => u128::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_s128(&mut self) -> i128 {
        let mut data = [0; 16];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => i128::from_be_bytes(data),
            EndianType::LITTLE => i128::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_float(&mut self) -> f32 {
        let mut data = [0; 4];
        self.read(&mut data);
        let value = match self.endian {
            EndianType::BIG => f32::from_be_bytes(data),
            EndianType::LITTLE => f32::from_le_bytes(data),
        };
        return value;
    }

    pub fn r_str(&mut self, length: usize) -> String {
        let mut buffer = Vec::new();
        for _ in 0..length {
            let byte = self.read_byte();
            buffer.push(byte);
        }
        return String::from_utf8(buffer).unwrap()
    }

    pub fn r_str_jps(&mut self) -> String {
        let size = self.r_u8();
        let text_length = self.r_u8();
        let text = self.r_str_null();
        self.align(4);
        return text
    }

    pub fn r_str_null(&mut self) -> String {
        let mut buffer = Vec::new();
        loop {
            let byte = self.read_byte();
            if byte == 0 {
                break;
            }
            buffer.push(byte);
        }
        return String::from_utf8(buffer).unwrap()
    }

    pub fn r_bool(&mut self) -> bool {
        // if its FF FF FF FF, return true, else return false
        let mut buffer = Vec::new();
        for _ in 0..4 {
            let byte = self.read_byte();
            buffer.push(byte);
        }
        return buffer == vec![255, 255, 255, 255]
    }

    pub fn w_u8(&mut self, data: u8) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_s8(&mut self, data: i8) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_u16(&mut self, data: u16) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_u16_jps(&mut self, data: u16, mode: i32) {
        self.w_u16(data);
        let byte_to_write = match mode {
            0 => 0xCD,
            1 => 0xFF,
            _ => panic!("Invalid mode"),
        };
        for _ in 0..2 {
            self.write_byte(byte_to_write);
        }
    }

    pub fn w_s16(&mut self, data: i16) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_s16_jps(&mut self, data: i16, mode: i32) {
        self.w_s16(data);
        let byte_to_write = match mode {
            0 => 0xCD,
            1 => 0xFF,
            _ => panic!("Invalid mode"),
        };
        for _ in 0..2 {
            self.write_byte(byte_to_write);
        }
    }

    pub fn w_u32(&mut self, data: u32) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_s32(&mut self, data: i32) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_u64(&mut self, data: u64) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_s64(&mut self, data: i64) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_u128(&mut self, data: u128) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_s128(&mut self, data: i128) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_float(&mut self, data: f32) {
        let bytes = match self.endian {
            EndianType::BIG => data.to_be_bytes(),
            EndianType::LITTLE => data.to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn w_str(&mut self, text: &str) {
        self.write(text.as_bytes());
    }

    pub fn w_str_jps(&mut self, text: &str) {
        let mut text_length = text.len();
        if text_length > 0 {
            text_length += 1;
        }
        let mut size = text_length + 2;
        while (size % 4) != 0 {
            size += 1;
        }
        self.w_u8(size as u8);
        self.w_u8(text_length as u8);
        self.w_str_null(text);
        
        let mut padding = 1;
        if text_length > 0 {
            padding = size - text_length - 2;
        }
        for _ in 0..padding {
            self.write_byte(0);
        }
    }

    pub fn w_str_null(&mut self, text: &str) {
        self.write(text.as_bytes());
        self.write_byte(0);
    }

    pub fn w_bool(&mut self, value: bool) {
        let num = if value { 255 } else { 0 };
        for _ in 0..4 {
            self.write_byte(num);
        }
    }

    pub fn flip_endian(&mut self) {
        self.endian = match self.endian {
            EndianType::BIG => EndianType::LITTLE,
            EndianType::LITTLE => EndianType::BIG,
        };
    }

    pub fn move_pos(&mut self, amount: isize) {
        let new_pos = self.pos as isize + amount;
        if new_pos < 0 || new_pos > self.data.len() as isize {
            panic!("Invalid position");
        }
        self.pos = new_pos as usize;
    }

    pub fn align(&mut self, num: usize) {
        let pos = self.pos;
        if (pos % num) != 0 {
            let padding = num - (pos % num);
            self.move_pos(padding as isize);
        }
    }

    pub fn pad(&mut self, amount: usize) {
        // align to amount but write 0s
        let pos = self.pos;
        let padding = amount - (pos % amount);
        for _ in 0..padding {
            self.write_byte(0);
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn set_endian(&mut self, endian: &str) {
        match endian {
            "big" => self.endian = EndianType::BIG,
            "little" => self.endian = EndianType::LITTLE,
            _ => panic!("Invalid endian type"),
        }
    }

    pub fn to_path(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.data).unwrap();
    }

    pub fn from_path(path: &str, endian: EndianType, write_mode: WriteMode) -> FileManipulator {
        let mut file = File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        FileManipulator::new(data, endian, write_mode)
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.pos];
        self.move_pos(1);
        byte
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.data.push(byte);
        self.move_pos(1);
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        let pos = self.pos;
        let data = &self.data[pos..];
        let len = buffer.len();
        buffer.copy_from_slice(&data[..len]);
        self.move_pos(len as isize);
    }

    // get the current position
    pub fn tell(&self) -> usize {
        return self.pos;
    }

    // set the current position
    pub fn seek(&mut self, pos: usize) {
        // if the position is greater than the size of the data, extend the data with 0s
        if pos > self.data.len() {
            let padding = pos - self.data.len();
            for _ in 0..padding {
                self.data.push(0);
            }
        }
        self.pos = pos;
    }

    // get the data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_endian(&self) -> &EndianType {
        &self.endian
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}


