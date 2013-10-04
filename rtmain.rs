extern mod rt;
use rt::*;

extern "C" {
    fn aheui_main();
}

#[fixed_stack_segment]
fn main() {
    rt_init();

    unsafe {
        aheui_main();
    }
}
