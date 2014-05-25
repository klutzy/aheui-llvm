#![crate_type = "lib"]
#![crate_id = "rt"]
#![feature(phase)]

#[phase(syntax, link)] extern crate log;

use std::cell::RefCell;
use std::io;

pub struct AheuiRt {
    dqs: Vec<Vec<i32>>,
}

local_data_key!(key_rt: RefCell<AheuiRt>)

#[no_mangle] #[inline(never)]
pub extern "C" fn aheui_getchar() -> char {
    let mut stdin = io::BufferedReader::new(io::stdin());
    print!("input an unicode character: ");
    let line = stdin.read_line().unwrap();
    line.as_slice().char_at(0)
}

#[no_mangle]
pub extern "C" fn aheui_putchar(c: char) {
    print!("{:c}", c);
}

#[no_mangle]
pub extern "C" fn aheui_getint() -> i32 {
    let mut stdin = io::BufferedReader::new(io::stdin());
    print!("input an integer: ");
    let line = stdin.read_line().unwrap();
    println!("line: {:?}", line);
    from_str(line.as_slice()).unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_putint(i: i32) {
    debug!("aheui_putint({:?})", i);
    print!("{:d}", i as int);
}

#[no_mangle]
pub extern "C" fn aheui_trace(x: i32, y: i32, c: char) {
    debug!("trace({:c}: {:d}, {:d})", c, x as int, y as int);
}

#[no_mangle]
pub extern "C" fn aheui_push(idx: i8, v: i32) {
    debug!("aheui_push(idx {:d}, val {:d})", idx as int, v as int);
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    match idx {
        27 => fail!("Aheui extension is not supported."),
        _ => {
            ar.dqs.get_mut(idx as uint).push(v);
        },
    }
    debug!("aheui_push: stack[{:d}]: {:?}", idx as int, ar.dqs.get(idx as uint).as_slice());
}

#[no_mangle]
pub extern "C" fn aheui_pop(idx: i8) -> i32 {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as uint;
    let ret = match idx {
        27 => fail!("Aheui extension is not supported."),
        21 => {
            ar.dqs.get_mut(i).shift()
        },
        _ => {
            ar.dqs.get_mut(i).pop()
        },
    };
    debug!("aheui_pop: stack[{:d}]: {:?}", idx as int, ar.dqs.get(i).as_slice());
    ret.unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_dup(idx: i8) {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as uint;
    match idx {
        27 => fail!("Aheui extension is not supported."),
        21 => {
            let dqs = ar.dqs.get_mut(i);
            let n = *dqs.get(0);
            dqs.unshift(n);
        },
        _ => {
            let dqs = ar.dqs.get_mut(i);
            let len = dqs.len();
            let n = *dqs.get(len - 1);
            dqs.push(n);
        },
    }
    debug!("aheui_dup: stack[{:d}]: {:?}", idx as int, ar.dqs.get(i).as_slice());
}

#[no_mangle]
pub extern "C" fn aheui_swap(idx: i8) {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as uint;
    match idx {
        27 => fail!("Aheui extension is not supported."),
        21 => {
            let dqs = ar.dqs.get_mut(i).as_mut_slice();
            let len = dqs.len();
            assert!(len >= 2);
            let m = dqs[0];
            let n = dqs[1];
            dqs[0] = n;
            dqs[1] = m;
        },
        _ => {
            let dqs = ar.dqs.get_mut(i).as_mut_slice();
            let len = dqs.len();
            assert!(len >= 2);
            let m = dqs[len - 2];
            let n = dqs[len - 1];
            dqs[len - 2] = n;
            dqs[len - 1] = m;
        },
    }
    debug!("aheui_swap: stack[{:d}]: {:?}", idx as int, ar.dqs.get(i).as_slice());
}

pub fn rt_init() {
    let mut dqs = Vec::new();
    for _ in range(0, 26) {
        dqs.push(Vec::new());
    }
    let ar = AheuiRt {
        dqs: dqs,
    };

    key_rt.replace(Some(RefCell::new(ar)));
}
