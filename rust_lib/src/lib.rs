extern crate zip;
extern crate image;
extern crate jni;

use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::path::PathBuf
use zip::read::{ZipArchive, ZipFile};
use zip::result::ZipError;
use image::{DynamicImage, GenericImageView};

// JNI module to communicate with Android
use jni::JNIEnv;
use jni::objects::{JString};
use jni::sys::{jint};

pub fn extract_cbz(file_path: &str) -> IoResult<()> {
    let zip_file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(zip_file);
    let mut archive: ZipArchive<BufReader<File>> = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let file: Result<ZipFile<'_>, ZipError> = archive.by_index(i);

        match file {
            Ok(f) => {
                let file_name: Option<PathBuf> = f.enclosed_name();
            },
            Err(e) => {
                eprintln!("Error reading file at index {} in dir {}: {:?}", i, file_path, e);
            }
        }
        


    }

    Ok(())
}