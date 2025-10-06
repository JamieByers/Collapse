use std::fs;
use std::{env, path::PathBuf};

mod debugger;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let folder_name: &str= &args[1];

    if args.len() < 2 {
        println!("No valid argument provided. Please enter a folder name");
        return;
    }

    match folder_name {
        "" => println!("No valid argument provided. Please enter a folder name"),
        "--help" => println!("Usage: collapse <folder_name> [--debug] [--help]"),
        _ => {
            let mut collapse = Collapse::new();
            collapse.handle_collapse(folder_name.to_string());

            if args.len() > 2 {
                match &args[2][..] {
                    "--debug" => collapse.debugger.display_logs(),
                    "--help" => println!("Usage: collapse <folder_name> [--debug] [--help]"),
                    _ => (),
                }
            }
        },
    }
}

pub struct Collapse {
    debugger: debugger::Debugger,
}

impl Collapse {
    fn new() -> Self {
        Collapse {
            debugger: debugger::Debugger::new(true),
        }
    }

    pub fn handle_file_path(&mut self, dir: String) {
        let _nwd = env::set_current_dir(dir);
        let current_dir = env::current_dir().unwrap();
        let read_dir = current_dir.read_dir().unwrap();

        for file in read_dir {
            let old_file_path = file.unwrap().path();
            let old_file_path_string = old_file_path.to_str().unwrap().to_string();
            let new_file_path = self.get_new_file_path(old_file_path);

            self.debugger.log(format!("Moving file from {} to {}", old_file_path_string, new_file_path));

            self.move_file(old_file_path_string, new_file_path)
        }
    }

    pub fn handle_collapse(&mut self, dir: String) {
        self.handle_file_path(dir.to_string());
        let cwd = env::current_dir().unwrap();

        let _ = fs::remove_dir(&cwd);
        self.debugger.log(format!("Removed empty directory: {}", cwd.display()));
    }

    pub fn get_new_file_path(&mut self, file_path: PathBuf) -> String {
        let mut split_file_path: Vec<_> = file_path.to_str().unwrap().split("/").collect();
        split_file_path.remove(split_file_path.len()-2);
        let new_file_path: String = split_file_path.join("/");

        new_file_path
    }

    pub fn move_file(&mut self, old_path: String, new_path: String) {
        std::fs::rename(old_path, new_path).unwrap();
    }

}

