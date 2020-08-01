// #[macro_use]
// extern crate bitflags;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Simplest function to test if bindings are working and correclty linked
    #[test]
    fn swap_u256() {
        let mut input_vec = vec![0; 256];
        let ptr: *mut u8 = input_vec.as_mut_ptr();
        unsafe {
            swap_u256_bytes(ptr);
        }
    }
}
