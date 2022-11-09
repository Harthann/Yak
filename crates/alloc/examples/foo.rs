extern crate alloc;

use alloc::add;

pub fn main() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        println!("{result}");
}
