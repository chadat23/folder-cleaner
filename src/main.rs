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
    println!("{:#?}", paths);
}

fn visit_children(path: &PathBuf) -> Vec<PathBuf> {
    let paths = fs::read_dir(path).unwrap();
    let mut found_paths: Vec<PathBuf> = Vec::new();
    for path in paths {
        // found_paths.push(path);
        let item = path.unwrap().path();
        found_paths.push(item);
        // println!("{:?}", found_paths.last().unwrap());
        if found_paths.last().unwrap().is_dir() {
            found_paths.extend(visit_children(&found_paths.last().unwrap()));
        }
    }
    found_paths
}

fn main() {
    print!("{}[2J", 27 as char);
    println!("Folder-cleaner, a folder cleaning app!");
    println!("Remember, you point me at a folder and I remove files and folders");
    println!("from it an all subfolders with the offending names, so be careful!");
    println!();
    println!("Enter the absolute path to the top level folder");

    let mut path = String::new();
    // io::stdin().read_line(&mut path).expect("Error reading input.");
    let mut path = String::from("/home/chad/rust/TestEasyRust");

    println!();
    println!("Enter the FOLDER names that you'd like to delete, \", \" delineated");
    let mut folders = String::new();
    // io::stdin().read_line(&mut folders).expect("Error reading input.");
    let folders = String::from(".git, .vscode, target");
    let folders = folders.split(", ");
    let mut folder_collection: HashSet<&str> = HashSet::new();
    for folder in folders {
        folder_collection.insert(folder);
    }
    // let numbers = vec![4, 7, 3, 1, 9, 6, 10, 8, 5, 2];
    // let set:HashSet<i32> = HashSet::from_iter(numbers.iter().cloned());

    println!();
    println!("Enter the FILE names that you'd like to delete, \", \" delineated");
    let mut files = String::new();
    // io::stdin().read_line(&mut files).expect("Error reading input.");
    let files = String::from(".gitignore, Cargo.lock");
    let files = files.split(", ");
    let mut file_collection: HashSet<&str> = HashSet::new();
    for file in files {
        file_collection.insert(file);
    }

    let path = PathBuf::from(&path);
    sanity_check(&path);

    // let path = PathBuf::from(&path);
    // let mut files = visit_children(&path);
    // files.sort();
    // let delete = Vec::new();
    // for file in files {
    //     println!("{:#?}, {:#?}", file, file.file_name().unwrap());

    // }

    // /home/chad/rust/TestEasyRust
}
