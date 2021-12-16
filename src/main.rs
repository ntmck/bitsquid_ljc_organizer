use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::ffi::OsStr;

use walkdir::WalkDir;

//Testing:
//  cargo run -- D:\RustProj\bitsquid_ljc_organizer\input\ D:\RustProj\bitsquid_ljc_organizer\

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Valid arguments: input_directory_path output_directory_path. Supplied: {:#?}", args);
    assert!(args[1].len() > 0);
    assert!(args[2].len() > 0);

    let input_dir_path = Path::new(&args[1]);
    let output_dir_path = Path::new(&args[2]);

    assert!(input_dir_path.exists());
    assert!(input_dir_path.is_dir());
    assert!(output_dir_path.is_dir());
    copy_organize(input_dir_path, output_dir_path);
}

fn copy_organize(idir: &Path, odir: &Path) {
    for entry in WalkDir::new(idir)
    .into_iter() {
        if entry.as_ref().unwrap().path().extension() == Some(OsStr::new("lua")) {
            let entry = entry.unwrap();
            let mut ljc = File::open(entry.path()).unwrap();
            let name_len = seek_from_start_read_uleb(&mut ljc, 13);
    
            let name = read_internal_path(&mut ljc, name_len as u64);
            let internal_path = Path::new(&name);

            let mut full_path = odir.to_path_buf();
            full_path.push(internal_path.parent().unwrap());
            fs::create_dir_all(&full_path).expect("Directories could not be created.");
    
            full_path.push(internal_path.file_name().unwrap());
            fs::copy(entry.path(), full_path).expect("File failed to copy.");
        }
    }
}

fn read_internal_path(ljc: &mut File, name_len: u64) -> String {
    let mut name_buf = [0u8; 255];
    ljc.take(name_len).read(&mut name_buf).unwrap();
    String::from_utf8(name_buf[0..name_len as usize].to_vec()).unwrap().replace("@", "")
}

fn seek_from_start_read_uleb(file: &mut File, offset: u64) -> u32 {
    file.seek(SeekFrom::Start(offset)).unwrap();
    let mut value: u32 = 0;
    let mut shift = 1;
    loop {
        let mut buf = [0u8; 1];
        file.take(1).read(&mut buf).unwrap();
        let byte = buf[0];
        let data = byte as u32 & 127;
        let cont = byte as u32 & 128;
        value += data * shift;
        shift *= 128;
        if cont == 0 { break; }
    }
    value
}