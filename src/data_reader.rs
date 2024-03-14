use std::{
    fs, path::{Path, PathBuf}
};

pub struct ProcessData {
    pid: u16,
    filename: String,
    state: char,
    parent_pid: u16,
    nice: i16,//TODO check whats faster, using signed int or just doing +20 at storing and -20 at using
    starttime: u64
}

pub fn read_data() -> Vec<ProcessData> {
    let mut processes: Vec<ProcessData> = Vec::new(); // Define empty vector outside loop

    let paths = fs::read_dir("/proc").unwrap_or_else(|err| {
        eprintln!("Error reading directory: {}", err);
        std::process::exit(1);
    });

    for path in paths {
        let path = path.unwrap().path();
        let string_path = path.to_string_lossy().to_string();
        let parts: Vec<&str> = string_path.split("proc/").collect();
        if let Some(second_part) = parts.get(1) {
            //path might not contain /stat, make check for this
            let path = format!("/proc/{}/stat", second_part);
            if !Path::exists(Path::new(&path)) {
                continue;
            }
            
            let task_path = PathBuf::from(format!("/proc/{}/stat", second_part));
            
            let md = fs::metadata(task_path.clone()).unwrap();
            if !md.is_file() {
                continue;
            }

            //println!("{}", task_path.to_string_lossy());

            
            let contents = fs::read_to_string(task_path).expect("reading file failed");

            let entries: Vec<String> = contents.split_whitespace().map(str::to_string).collect();

            // Parse entries and extract data (replace with your parsing logic)
            let pid: u16 = entries[0].parse().unwrap();
            let filename = entries[1].clone(); // Fix so it parses everything between (), not just 2nd element
            let state: char = entries[2].clone().chars().next().expect("string is empty");
            println!("{}", entries[3].clone());
            let parent_pid: u16 = entries[3].parse().ok().unwrap_or(0);
            let nice = entries[18].parse().unwrap();
            let starttime = entries[21].parse().unwrap();

            let process_data = ProcessData {
                pid,
                filename,
                state,
                parent_pid,
                nice,
                starttime
            };

            processes.push(process_data); // Add data to the processes vector

            println!("");
            for entry in entries {
            //    print!("{} ", entry)
            }
        }
    }
    return processes;
}