use crate::alloc::vec::Vec;
use crate::alloc::string::String;
use crate::cli::commands::hexdump;

pub fn debugfs(mut command: Vec<String>) {
    command.remove(0); // Delete coommand name before sending to subcommand
    match command[0].as_str() {
        "ls" => ls(command),
        "cat" => cat(command),
        "imap" => imap(command),
        _ => crate::kprintln!("Unknown command: {}", command[0]),
    }
}

fn cat(command: Vec<String>) {
    let file_content = crate::fs::ext2::get_file_content(command[1].as_str());
    for i in file_content {
        crate::kprint!("{}", i);
    }
}

fn ls(command: Vec<String>) {
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

fn imap(command: Vec<String>) {
    let path = match command.len() {
        1 => "/",
        _ => command[1].as_str(),
    };
    let ext2 = crate::fs::ext2::Ext2::new();
    let lookup = ext2.get_inode_of(path);
    match lookup {
        None => crate::kprintln!("File not found"),
        Some((inodeno, _)) => {
            crate::kprintln!("Inode {inodeno} is part of block group {}", ext2.inode_to_bgroup(inodeno as u32));
            crate::kprintln!("{:8} located at block {}, offset {:#04x}", "", ext2.inode_to_block(inodeno as u32), ext2.inode_to_offset(inodeno as u32));
        }
    }
}

