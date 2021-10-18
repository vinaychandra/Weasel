#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod module;
pub mod value;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "wast.pest"]
pub struct WastParser;

#[cfg(test)]
mod tests {
    use wabt::wat2wasm;

    #[test]
    fn test_wabt_load() {
        assert_eq!(
            wat2wasm("(module)").unwrap(),
            &[
                0, 97, 115, 109, // \0ASM - magic
                1, 0, 0, 0 //  0x01 - version
            ]
        );
    }
}
