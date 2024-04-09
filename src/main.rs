// io stuff
//

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use collectible_database::CollectibleDatabase;
use packfile::Packfile;

mod file_manipulator;
mod collectible_database;
mod packfile;
mod dct;
mod scene_file;

fn main() {
    // testing out the packfile stuff
    let mut pak = Packfile::from_binary_path(r"C:\Users\thise\Documents\epic_mickey_clean\DATA\files\packfiles\_Dynamic_og.pak".to_owned());
    pak.extract_decompiled("out".to_owned(), file_manipulator::EndianType::BIG, false);
}
