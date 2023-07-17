use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::cli::commands::hexdump;

static mut CURRENTDIR_INODE: usize = 2;

pub fn debugfs(mut command: Vec<String>) {
	command.remove(0); // Delete coommand name before sending to subcommand
	match command[0].as_str() {
		"ls" => ls(command),
		"cat" => cat(command),
		"imap" => imap(command),
		"cd" => cd(command),
		"test" => test(command),
		_ => crate::kprintln!("Unknown command: {}", command[0])
	}
}

fn cat(command: Vec<String>) {
	let file_content = crate::fs::ext2::get_file_content(command[1].as_str());
	for i in file_content {
		crate::kprint!("{}", i);
	}
}

fn test(command: Vec<String>) {
	let mut ext2 = crate::fs::ext2::Ext2::new();
	//let mut dentry = crate::fs::ext2::inode::Dentry::default();
    let mut bmap = ext2.read_block_map(0);
    let mut imap = ext2.read_inode_map(0);
    crate::kprintln!("Space {} {}", bmap.get_space().0, bmap.get_space().1);
    crate::kprintln!("Space {} {}", imap.get_space().0, imap.get_space().1);
    let dentry_block = bmap.get_free_node().unwrap();
    let dentry_inode = imap.get_free_node().unwrap();
    crate::kprintln!("Free node {} {}", dentry_inode, dentry_block);

    ext2.write_block_map(0, bmap);
    ext2.write_inode_map(0, imap);

	//dentry.name = String::from("hell");
	//dentry.name_length = dentry.name.len() as u8;
	//dentry.r#type = crate::fs::ext2::inode::Dtype::Directory as u8;
	//dentry.dentry_size = crate::utils::math::roundup(8 + dentry.name_length as u16, 4);
	//ext2.add_dentry(2, dentry);
}

fn ls(command: Vec<String>) {
	let path = match command.len() {
		1 => "",
		_ => command[1].as_str()
	};
	crate::dprintln!("Ls: {}", path);
	let dentries = crate::fs::ext2::list_dir(path, unsafe { CURRENTDIR_INODE });

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
	let ext2 = crate::fs::ext2::Ext2::new();
	let lookup = ext2.recurs_find(path, unsafe { CURRENTDIR_INODE });
	match lookup {
		None => crate::kprintln!("Dir not found"),
		Some((inodeno, inode)) => {
			if inode.is_dir() {
				unsafe { CURRENTDIR_INODE = inodeno };
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
	let ext2 = crate::fs::ext2::Ext2::new();
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
