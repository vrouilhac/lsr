use colored::*;
use std::env;
use std::fmt::Debug;
use std::fs;
// use std::io;
// use std::path::Path;

struct Lister {
    path: String,
    with_dir: bool,
    with_file: bool,
    with_hidden: bool,
    with_symlink: bool,
}

#[derive(PartialEq, Debug)]
enum FileType {
    DIRECTORY,
    SYMLINK,
    FILE,
    HIDDEN,
    OTHER,
}

struct PathInfo {
    name: String,
    file_type: FileType,
}

impl Lister {
    pub fn default() -> Self {
        let default_dir = String::from('.');

        Lister {
            path: default_dir,
            with_dir: false,
            with_file: true,
            with_hidden: false,
            with_symlink: false,
        }
    }

    pub fn with_dir(&self, with_dir: bool) -> Self {
        Lister {
            path: String::from(self.path.to_string()),
            with_dir,
            with_file: self.with_file,
            with_hidden: self.with_hidden,
            with_symlink: self.with_symlink,
        }
    }

    pub fn with_path(&self, path: String) -> Self {
        Lister {
            path,
            with_dir: self.with_dir,
            with_file: self.with_file,
            with_hidden: self.with_hidden,
            with_symlink: self.with_symlink,
        }
    }

    pub fn with_hidden(&self, with_hidden: bool) -> Self {
        Lister {
            path: String::from(self.path.to_string()),
            with_dir: self.with_dir,
            with_file: self.with_file,
            with_hidden,
            with_symlink: self.with_symlink,
        }
    }

    pub fn with_file(&self, with_file: bool) -> Self {
        Lister {
            path: String::from(self.path.to_string()),
            with_dir: self.with_dir,
            with_file,
            with_hidden: self.with_hidden,
            with_symlink: self.with_symlink,
        }
    }

    pub fn with_symlink(&self, with_symlink: bool) -> Self {
        Lister {
            path: String::from(self.path.to_string()),
            with_dir: self.with_dir,
            with_file: self.with_file,
            with_hidden: self.with_hidden,
            with_symlink,
        }
    }

    pub fn get_list(&self) -> Vec<PathInfo> {
        let entries: Vec<PathInfo> = match fs::read_dir(&self.path) {
            Ok(entries) => {
                let mut list: Vec<PathInfo> = vec![];

                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let mut to_include: bool = false;
                    let mut path_str = match path.to_str() {
                        Some(str) => String::from(str),
                        _ => String::from(""),
                    };

                    let mut path = String::from(&self.path);

                    if !char_at_is(&self.path, &self.path.len() - 1, '/') {
                        path.push_str("/");
                    }
                    path_str = path_str.replace(&path, "");

                    let file_type: FileType = if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            FileType::DIRECTORY
                        } else if file_type.is_file() {
                            if path_str.contains("./.") {
                                FileType::HIDDEN
                            } else {
                                FileType::FILE
                            }
                        } else {
                            FileType::SYMLINK
                        }
                    } else {
                        FileType::OTHER
                    };

                    if path_str != "" {
                        if file_type == FileType::DIRECTORY && self.with_dir {
                            to_include = true;
                        }

                        if file_type == FileType::HIDDEN && self.with_hidden {
                            to_include = true;
                        }

                        if file_type == FileType::FILE && self.with_file {
                            to_include = true;
                        }

                        if file_type == FileType::SYMLINK && self.with_symlink {
                            to_include = true;
                        }
                    }

                    if to_include {
                        list.push(PathInfo {
                            name: path_str,
                            file_type,
                        });
                    }
                }
                list
            }
            Err(err) => {
                println!("{:?}", err);
                vec![]
            }
        };

        entries
    }
}

fn char_at_is(string: &String, index: usize, c: char) -> bool {
    let char_at = match string.chars().nth(index) {
        Some(str) => str,
        _ => ' ',
    };

    c == char_at
}

fn show_list(list: Vec<String>) {
    for entry in list {
        println!("{}", entry.white());
    }
}

fn add_color(formatted_str: String, entry: PathInfo) -> String {
    match entry.file_type {
        FileType::DIRECTORY => {
            let mut string = String::from("📁 ");
            string.push_str(&formatted_str.truecolor(255, 180, 20).bold().to_string());
            return string;
        }
        FileType::FILE => {
            let mut string = String::from("📄 ");
            string.push_str(&formatted_str.truecolor(255, 255, 255).to_string());
            return string;
        }
        FileType::SYMLINK => {
            let mut string = String::from("🔗 ");
            string.push_str(&formatted_str.truecolor(170, 250, 250).to_string());
            return string;
        }
        FileType::HIDDEN => {
            let mut string = String::from("🫣 ");
            string.push_str(&formatted_str.truecolor(130, 130, 130).italic().to_string());
            return string;
        }
        _ => formatted_str,
    }
}

fn format_list(list: Vec<PathInfo>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for entry in list {
        let formatted_entry = entry.name.trim_start_matches("./");
        let colored_entry = add_color(formatted_entry.to_string(), entry);
        result.push(colored_entry);
    }

    result
}

/*
 * usage: $ lsr
 * options:
 *  * -d        directories
 *  * -s        symlink
 *  * -h        hidden files
 *  * -f        other files
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len() - 1;

    let mut with_dir: bool = false;
    let mut with_hidden: bool = false;
    let mut with_file: bool = false;
    let mut with_symlink: bool = false;

    for arg in &args[1..] {
        if is_arg_option(arg) {
            if arg.contains(&String::from("d")) {
                with_dir = true;
            }
            if arg.contains(&String::from("h")) {
                with_hidden = true;
            }
            if arg.contains(&String::from("f")) {
                with_file = true;
            }
            if arg.contains(&String::from("s")) {
                with_symlink = true;
            }
        }
    }

    let mut path = String::from(".");

    if check_for_path(&args[args_len]) {
        path = String::from(&args[args_len]);
    }

    let lister = Lister::default()
        .with_dir(with_dir)
        .with_file(with_file)
        .with_hidden(with_hidden)
        .with_symlink(with_symlink)
        .with_path(path);

    let ls_result: Vec<PathInfo> = lister.get_list();
    let result = format_list(ls_result);
    show_list(result);
}

fn is_arg_option(string: &str) -> bool {
    if string.chars().nth(0) == Some('-') {
        return true;
    }

    false
}

fn check_for_path(string: &str) -> bool {
    if is_arg_option(string) {
        return false;
    }

    true
}
