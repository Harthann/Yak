use core::slice::from_raw_parts;
use core::str::from_utf8;

fn find(start: usize, end: usize, pattern: &str) {
    unsafe {

        for i in start..end {
            let tmp = from_utf8(from_raw_parts(i as *const u8, pattern.len()));
            if tmp.is_ok() && tmp.ok().is_some() {
                if tmp.ok().unwrap() == pattern {
                    crate::kprintln!("{:?}: at {:#p}", tmp.ok().unwrap(), i as *const u8);
                    return ;
                }
            }
        }
    }
}

pub fn parse() {
    find(0x000E0000, 0x000FFFFF, "RSD PTR ");
}
