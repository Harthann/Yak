use crate::alloc::vec::Vec;
use crate::alloc::string::String;

pub fn debugfs(mut command: Vec<String>) {
    command.remove(0); // Delete coommand name before sending to subcommand
    match command[0].as_str() {
        "ls" => ls(command),
        "cat" => cat(command),
        _ => crate::kprintln!("Unknown command: {}", command[0]),
    }
}

pub fn cat(command: Vec<String>) {
    let file_content = crate::fs::ext2::get_file_content(command[1].as_str());
    for i in file_content {
        crate::kprint!("{}", i);
    }
}

pub fn ls(command: Vec<String>) {
    let path = match command.len() {
        1 => "/",
        _ => command[1].as_str(),
    };
    crate::dprintln!("Ls: {}", path);
    let dentries = crate::fs::ext2::list_dir(path);

    for i in dentries {
        crate::kprint!("{} ", i.name);
    }
    crate::kprintln!("");
}


