use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use shared::input::AocBufReader;

lazy_static! {
    static ref CD_REGEX: Regex = Regex::new(r"^\$ cd ([a-zA-Z\.]*)$").unwrap();
    static ref LS_REGEX: Regex = Regex::new(r"^\$ ls$").unwrap();
    static ref FILE_LISTING_REGEX: Regex = Regex::new(r"^([0-9]*) ([a-zA-Z\.]*)$").unwrap();
}

#[derive(Clone, PartialEq, Eq)]
enum FileType {
    FILE,
    DIRECTORY,
}

#[derive(Clone)]
struct File {
    id: usize,
    name: String,
    file_type: FileType,
    parent_file_id: Option<usize>,
    children_file_names_by_id: HashMap<usize, String>,
    children_file_ids_by_name: HashMap<String, usize>,
    size: usize,
}

impl File {
    fn new_directory(id: usize, name: String, parent_file_id: Option<usize>) -> File {
        File {
            id,
            name,
            file_type: FileType::DIRECTORY,
            parent_file_id,
            children_file_names_by_id: HashMap::new(),
            children_file_ids_by_name: HashMap::new(),
            size: 0,
        }
    }

    fn new_file(id: usize, name: String, parent_file_id: usize, size: usize) -> File {
        File {
            id,
            name,
            file_type: FileType::FILE,
            parent_file_id: Some(parent_file_id),
            children_file_names_by_id: HashMap::new(),
            children_file_ids_by_name: HashMap::new(),
            size: size,
        }
    }

    fn total_size(&self, file_system: &FileSystem) -> usize {
        match self.file_type {
            FileType::FILE => self.size,
            FileType::DIRECTORY => self
                .children_file_names_by_id
                .keys()
                .map(|file_id| {
                    file_system
                        .get_file_by_id(*file_id)
                        .total_size(&file_system)
                })
                .sum(),
        }
    }
}

struct FileSystem {
    files: Vec<File>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            files: vec![File::new_directory(0, "/".to_string(), None)],
        }
    }

    fn make_directory(&mut self, name: String, parent_id: usize) -> usize {
        let n_files = self.files.len();
        let new_file = File::new_directory(n_files, name.clone(), Some(parent_id));
        self.files.push(new_file);
        self.files[parent_id]
            .children_file_ids_by_name
            .insert(name.clone(), n_files);
        self.files[parent_id]
            .children_file_names_by_id
            .insert(n_files, name);
        n_files
    }

    fn add_file(&mut self, name: String, parent_id: usize, size: usize) -> usize {
        let n_files = self.files.len();
        let new_file = File::new_file(n_files, name.clone(), parent_id, size);
        let new_file_id = new_file.id;
        self.files.push(new_file);
        self.files[parent_id]
            .children_file_ids_by_name
            .insert(name.clone(), new_file_id);
        self.files[parent_id]
            .children_file_names_by_id
            .insert(new_file_id, name);
        n_files
    }

    fn get_file_by_id(&self, id: usize) -> &File {
        &self.files[id]
    }

    fn directories(&self) -> Vec<File> {
        self.files
            .iter()
            .filter(|file| file.file_type == FileType::DIRECTORY)
            .cloned()
            .collect()
    }
}

fn parse_input(mut reader: AocBufReader) -> FileSystem {
    reader.next().unwrap(); // `$ cd /`
    let mut file_system = FileSystem::new();
    let mut cwd_id: usize = 0;
    let mut cwd: File;

    while let Some(command) = reader.next() {
        cwd = file_system.get_file_by_id(cwd_id).clone();
        if let Some(dest_directory_match) = CD_REGEX.captures(&command) {
            let dest_directory_name: String =
                dest_directory_match.get(1).unwrap().as_str().to_string();
            if dest_directory_name == "..".to_string() {
                cwd_id = cwd.parent_file_id.unwrap();
            } else {
                if cwd
                    .children_file_ids_by_name
                    .contains_key(&dest_directory_name)
                {
                    cwd_id = *cwd
                        .children_file_ids_by_name
                        .get(&dest_directory_name)
                        .unwrap();
                } else {
                    cwd_id = file_system.make_directory(dest_directory_name, cwd.id);
                }
            }
        } else if let Some(_ls_match) = LS_REGEX.captures(&command) {
            continue;
        } else if let Some(file_listing) = FILE_LISTING_REGEX.captures(&command) {
            let new_file_name: String = file_listing.get(2).unwrap().as_str().to_string();
            let new_file_size: usize = file_listing
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            file_system.add_file(new_file_name, cwd.id, new_file_size);
        }
    }

    file_system
}

fn part_1(file_system: &FileSystem) -> usize {
    let part_1_threshold: usize = 100_000;

    file_system
        .directories()
        .iter()
        .map(|directory| directory.total_size(file_system))
        .filter(|total_size| total_size <= &part_1_threshold)
        .sum()
}

fn part_2(file_system: &FileSystem) -> usize {
    let system_disk_space: usize = 70_000_000;
    let required_disk_space: usize = 30_000_000;
    let remaining_disk_space =
        system_disk_space - file_system.get_file_by_id(0).total_size(file_system);
    let disk_space_must_free = required_disk_space - remaining_disk_space;

    file_system
        .directories()
        .iter()
        .map(|directory| directory.total_size(file_system))
        .filter(|total_size| total_size >= &disk_space_must_free)
        .min()
        .unwrap()
}

fn main() {
    let file_system = parse_input(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{}", part_1(&file_system));
    println!("{}", part_2(&file_system));
}
