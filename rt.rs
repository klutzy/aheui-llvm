#[no_mangle]
extern fn aheui_getchar() -> char {
    println("fake aheui_getchar(): return 0");
    '\x00'
}

#[no_mangle]
extern "C" fn aheui_putchar(c: char) {
    print(fmt!("%c", c));
}

#[no_mangle]
extern "C" fn aheui_getint() -> i32 {
    println("fake aheui_getint(): return 0");
    0i32
}

#[no_mangle]
extern "C" fn aheui_putint(i: i32) {
    print(fmt!("%d", i as int));
}

#[no_mangle]
extern "C" fn aheui_trace(x: i32, y: i32) {
    debug!("trace(%d, %d)", x as int, y as int);
}

extern "C" {
    fn aheui_main();
}

#[fixed_stack_segment]
fn main() {
    unsafe {
        aheui_main();
    }
}
