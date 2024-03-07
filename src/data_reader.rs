use std::{
    collections::HashMap, fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf
};

struct ProcessData {
    pid: u16,
    filename: String,
    state: char,
    parent_pid: u16,
    nice: i16,//TODO check whats faster, using signed int or just doing +20 at storing and -20 at using
    starttime: u64
}

fn read_data() -> Vec<ProcessData> {
    let paths = fs::read_dir("/proc").unwrap_or_else(|err| {
        eprintln!("Error reading directory: {}", err);
        std::process::exit(1);
    });

    for path in paths {
        let path = path.unwrap().path();
        let string_path = path.to_string_lossy().to_string();
        let parts: Vec<&str> = string_path.split("proc/").collect();
        let mut task_path = PathBuf::new();
        if let Some(second_part) = parts.get(1) {
            task_path = PathBuf::from(format!("/proc/{}/stat", second_part));
        }
        println!("{}", task_path.to_string_lossy());
    }

    //to make the linter shut up
    let mut vec = Vec::with_capacity(10);
    return vec;
}

//I could use this to make my code much more clear, but it would be faster to do it without. Not sure yet if I'll use it
fn generate_proc_stats_hashmap() {
    let proc_stats: HashMap<i16, String> = {
        let mut map = HashMap::new();
        map.insert(1, String::from("pid"));
        map.insert(2, String::from("comm"));
        map.insert(3, String::from("state"));
        map.insert(4, String::from("ppid"));
        map.insert(5, String::from("pgrp"));
        map.insert(6, String::from("session"));
        map.insert(7, String::from("tty_nr"));
        map.insert(8, String::from("tpgid"));
        map.insert(9, String::from("flags"));
        map.insert(10, String::from("minflt"));
        map.insert(11, String::from("cminflt"));
        map.insert(12, String::from("majflt"));
        map.insert(13, String::from("cmajflt"));
        map.insert(14, String::from("utime"));
        map.insert(15, String::from("stime"));
        map.insert(16, String::from("cutime"));
        map.insert(17, String::from("cstime"));
        map.insert(18, String::from("priority"));
        map.insert(19, String::from("nice"));
        map.insert(20, String::from("num_threads"));
        map.insert(21, String::from("itrealvalue"));
        map.insert(22, String::from("starttime"));
        map.insert(23, String::from("vsize"));
        map.insert(24, String::from("rss"));
        map.insert(25, String::from("rsslim"));
        map.insert(26, String::from("startcode"));
        map.insert(27, String::from("endcode"));
        map.insert(28, String::from("startstack"));
        map.insert(29, String::from("kstkesp"));
        map.insert(30, String::from("kstkeip"));
        map.insert(31, String::from("signal"));
        map.insert(32, String::from("blocked"));
        map.insert(33, String::from("sigignore"));
        map.insert(34, String::from("sigcatch"));
        map.insert(35, String::from("wchan"));
        map.insert(36, String::from("nswap"));
        map.insert(37, String::from("cnswap"));
        map.insert(38, String::from("exit_signal"));
        map.insert(39, String::from("processor"));
        map.insert(40, String::from("rt_priority"));
        map.insert(41, String::from("policy"));
        map.insert(42, String::from("delayacct_blkio_ticks"));
        map.insert(43, String::from("guest_time"));
        map.insert(44, String::from("cguest_time"));
        map.insert(45, String::from("start_data"));
        map.insert(46, String::from("end_data"));
        map.insert(47, String::from("start_brk"));
        map.insert(48, String::from("arg_start"));
        map.insert(49, String::from("arg_end"));
        map.insert(50, String::from("env_start"));
        map.insert(51, String::from("env_end"));
        map.insert(52, String::from("exit_code"));
        map
    };
}