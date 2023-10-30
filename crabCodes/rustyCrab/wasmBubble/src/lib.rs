mod bubble;

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wee_alloc;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use bubble::sort_array;