use core::ffi::CStr;

use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

use crate::cli::commands::hexdump;
use crate::fs::ext2;

pub static ROOT_INODE: usize = 2;
pub static mut CURRENTDIR_INODE: usize = ROOT_INODE;
pub static mut PWD: [u8; 256] = [0; 256];

fn help() {
	crate::kprintln!("Command available: ls,cat,imap,cd,mkdir,pwd,test");
}

pub fn debugfs(mut command: Vec<String>) {
	if command.len() > 0 {
		command.remove(0); // Delete command name before sending to subcommand
		match command[0].as_str() {
			"ls" => ls(command),
			"cat" => cat(command),
			"imap" => imap(command),
			"cd" => cd(command),
			"mkdir" => mkdir(command),
			"pwd" => pwd(),
			"test" => test(command),
			_ => {
				crate::kprintln!("Unknown command: {}", command[0]);
				help();
			}
		}
	} else {
		help();
	}
}

fn pwd() {
	crate::kprintln!("[pwd]   INODE: {:>6}  PATH: {}", unsafe { CURRENTDIR_INODE }, unsafe { CStr::from_bytes_until_nul(&PWD).unwrap().to_str().unwrap() });
	crate::kprintln!("[root]  INODE: {:>6}  PATH: /", ROOT_INODE);
}

fn mkdir(command: Vec<String>) {
	ext2::create_dir(command[1].as_str(), unsafe { CURRENTDIR_INODE });
}

fn cat(command: Vec<String>) {
	let file_content = ext2::get_file_content(command[1].as_str(), unsafe { CURRENTDIR_INODE });
	for i in file_content {
		crate::kprint!("{}", i);
	}
}

fn test(command: Vec<String>) {
	let mut ext2 = ext2::Ext2::new();
	// let mut dentry = crate::fs::ext2::inode::Dentry::default();

	let node = ext2.alloc_node(0);
	let block = ext2.alloc_block(0);
	crate::dprintln!("Node {}", node);
}

fn ls(command: Vec<String>) {
	let path = match command.len() {
		1 => "",
		_ => command[1].as_str()
	};
	crate::dprintln!("Ls: {}", path);
	let dentries = ext2::list_dir(path, unsafe { CURRENTDIR_INODE });

	for i in dentries {
		crate::kprint!("{} ", i.name);
	}
	crate::kprintln!("");
}

fn cd(command: Vec<String>) {
	let path = match command.len() {
		1 => "",
		_ => command[1].as_str()
	};
	let root = path.starts_with('/');
	let mut splited: Vec<&str> = path.split("/").filter(|s| !s.is_empty()).collect();
	let mut path = splited.join("/");
	if root {
		path.insert_str(0, "/");
	}
	let ext2 = ext2::Ext2::new();
	let lookup = ext2.recurs_find(&path, unsafe { CURRENTDIR_INODE });
	match lookup {
		None => crate::kprintln!("Dir not found"),
		Some((inodeno, inode)) => {
			if inode.is_dir() {
				unsafe {
					CURRENTDIR_INODE = inodeno;
					let mut pwd = CStr::from_bytes_until_nul(&PWD).unwrap().to_str().unwrap().to_string();
					if root {
						pwd = path.clone();
					} else {
						pwd.push_str("/");
						pwd.push_str(&path);
					}
					let mut splited: Vec<&str> = pwd.split("/").filter(|s| !s.is_empty() && s != &".").collect();
					let splited_cpy = splited.clone();
					let len = splited.len();
					let mut index = 0;
					for elem in &mut splited_cpy.iter() {
						if elem == &".." {
							if index != 0 {
								splited.remove(index);
								splited.remove(index - 1);
								index -= 1;
							} else {
								splited.remove(index);
							}
						} else {
							index += 1;
						}
					}
					let mut pwd = splited.join("/");
					pwd.insert_str(0, "/");
					PWD[0..pwd.len()].clone_from_slice(pwd.as_bytes());
					PWD[pwd.len()] = b'\0';
				};
			} else {
				crate::kprintln!("Error: {} is not a directory", path);
			}
		},
	};
}

fn imap(command: Vec<String>) {
	let path = match command.len() {
		1 => "/",
		_ => command[1].as_str()
	};
	let ext2 = ext2::Ext2::new();
	let lookup = ext2.get_inode_of(path);
	match lookup {
		None => crate::kprintln!("File not found"),
		Some((inodeno, _)) => {
			crate::kprintln!(
				"Inode {inodeno} is part of block group {}",
				ext2.inode_to_bgroup(inodeno as u32)
			);
			crate::kprintln!(
				"{:8} located at block {}, offset {:#04x}",
				"",
				ext2.inode_to_block(inodeno as u32),
				ext2.inode_to_offset(inodeno as u32)
			);
		}
	}
}
