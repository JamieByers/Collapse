use std::fs;
use std::{env, path::PathBuf};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() >= 2 {
        println!("{:?}", args);
    }

    let folder_name: &str= &args[1];

    match folder_name {
        "" => println!("No valid argument provided. Please enter a folder name"),
        _ => handle_collapse(folder_name.to_string()),
    }

}

fn handle_collapse(dir: String) {
    handle_file_path(dir.to_string());
    let cwd = env::current_dir().unwrap();

    let _ = fs::remove_dir(&cwd);
    println!("Removed empty directory: {}", cwd.display());
}

fn handle_file_path(dir: String) {
    let _nwd = env::set_current_dir(dir);
    let current_dir = env::current_dir().unwrap();
    let read_dir = current_dir.read_dir().unwrap();

    for file in read_dir {
        let old_file_path = file.unwrap().path();
        let old_file_path_string = old_file_path.to_str().unwrap().to_string();
        let new_file_path = get_new_file_path(old_file_path);
        println!("Moved {} to {}", old_file_path_string, new_file_path);

        move_file(old_file_path_string, new_file_path)
    }
}

fn get_new_file_path(file_path: PathBuf) -> String {
    let mut split_file_path: Vec<_> = file_path.to_str().unwrap().split("/").collect();
    split_file_path.remove(split_file_path.len()-2);
    let new_file_path: String = split_file_path.join("/");

    new_file_path
}

fn move_file(old_path: String, new_path: String) {
    std::fs::rename(old_path, new_path).unwrap();
}

