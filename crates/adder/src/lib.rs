#![cfg_attr(feature = "cross-compiled", no_std)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2,2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_well() {
        let result = add(5,5);
        assert_eq!(result, 10);
    }
}
