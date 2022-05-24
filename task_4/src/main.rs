use std::{fs, io};
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn visit_dirs(dir: &Path, file_extension: &String) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_extension)?;
            } else {
                check_file_extension_and_line_count(&entry, file_extension)?;
            }
        }
    }
    Ok(())
}

fn check_file_extension_and_line_count(file: &DirEntry, file_extension: &String) -> io::Result<()> {
    if file.file_name()
        .to_str().unwrap()
        .split(".")
        .last().unwrap() == file_extension {
        let reader = BufReader::new(File::open(file.path())?);
        println!("File: {} Lines: {}", file.file_name().to_str().unwrap(), reader.lines().count());
    }
    Ok(())
}

fn files_walk(dir: &String, file_extension: &String) -> io::Result<()> {
    visit_dirs(Path::new(dir), file_extension)
}

fn main() -> io::Result<()> {
    files_walk(&".".to_string(), &"txt".to_string())
}
