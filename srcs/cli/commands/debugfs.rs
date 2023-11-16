use crate::alloc::string::String;
use crate::alloc::vec::Vec;

use crate::fs::ext2;
use crate::spin::Mutex;
use crate::utils::path::Path;

pub static ROOT_INODE: usize = 2;
pub static CURRENTDIR_INODE: Mutex<usize> = Mutex::new(ROOT_INODE);
pub static PWD: Mutex<Option<Path>> = Mutex::new(None);
pub static DISKNO: Mutex<Option<ext2::Ext2>> = Mutex::new(None);

fn help() {
	crate::kprintln!(
		"Command available: ls,stat,cat,imap,cd,touch,mkdir,rm,pwd,test"
	);
}

pub fn debugfs(mut command: Vec<String>) {
	if command.len() > 1 {
		command.remove(0); // Delete command name before sending to subcommand
		match command[0].as_str() {
			"ls" => ls(command),
			"stat" => stat(command),
			"cat" => cat(command),
			"imap" => imap(command),
			"cd" => cd(command),
			"touch" => touch(command),
			"mkdir" => mkdir(command),
			"rm" => rm(command),
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
		*CURRENTDIR_INODE.lock(),
		PWD.lock().as_ref().unwrap_or(&Path::new("/"))
	);
	crate::kprintln!("[root]  INODE: {:>6}  PATH: /", ROOT_INODE);
}

fn rm(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs rm FILE");
		return;
	}
	ext2::remove_file(
		DISKNO.lock().as_mut().unwrap(),
		command[1].as_str(),
		*CURRENTDIR_INODE.lock()
	);
}

fn stat(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs stat FILE");
		return;
	}
	ext2::show_inode_info(
		DISKNO.lock().as_ref().unwrap(),
		command[1].as_str(),
		*CURRENTDIR_INODE.lock()
	);
}

fn mkdir(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs mkdir DIR");
		return;
	}
	ext2::create_dir(
		DISKNO.lock().as_mut().unwrap(),
		command[1].as_str(),
		*CURRENTDIR_INODE.lock()
	);
}

fn touch(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs touch FILE");
		return;
	}
	ext2::create_file(
		DISKNO.lock().as_mut().unwrap(),
		command[1].as_str(),
		*CURRENTDIR_INODE.lock()
	);
}

fn cat(command: Vec<String>) {
	if command.len() < 2 {
		crate::kprintln!("usage: debugfs cat FILE");
		return;
	}
	let file_content = ext2::get_file_content(
		DISKNO.lock().as_ref().unwrap(),
		command[1].as_str(),
		*CURRENTDIR_INODE.lock()
	);
	for i in file_content {
		crate::kprint!("{}", i);
	}
}

fn test() {
	// let binding = DISKNO.lock();
	// let ext2 = binding.as_ref().unwrap();
	// let mut dentry = crate::fs::ext2::inode::Dentry::default();
	//
	// let _node = ext2.alloc_node(0);
	// let _block = ext2.alloc_block(0);
	// crate::dprintln!("Node {}", _node);
}

fn ls(command: Vec<String>) {
	let path = match command.len() {
		1 => "",
		_ => command[1].as_str()
	};
	crate::dprintln!("Ls: {}", path);
	let dentries = ext2::list_dir(
		DISKNO.lock().as_ref().unwrap(),
		path,
		*CURRENTDIR_INODE.lock()
	);

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
	let binding = DISKNO.lock();
	let ext2 = binding.as_ref().unwrap();
	let lookup = ext2.recurs_find(path.as_str(), *CURRENTDIR_INODE.lock());
	match lookup {
		None => crate::kprintln!("Dir not found"),
		Some((inodeno, inode)) => {
			if inode.is_dir() {
				*CURRENTDIR_INODE.lock() = inodeno;
				let mut pwd;
				if path.has_root() {
					pwd = Path::new(path.as_str());
				} else {
					pwd = match (*PWD.lock()).clone() {
						Some(x) => x.join(path.as_str()),
						None => Path::new(&["/", path.as_str()].join(""))
					};
				}
				pwd.cleanup();
				*PWD.lock() = Some(pwd);
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
	let binding = DISKNO.lock();
	let ext2 = binding.as_ref().unwrap();
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
