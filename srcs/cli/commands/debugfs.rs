use core::ffi::CStr;

use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

use crate::fs::ext2;
use crate::utils::path::Path;

pub static ROOT_INODE: usize = 2;
pub static mut CURRENTDIR_INODE: usize = ROOT_INODE;
pub static mut PWD: [u8; 256] = [0; 256];

fn help() {
	crate::kprintln!("Command available: ls,cat,imap,cd,touch,mkdir,pwd,test");
}

pub fn debugfs(mut command: Vec<String>) {
	if command.len() > 1 {
		command.remove(0); // Delete command name before sending to subcommand
		match command[0].as_str() {
			"ls" => ls(command),
			"cat" => cat(command),
			"imap" => imap(command),
			"cd" => cd(command),
			"touch" => touch(command),
			"mkdir" => mkdir(command),
			"pwd" => pwd(),
			"test" => test(),
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
	crate::kprintln!(
		"[pwd]   INODE: {:>6}  PATH: {}",
		unsafe { CURRENTDIR_INODE },
		unsafe { CStr::from_bytes_until_nul(&PWD).unwrap().to_str().unwrap() }
	);
	crate::kprintln!("[root]  INODE: {:>6}  PATH: /", ROOT_INODE);
}

fn mkdir(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs mkdir DIR");
		return;
	}
	ext2::create_dir(command[1].as_str(), unsafe { CURRENTDIR_INODE });
}

fn touch(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs touch FILE");
		return;
	}
	ext2::create_file(command[1].as_str(), unsafe { CURRENTDIR_INODE });
}

fn cat(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs cat FILE");
		return;
	}
	let file_content = ext2::get_file_content(command[1].as_str(), unsafe {
		CURRENTDIR_INODE
	});
	for i in file_content {
		crate::kprint!("{}", i);
	}
}

fn test() {
	let mut ext2 = ext2::Ext2::new(unsafe { ext2::DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	// let mut dentry = crate::fs::ext2::inode::Dentry::default();

	let _node = ext2.alloc_node(0);
	let _block = ext2.alloc_block(0);
	crate::dprintln!("Node {}", _node);
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
	let path = Path::new(path);
	let ext2 = ext2::Ext2::new(unsafe { ext2::DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
	let lookup = ext2.recurs_find(path.as_str(), unsafe { CURRENTDIR_INODE });
	match lookup {
		None => crate::kprintln!("Dir not found"),
		Some((inodeno, inode)) => {
			if inode.is_dir() {
				unsafe {
					CURRENTDIR_INODE = inodeno;
					let mut pwd = CStr::from_bytes_until_nul(&PWD)
						.unwrap()
						.to_str()
						.unwrap()
						.to_string();
					if path.has_root() {
						pwd = path.as_str().to_string();
					} else {
						pwd.push_str("/");
						pwd.push_str(path.as_str());
					}
					let mut pwd = Path::new(&pwd);
					pwd.cleanup();
					PWD[0..pwd.as_str().len()]
						.clone_from_slice(pwd.as_str().as_bytes());
					PWD[pwd.as_str().len()] = b'\0';
				};
			} else {
				crate::kprintln!("Error: {} is not a directory", path.as_str());
			}
		},
	};
}

fn imap(command: Vec<String>) {
	let path = match command.len() {
		1 => "/",
		_ => command[1].as_str()
	};
	let ext2 = ext2::Ext2::new(unsafe { ext2::DISKNO as u8 })
		.expect("Disk is not a ext2 filesystem.");
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
