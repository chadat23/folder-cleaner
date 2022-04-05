use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn sanity_check(path: &PathBuf) {
    let paths = fs::read_dir(path).unwrap();

    let mut paths: Vec<String> = paths.into_iter().map(|p| {
        format!("{:#?}", p.unwrap().path()).replace("\"", "")
    }).collect();
    paths.sort();
    println!("{:#?}", paths)
}

fn visit_children(path: &PathBuf) -> Vec<PathBuf> {
    let paths = fs::read_dir(path).unwrap();
    let mut found_paths: Vec<PathBuf> = Vec::new();
    for path in paths {
        let item = path.unwrap().path();
        found_paths.push(item);
        if found_paths.last().unwrap().is_dir() {
            found_paths.extend(visit_children(&found_paths.last().unwrap()));
        }
    }
    found_paths
}

fn find_junk(input_pathes: &Vec<PathBuf>, junk_files: &HashSet<&str>, junk_folders: &HashSet<&str>) -> Vec<usize> {
    let mut stuff_to_delete: Vec<usize> = Vec::new();

    for (i, path) in input_pathes.iter().enumerate() {
        if path.is_dir() {
            if junk_folders.contains(&path.file_name().unwrap().to_str().unwrap()) {
                stuff_to_delete.push(i);
            }
        }
        if path.is_file() {
            if junk_files.contains(&path.file_name().unwrap().to_str().unwrap()) {
                stuff_to_delete.push(i);
            }
        }
    }

    stuff_to_delete
}

fn delete_junk(paths: &Vec<PathBuf>, indices: &Vec<usize>) {
    for index in indices {
        let path = paths.get(*index).unwrap();
        if path.exists() {
            if path.is_file() {
                fs::remove_file(path);
            } else if path.is_dir() {
                fs::remove_dir_all(path);
            }
        }
    }
}

fn main() {
    print!("{}[2J", 27 as char);
    println!("Folder-cleaner, a folder cleaning app!");
    println!("Remember, you point me at a folder and I remove files and folders");
    println!("from it an all subfolders with the offending names, so be careful!");
    println!();
    println!("Enter the absolute path to the top level folder");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Error reading input.");
    let path = path.replace("\n", "");
    let path = path.as_str();
    // let mut path = String::from("/home/chad/rust/EasyRust");

    println!();
    println!("Enter the FOLDER names that you'd like to delete, \", \" delineated");
    let mut folders = String::new();
    io::stdin().read_line(&mut folders).expect("Error reading input.");
    // let folders = String::from(".git, .vscode, target");
    let folders = folders.replace("\n", "");
    let folders = folders.as_str();
    let folders = folders.split(", ");
    let mut junk_folders: HashSet<&str> = HashSet::new();
    for folder in folders {
        junk_folders.insert(folder);
    }
    // let numbers = vec![4, 7, 3, 1, 9, 6, 10, 8, 5, 2];
    // let set:HashSet<i32> = HashSet::from_iter(numbers.iter().cloned());

    println!();
    println!("Enter the FILE names that you'd like to delete, \", \" delineated");
    let mut files = String::new();
    io::stdin().read_line(&mut files).expect("Error reading input.");
    let files = files.replace("\n", "");
    let files = files.as_str();
    // let files = String::from(".gitignore, Cargo.lock");
    let files = files.split(", ");
    let mut junk_files: HashSet<&str> = HashSet::new();
    for file in files {
        junk_files.insert(file);
    }

    let path = PathBuf::from(&path);
    sanity_check(&path);

    println!("Are these the files in the folder that you want to clean (they don't all gets deleted)? Exit if it dosn't look right.");
    let mut junk = String::new();
    
    io::stdin().read_line(&mut junk).expect("Error reading input.");

    let paths = visit_children(&path);
    let junk = find_junk(&paths, &junk_files, &junk_folders);
    delete_junk(&paths, &junk);    
}
