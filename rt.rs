use std::local_data;

struct AheuiRt {
    dqs: ~[~[i32]],
}

local_data_key!(key_rt: AheuiRt)

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

#[no_mangle]
extern "C" fn aheui_push(idx: i8, v: i32) {
    debug!("aheui_push(idx %d, val %d)", idx as int, v as int);
    do local_data::get_mut(key_rt) |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            _ => {
                ar.dqs[idx].push(v);
            },
        }
        debug!("aheui_push: stack[%d]: %?", idx as int, ar.dqs[idx]);
    }
}

#[no_mangle]
extern "C" fn aheui_pop(idx: i8) -> i32 {
    do local_data::get_mut(key_rt) |ar| {
        let ar = ar.unwrap();
        let ret = match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                ar.dqs[idx].shift()
            },
            _ => {
                ar.dqs[idx].pop()
            },
        };
        debug!("aheui_pop: stack[%d]: %?", idx as int, ar.dqs[idx]);
        ret
    }
}

extern "C" {
    fn aheui_main();
}

#[fixed_stack_segment]
fn main() {
    let mut dqs = ~[];
    for _ in range(0, 26) {
        dqs.push(~[]);
    }
    let ar = AheuiRt {
        dqs: dqs,
    };

    local_data::set(key_rt, ar);

    unsafe {
        aheui_main();
    }
}
