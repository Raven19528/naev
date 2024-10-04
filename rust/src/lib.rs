
#![allow(unused_doc_comments)]

/// cbindgen:ignore
extern "C" {
    fn SDL_GetTicks() -> i32;
}

fn ticks() -> i32 {
    return unsafe{SDL_GetTicks()};
}

extern crate colour;
use colour::*;

#[no_mangle]
extern "C" fn greet() {
    green_ln!("Hello from Rust code! {}", ticks());
}
