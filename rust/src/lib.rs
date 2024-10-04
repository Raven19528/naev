
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

extern crate sdl2;
//use sdl2;

#[no_mangle]
extern "C" fn test_sdl() {
    let sdl = sdl2::init().unwrap();
    let t = sdl.timer().expect("Timer Failed");
    green_ln!("{}", sdl2::TimerSubsystem::ticks(&t));
}

#[no_mangle]
extern "C" fn test_c() {
    green_ln!("Hello from Rust code! {}", ticks());
}
