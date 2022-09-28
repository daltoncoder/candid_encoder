mod utils;

use candid::{check_prog, IDLArgs, TypeEnv};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate(prog: &str) -> Option<Vec<u8>> {
    let args = prog.parse::<IDLArgs>().ok()?;
    let mut env = TypeEnv::new();
    let encoded_args = args.to_bytes_with_types(&env, &args.get_types()).ok()?;
    Option::from(encoded_args)
}
