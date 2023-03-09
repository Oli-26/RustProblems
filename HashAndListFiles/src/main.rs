
use std::fs;
use std::fs::metadata;
use sha2::{Sha256, Sha512, Digest};

struct File_info {
    file_path : String,
    file_hash : String,
    file_size : u64,
}

fn main() {
   let all_files = search_files("./".to_string());
   let total_file_sizes : u64 = all_files.iter().map(|file| file.file_size).sum();
   for file in all_files {
       println!("{}", file.file_hash);
   }
    println!("Total file size: {}", total_file_sizes);
}

fn search_files(directory : String) -> Vec<File_info> {
    let mut files = Vec::new();
    match metadata(directory.clone()){
        Ok(md) => {
            if md.is_dir(){
                match fs::read_dir(directory.clone()){
                    Ok(paths) => {
                        for path in paths {
                            match path.unwrap().path().to_str(){
                                Some(path_str) => {
                                    files.append(&mut search_files(path_str.to_string()));
                                },
                                None => {}
                            }
                        }
                        files
                    },
                    Err(_) => {
                        Vec::new()
                    }
                }
            }else{
                files.push(
                    File_info{
                        file_path : directory.clone(),
                        file_hash : hash_file_contents(directory.clone()),
                        file_size : md.len(),
                    }
                );
                return files;   
            }
        },
        Err(_) => {
            vec![]
        }
    }
}

fn hash_file_contents(file_path : String) -> String {
    let mut hasher = Sha256::new();
    match fs::read(file_path.clone()){
        Ok(file_contents) => {
            hasher.update(file_contents);
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        Err(_) => {
            String::new()
        }
    }
}