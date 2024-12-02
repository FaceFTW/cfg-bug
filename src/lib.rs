#[cfg(target_arch = "wasm32")]
pub fn foo() -> u32 {
    42
}

#[cfg(not(target_arch = "wasm32"))]
pub fn foo() -> u32 {
    420 //Not the source of the bug but helps check if conditional compilation is respected at this level
}
