#![feature(rustc_private)]

#[macro_use] extern crate log;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, stdin};

pub struct AheuiRt {
    dqs: Vec<Vec<i32>>,
}

impl AheuiRt {
    fn new() -> Self {
        let mut dqs = Vec::new();
        for _ in 0.. 26 { dqs.push(Vec::new()); }

        AheuiRt {
            dqs: dqs,
        }
    }
}

thread_local!(static KEY_RT: RefCell<AheuiRt> = RefCell::new(AheuiRt::new()));

#[no_mangle] #[inline(never)]
pub extern "C" fn aheui_getchar() -> char {
    let mut stdin = BufReader::new(stdin());
    print!("input an unicode character: ");
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    line.chars().next().unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_putchar(c: char) {
    print!("{}", c);
}

#[no_mangle]
pub extern "C" fn aheui_getint() -> i32 {
    let mut stdin = BufReader::new(stdin());
    print!("input an integer: ");
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
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
    let idx = idx as usize;

    match idx {
        27 => panic!("Aheui extension is not supported."),
        _ => {
            KEY_RT.with(|cell| {
                let mut ar = cell.borrow_mut();
                ar.dqs[idx].push(v);
                debug!("aheui_push: stack[{}]: {:?}", idx, ar.dqs[idx]);
            })
        }
    }
}

#[no_mangle]
pub extern "C" fn aheui_pop(idx: i8) -> i32 {
    let idx = idx as usize;

    KEY_RT.with(|cell| {
        let mut ar = cell.borrow_mut();
        let ret = match idx {
            27 => panic!("Aheui extension is not supported."),
            21 => ar.dqs[idx].remove(0),
            _ => ar.dqs[idx].pop().unwrap(),
        };
        debug!("aheui_pop: stack[{}]: {:?}", idx, ar.dqs[idx]);
        ret
    })
}

#[no_mangle]
pub extern "C" fn aheui_dup(idx: i8) {
    let idx = idx as usize;

    KEY_RT.with(|cell| {
        let mut ar = cell.borrow_mut();
        match idx {
            27 => panic!("Aheui extension is not supported."),
            21 => {
                let dqs = &mut ar.dqs[idx];
                let n = dqs[0];
                dqs.insert(0, n);
            },
            _ => {
                let dqs = &mut ar.dqs[idx];
                let len = dqs.len();
                let n = dqs[len - 1];
                dqs.push(n);
            },
        }
        debug!("aheui_dup: stack[{}]: {:?}", idx, ar.dqs[idx]);
    });
}

#[no_mangle]
pub extern "C" fn aheui_swap(idx: i8) {
    let idx = idx as usize;

    KEY_RT.with(|cell| {
        let mut ar = cell.borrow_mut();
        match idx {
            27 => panic!("Aheui extension is not supported."),
            21 => {
                let dqs = &mut ar.dqs[idx];
                let len = dqs.len();
                assert!(len >= 2);
                let m = dqs[0];
                let n = dqs[1];
                dqs[0] = n;
                dqs[1] = m;
            },
            _ => {
                let dqs = &mut ar.dqs[idx];
                let len = dqs.len();
                assert!(len >= 2);
                let m = dqs[len - 2];
                let n = dqs[len - 1];
                dqs[len - 2] = n;
                dqs[len - 1] = m;
            },
        }
        debug!("aheui_swap: stack[{}]: {:?}", idx, ar.dqs[idx]);
    });
}

extern "C" {
    fn aheui_main();
}

fn main() {
    unsafe { aheui_main(); }
}
