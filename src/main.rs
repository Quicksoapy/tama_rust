use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let paths = fs::read_dir("/proc").unwrap_or_else(|err| {
        eprintln!("Error reading directory: {}", err);
        std::process::exit(1);
    });

    for path in paths {
        let path = path.unwrap().path();
        let string_path = path.to_string_lossy().to_string();
        let parts: Vec<&str> = string_path.split("proc/").collect();
        if let Some(second_part) = parts.get(1) {
            let task_path = PathBuf::from(format!("/proc/{}/task/{}/status", second_part, second_part));
            
            // Open the file and handle errors
            let file = match File::open(&task_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error opening file {}: {}", task_path.display(), e);
                    continue;
                }
            };

            let reader = BufReader::new(file);

            let name = match reader.lines().next() {
                Some(Ok(first_line)) => first_line,
                _ => "name not found".to_string(),
            };
                
                print!("{}", name + " ");
                println!("{}", task_path.display()); 
            }
        }
    }

