#![crate_type = "lib"]
#![crate_id = "rt"]
#![feature(phase)]

#[macro_use] extern crate log;

use std::cell::RefCell;
use std::io;

pub struct AheuiRt {
    dqs: Vec<Vec<i32>>,
}

thread_local!(static key_rt: RefCell<AheuiRt> = RefCell::new(AheuiRt { dqs: Vec::new() }));

#[no_mangle] #[inline(never)]
pub extern "C" fn aheui_getchar() -> char {
    let mut stdin = io::BufReader::new(io::stdin());
    print!("input an unicode character: ");
    let line = stdin.read_line().unwrap();
    line.as_slice().char_at(0)
}

#[no_mangle]
pub extern "C" fn aheui_putchar(c: char) {
    print!("{}", c);
}

#[no_mangle]
pub extern "C" fn aheui_getint() -> i32 {
    let mut stdin = io::BufReader::new(io::stdin());
    print!("input an integer: ");
    let line = stdin.read_line().unwrap();
    println!("line: {:?}", line);
    line.parse().unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_putint(i: i32) {
    debug!("aheui_putint({:?})", i);
    print!("{}", i);
}

#[no_mangle]
pub extern "C" fn aheui_trace(x: i32, y: i32, c: char) {
    debug!("trace({}: {}, {})", c, x, y);
}

#[no_mangle]
pub extern "C" fn aheui_push(idx: i8, v: i32) {
    debug!("aheui_push(idx {}, val {})", idx, v);
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    match idx {
        27 => panic!("Aheui extension is not supported."),
        _ => {
            ar.dqs.get_mut(idx as usize).push(v);
        },
    }
    debug!("aheui_push: stack[{}]: {:?}", idx, ar.dqs.get(idx as usize).as_slice());
}

#[no_mangle]
pub extern "C" fn aheui_pop(idx: i8) -> i32 {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as usize;
    let ret = match idx {
        27 => panic!("Aheui extension is not supported."),
        21 => {
            ar.dqs.get_mut(i).shift()
        },
        _ => {
            ar.dqs.get_mut(i).pop()
        },
    };
    debug!("aheui_pop: stack[{}]: {:?}", idx, ar.dqs.get(i).as_slice());
    ret.unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_dup(idx: i8) {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as usize;
    match idx {
        27 => panic!("Aheui extension is not supported."),
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
    debug!("aheui_dup: stack[{}]: {:?}", idx, ar.dqs.get(i).as_slice());
}

#[no_mangle]
pub extern "C" fn aheui_swap(idx: i8) {
    let cell = key_rt.get().unwrap();
    let mut ar = cell.borrow_mut();
    let i = idx as usize;
    match idx {
        27 => panic!("Aheui extension is not supported."),
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
    debug!("aheui_swap: stack[{}]: {:?}", idx, ar.dqs.get(i).as_slice());
}

pub fn rt_init() {
    let mut dqs = Vec::new();
    for _ in 0.. 26 {
        dqs.push(Vec::new());
    }
    let ar = AheuiRt {
        dqs: dqs,
    };

    key_rt.replace(Some(RefCell::new(ar)));
}
