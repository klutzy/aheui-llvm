use std::local_data;

struct AheuiRt {
    dqs: ~[~[i32]],
}

local_data_key!(key_rt: AheuiRt)

#[no_mangle]
extern fn aheui_getchar() -> char {
    let stdin = std::io::stdin();
    print("input an unicode character: ");
    let line = stdin.read_line();
    line.char_at(0)
}

#[no_mangle]
extern "C" fn aheui_putchar(c: char) {
    print(fmt!("%c", c));
}

#[no_mangle]
extern "C" fn aheui_getint() -> i32 {
    let stdin = std::io::stdin();
    print("input an integer: ");
    let line = stdin.read_line();
    println(fmt!("line: %?", line));
    from_str(line).unwrap()
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

#[no_mangle]
extern "C" fn aheui_dup(idx: i8) {
    do local_data::get_mut(key_rt) |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                let n = ar.dqs[idx][0];
                ar.dqs[idx].unshift(n);
            },
            _ => {
                let n = ar.dqs[idx][ar.dqs[idx].len() - 1];
                ar.dqs[idx].push(n)
            },
        }
        debug!("aheui_dup: stack[%d]: %?", idx as int, ar.dqs[idx]);
    }
}

#[no_mangle]
extern "C" fn aheui_swap(idx: i8) {
    do local_data::get_mut(key_rt) |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                let len = ar.dqs[idx].len();
                assert!(len >= 2);
                let m = ar.dqs[idx][0];
                let n = ar.dqs[idx][1];
                ar.dqs[idx][0] = n;
                ar.dqs[idx][1] = m;
            },
            _ => {
                let len = ar.dqs[idx].len();
                assert!(len >= 2);
                let m = ar.dqs[idx][len - 2];
                let n = ar.dqs[idx][len - 1];
                ar.dqs[idx][len - 2] = n;
                ar.dqs[idx][len - 1] = m;
            },
        }
        debug!("aheui_swap: stack[%d]: %?", idx as int, ar.dqs[idx]);
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
