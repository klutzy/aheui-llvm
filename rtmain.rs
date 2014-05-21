#![feature(globs)]

extern crate rt;
use rt::*;

extern "C" {
    fn aheui_main();
}

fn main() {
    rt_init();

    unsafe {
        aheui_main();
    }
}
