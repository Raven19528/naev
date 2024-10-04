
#![allow(unused_doc_comments)]

/// cbindgen:ignore
extern "C" {
    fn SDL_GetTicks() -> i32;
}

fn ticks() -> i32 {
    return unsafe{SDL_GetTicks()};
}

#[no_mangle]
extern "C" fn greet() {
    println!("Hello from Rust code! {}", ticks());
}
