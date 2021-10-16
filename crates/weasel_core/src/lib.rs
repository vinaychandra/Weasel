#![cfg_attr(not(test), no_std)]

pub mod module;

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
