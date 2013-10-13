#[feature(globs)];

extern mod extra;
extern mod rustc;

use std::libc::{c_uint, c_ulonglong};

use extra::getopts;

use rustc::lib::llvm::{ContextRef, BuilderRef, BasicBlockRef, ValueRef};
use rustc::lib::llvm::{ModuleRef, TypeRef};
use rustc::lib::llvm::{IntULE, IntEQ};
use rustc::lib::llvm::True;
use rustc::lib::llvm::llvm;

#[deriving(Eq, FromPrimitive)]
pub enum Cho {
    cㄱ, cㄲ, cㄴ, cㄷ, cㄸ, cㄹ, cㅁ, cㅂ,
    cㅃ, cㅅ, cㅆ, cㅇ, cㅈ, cㅉ, cㅊ, cㅋ,
    cㅌ, cㅍ, cㅎ, cNone
}

#[deriving(Eq, FromPrimitive)]
pub enum Jung {
    ㅏ, ㅐ, ㅑ, ㅒ, ㅓ, ㅔ, ㅕ, ㅖ,
    ㅗ, ㅘ, ㅙ, ㅚ, ㅛ, ㅜ, ㅝ, ㅞ,
    ㅟ, ㅠ, ㅡ, ㅢ, ㅣ, juNone
}

#[deriving(Eq, FromPrimitive)]
pub enum Jong {
    joNone,
    jㄱ, jㄲ, jㄳ, jㄴ, jㄵ, jㄶ, jㄷ, jㄹ,
    jㄺ, jㄻ, jㄼ, jㄽ, jㄾ, jㄿ, jㅀ, jㅁ,
    jㅂ, jㅄ, jㅅ, jㅆ, jㅇ, jㅈ, jㅊ, jㅋ,
    jㅌ, jㅍ, jㅎ
}

impl Jong {
    fn val(&self) -> uint {
        if *self == jㅇ || *self == jㅎ {
            fail!("Jong::val(ㅇ or ㅎ)");
        }

        let map = [
            0,
            2, 4, 4, 2, 5, 5, 3, 5,
            7, 9, 9, 7, 9, 9, 8, 4,
            4, 6, 2, 4, 0, 3, 4, 3,
            4, 4, 0,
        ];
        map[*self as uint]
    }
}

#[deriving(Eq)]
pub enum Flow {
    FlowLeft,
    FlowRight,
    FlowUp,
    FlowDown,
}

impl Flow {
    fn reverse(&self) -> Flow {
        match *self {
            FlowLeft => FlowRight,
            FlowRight => FlowLeft,
            FlowUp => FlowDown,
            FlowDown => FlowUp,
        }
    }

    fn from_jung(jung: Jung) -> Option<Flow> {
        match jung {
            ㅏ | ㅑ => Some(FlowRight),
            ㅓ | ㅕ => Some(FlowLeft),
            ㅗ | ㅛ => Some(FlowUp),
            ㅜ | ㅠ => Some(FlowDown),
            _ => None,
        }
    }
}

#[deriving(Eq)]
struct Hangul {
    cho: Cho,
    jung: Jung,
    jong: Jong,
    c: char,
}

impl Hangul {
    fn none() -> Hangul {
        Hangul {
            cho: cNone,
            jung: juNone,
            jong: joNone,
            c: '?',
        }
    }

    fn from_char(c: char) -> Hangul {
        let u = c as uint;
        let ga = '가' as uint;
        let hih = '힣' as uint;
        if (u < ga || u > hih) {
            return Hangul::none();
        }
        let u = u - ga;
        let cho = u / 28 / 21;
        let jung = (u / 28) % 21;
        let jong = u % 28;
        Hangul {
            cho: std::num::from_uint(cho).unwrap(),
            jung: std::num::from_uint(jung).unwrap(),
            jong: std::num::from_uint(jong).unwrap(),
            c: c,
        }
    }
}

struct AheuiBlock {
    h: Hangul,
    x: uint,
    y: uint,
    cx: ContextRef,
    bld: BuilderRef,
    bb: BasicBlockRef,
}

impl AheuiBlock {
    #[fixed_stack_segment]
    fn new(
        h: Hangul, x: uint, y: uint, cx: ContextRef,
        bld: BuilderRef, main_fn: ValueRef
    ) -> AheuiBlock {
        let this_bb = do fmt!("aheui_bb_%u_%u", x, y).with_c_str |buf| {
            unsafe { llvm::LLVMAppendBasicBlockInContext(cx, main_fn, buf) }
        };
        AheuiBlock {
            h: h,
            x: x,
            y: y,
            cx: cx,
            bld: bld,
            bb: this_bb,
        }
    }

    #[fixed_stack_segment]
    fn aheui_trace(&self, a: &Aheui) {
        let (x, y) = (self.x, self.y);
        let (x, y) = (x as c_ulonglong, y as c_ulonglong);
        let (x, y) = unsafe {
            let x = llvm::LLVMConstInt(a.ty.i32_ty, x, 0);
            let y = llvm::LLVMConstInt(a.ty.i32_ty, y, 0);
            (x, y)
        };
        let h = a.b.get_hangul(self.x, self.y);
        let c = h.c as c_ulonglong;
        let c = unsafe { llvm::LLVMConstInt(a.ty.i32_ty, c, 0) };
        let args = [x, y, c];
        a.call_rt(a.rt.tr, args, "");
    }

    #[fixed_stack_segment]
    fn gen_bb(&self, a: &Aheui) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.bld, self.bb);
        }
        self.aheui_trace(a);

        let f = 0;
        let comp_init = unsafe { llvm::LLVMConstInt(a.ty.i1_ty, f, 0) };
        unsafe { llvm::LLVMBuildStore(a.bld, comp_init, a.comp); }

        let cur = a.load(a.cur, "cur");
        match self.h.cho {
            cㄴ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let ret = do "ret".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildUDiv(a.bld, v2, v1, buf) }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㄷ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let ret = do "ret".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildAdd(a.bld, v1, v2, buf) }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㄸ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let ret = do "ret".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildMul(a.bld, v1, v2, buf) }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㄹ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let ret = do "ret".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildURem(a.bld, v2, v1, buf) }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㅁ => {
                let ret = a.call_rt(a.rt.po, [cur], "");
                match self.h.jong {
                    jㅇ => {
                        a.call_rt(a.rt.pi, [ret], "");
                    },
                    jㅎ => {
                        a.call_rt(a.rt.pc, [ret], "");
                    },
                    _ => {},
                }
            },
            cㅂ => {
                let ret = match self.h.jong {
                    jㅇ => {
                        a.call_rt(a.rt.gi, [], "ret")
                    },
                    jㅎ => {
                        a.call_rt(a.rt.gc, [], "ret")
                    },
                    _ => {
                        let j = self.h.jong.val() as c_ulonglong;
                        unsafe { llvm::LLVMConstInt(a.ty.i32_ty, j, 0) }
                    }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㅃ => {
                a.call_rt(a.rt.du, [cur], "");
            },
            cㅅ => {
                let j = self.h.jong as c_ulonglong;
                let ncur = unsafe { llvm::LLVMConstInt(a.ty.i8_ty, j, 0) };
                unsafe { llvm::LLVMBuildStore(a.bld, ncur, a.cur); }
            },
            cㅆ => {
                let v = a.call_rt(a.rt.po, [cur], "");
                let j = self.h.jong as c_ulonglong;
                let ncur = unsafe { llvm::LLVMConstInt(a.ty.i8_ty, j, 0) };
                a.call_rt(a.rt.pu, [ncur, v], "");
            },
            cㅈ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let op = IntULE as c_uint;
                let cmp = do "cmp".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildICmp(a.bld, op, v1, v2, buf) }
                };
                let ret = do "ret".with_c_str |buf| {
                    unsafe {
                        llvm::LLVMBuildZExt(a.bld, cmp, a.ty.i32_ty, buf)
                    }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㅊ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = unsafe { llvm::LLVMConstInt(a.ty.i32_ty, 0, 0) };
                let op = IntEQ as c_uint;
                let cmp = do "cmp".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildICmp(a.bld, op, v1, v2, buf) }
                };
                unsafe {
                    llvm::LLVMBuildStore(a.bld, cmp, a.comp);
                }
            },
            cㅌ => {
                let v1 = a.call_rt(a.rt.po, [cur], "");
                let v2 = a.call_rt(a.rt.po, [cur], "");
                let ret = do "ret".with_c_str |buf| {
                    unsafe { llvm::LLVMBuildSub(a.bld, v2, v1, buf) }
                };
                a.call_rt(a.rt.pu, [cur, ret], "");
            },
            cㅍ => {
                a.call_rt(a.rt.sw, [cur], "");
            },
            cㅎ => {
                unsafe {
                    llvm::LLVMBuildRetVoid(self.bld);
                }
                return;
            },
            cㄱ | cㄲ | cㅇ | cㅉ | cㅋ | cNone => {
                // nothing
            },
        }

        #[fixed_stack_segment]
        fn set_next(a: &Aheui, _ab: &AheuiBlock, nf: Jung) {
            let j = match nf {
                ㅣ => a.nfs[0],
                ㅡ => a.nfs[1],
                ㅢ => a.nfs[2],
                _ => fail!("???"),
            };
            let v = a.load(a.fl, "aheui_flow_orig");
            unsafe {
                do "tmp".with_c_str |b| {
                    let z = 0 as c_ulonglong;
                    let c0 = llvm::LLVMConstInt(a.ty.i8_ty, z, 0);
                    let tmp = ~[c0, v];
                    let nv = do tmp.as_imm_buf |tmp, n| {
                        llvm::LLVMBuildGEP(a.bld, j, tmp, n as c_uint, b)
                    };
                    let nv = a.load(nv, "aheui_flow_nv");
                    llvm::LLVMBuildStore(a.bld, nv, a.fl);
                }
            }
        }

        #[fixed_stack_segment]
        fn jmp_next(a: &Aheui, ab: &AheuiBlock) {
            let nps = [
                a.next_pos(ab.x, ab.y, FlowLeft),
                a.next_pos(ab.x, ab.y, FlowRight),
                a.next_pos(ab.x, ab.y, FlowUp),
                a.next_pos(ab.x, ab.y, FlowDown),
            ].map(|&(nx, ny)| a.b.get_bb(nx, ny));

            let r = a.load(a.fl, "aheui_flow_v");
            let sw = unsafe {
                llvm::LLVMBuildSwitch(a.bld, r, nps[3], 3 as c_uint)
            };
            for (i, nbb) in nps.iter().take(3).enumerate() {
                unsafe {
                    let j = i as c_ulonglong;
                    let c = llvm::LLVMConstInt(a.ty.i8_ty, j, 0);
                    llvm::LLVMAddCase(sw, c, *nbb);
                }
            }
        }

        let comp = a.load(a.comp, "comp_v");
        match self.h.jung {
            ㅏ | ㅓ | ㅗ | ㅜ | ㅑ | ㅕ | ㅛ | ㅠ => {
                let flow = Flow::from_jung(self.h.jung).unwrap();

                let next_pos = |flow| {
                    match self.h.jung {
                        ㅏ | ㅓ | ㅗ | ㅜ => a.next_pos(self.x, self.y, flow),
                        ㅑ | ㅕ | ㅛ | ㅠ => {
                            let (nx, ny) = a.next_pos(self.x, self.y, flow);
                            a.next_pos(nx, ny, flow)
                        },
                        _ => fail!("???"),
                    }
                };
                let (nx, ny) = next_pos(flow);
                let (rx, ry) = next_pos(flow.reverse());
                let dest_bb = a.b.get_bb(nx, ny);
                let r_dest_bb = a.b.get_bb(rx, ry);

                let fl = flow as c_ulonglong;
                unsafe {
                    let fl_i8 = llvm::LLVMConstInt(a.ty.i8_ty, fl, 0);
                    llvm::LLVMBuildStore(a.bld, fl_i8, a.fl);
                }
                unsafe {
                    llvm::LLVMBuildCondBr(a.bld, comp, r_dest_bb, dest_bb);
                };
            },
            _ => {
                // TODO: flow rework
                if self.h.cho == cㅊ {
                    fail!("unimplemented: %?", self.h.cho);
                }

                match self.h.jung {
                    ㅣ | ㅡ | ㅢ => {
                        set_next(a, self, self.h.jung);
                    },
                    _ => {},
                }
                jmp_next(a, self);
            },
        }
    }
}

struct AheuiRt {
    gc: ValueRef,
    pc: ValueRef,
    gi: ValueRef,
    pi: ValueRef,
    tr: ValueRef,
    pu: ValueRef,
    po: ValueRef,
    du: ValueRef,
    sw: ValueRef,
}

type AheuiMapImpl = ~[~[~AheuiBlock]];

trait AheuiMap {
    fn get_bb(&self, x: uint, y: uint) -> BasicBlockRef;
    fn get_hangul(&self, x: uint, y: uint) -> Hangul;
}

impl AheuiMap for AheuiMapImpl {
    fn get_bb(&self, x: uint, y: uint) -> BasicBlockRef {
        self[y][x].bb
    }

    fn get_hangul(&self, x: uint, y: uint) -> Hangul {
        self[y][x].h
    }
}

struct Types {
    i1_ty: TypeRef,
    i8_ty: TypeRef,
    i32_ty: TypeRef,
    void_ty: TypeRef,
}

struct Aheui {
    b: AheuiMapImpl,
    cx: ContextRef,
    bld: BuilderRef,
    mf: ValueRef,
    md: ModuleRef,
    rt: AheuiRt,
    fl: ValueRef,
    cur: ValueRef,
    comp: ValueRef,
    nfs: ~[ValueRef],
    ty: Types,
}

#[fixed_stack_segment]
fn new_var(bld: BuilderRef, v: u8, ty: TypeRef, name: &str) -> ValueRef {
    let var = do name.with_c_str |buf| {
        unsafe { llvm::LLVMBuildAlloca(bld, ty, buf) }
    };
    unsafe {
        let c = llvm::LLVMConstInt(ty, v as c_ulonglong, 0);
        llvm::LLVMBuildStore(bld, c, var);
    }
    var
}

impl Aheui {
    fn next_pos(&self, x: uint, y: uint, flow: Flow) -> (uint, uint) {
        match flow {
            FlowLeft | FlowRight => {
                let len = self.b[y].len();
                let next_x = if flow == FlowLeft {
                    (len + x - 1) % len
                } else {
                    (x + 1) % len
                };
                return (next_x, y);
            }
            FlowUp | FlowDown => {
                if flow == FlowDown {
                    let ly = y + 1;
                    let it = self.b.iter().enumerate().skip(ly);
                    let it = it.chain(self.b.iter().enumerate().take(ly));
                    let mut it = it;
                    for (cur_y, line) in it {
                        if x < line.len() {
                            return (x, cur_y);
                        }
                    }
                    fail!("Failed to find next position");
                } else {
                    let ly = self.b.len() - y;
                    let it = self.b.rev_iter().enumerate().skip(ly);
                    let it = it.chain(self.b.rev_iter().enumerate().take(ly));
                    let mut it = it;
                    for (cur_y, line) in it {
                        if x < line.len() {
                            return (x, self.b.len() - 1 - cur_y);
                        }
                    }
                    fail!("Failed to find next position");
                }
            }
        }
    }

    #[fixed_stack_segment]
    fn load(&self, val: ValueRef, name: &str) -> ValueRef {
        do name.with_c_str |buf| {
            unsafe { llvm::LLVMBuildLoad(self.bld, val, buf) }
        }
    }

    #[fixed_stack_segment]
    fn call_rt(&self, f: ValueRef, args: &[ValueRef], n: &str) -> ValueRef {
        do n.with_c_str |buf| {
            unsafe {
                llvm::LLVMBuildCall(
                    self.bld, f, std::vec::raw::to_ptr(args),
                    args.len() as std::libc::c_uint, buf
                )
            }
        }
    }

    #[fixed_stack_segment]
    fn new(h: ~[~[Hangul]], md_name: &str, fn_name: &str) -> Aheui {
        let cx = unsafe { llvm::LLVMContextCreate() };
        let md = do md_name.with_c_str |buf| {
            unsafe { llvm::LLVMModuleCreateWithNameInContext(buf, cx) }
        };
        let bld = unsafe { llvm::LLVMCreateBuilderInContext(cx) };

        let i1_ty = unsafe { llvm::LLVMInt1TypeInContext(cx) };
        let i8_ty = unsafe { llvm::LLVMInt8TypeInContext(cx) };
        let i32_ty = unsafe { llvm::LLVMInt32TypeInContext(cx) };
        let void_ty = unsafe { llvm::LLVMVoidTypeInContext(cx) };

        #[fixed_stack_segment]
        fn declare_fn(md: ModuleRef, n: &str, ty: TypeRef) -> ValueRef {
            do n.with_c_str |buf| {
                unsafe { llvm::LLVMGetOrInsertFunction(md, buf, ty) }
            }
        }

        #[fixed_stack_segment]
        fn fn_ty(rt: TypeRef, par: &[TypeRef]) -> TypeRef {
            unsafe {
                llvm::LLVMFunctionType(
                    rt, std::vec::raw::to_ptr(par),
                    par.len() as std::libc::c_uint, 0
                )
            }
        }

        let main_ty = fn_ty(void_ty, []);
        let mf = do fn_name.with_c_str |buf| {
            unsafe { llvm::LLVMAddFunction(md, buf, main_ty) }
        };

        // declare runtime functions
        // extern "C" fn aheui_getchar() -> char
        let gc_fn_ty = fn_ty(i32_ty, []);
        let gc_fn = declare_fn(md, "aheui_getchar", gc_fn_ty);

        // extern "C" fn aheui_putchar(c: char)
        let pc_fn_ty = fn_ty(void_ty, [i32_ty]);
        let pc_fn = declare_fn(md, "aheui_putchar", pc_fn_ty);

        // extern "C" fn aheui_getint() -> i32
        let gi_fn_ty = fn_ty(i32_ty, []);
        let gi_fn = declare_fn(md, "aheui_getint", gi_fn_ty);

        // extern "C" fn aheui_putint(i: i32)
        let pi_fn_ty = fn_ty(void_ty, [i32_ty]);
        let pi_fn = declare_fn(md, "aheui_putint", pi_fn_ty);

        // extern "C" fn aheui_trace(x: i32, y: i32, c: char)
        let tr_fn_ty = fn_ty(void_ty, [i32_ty, i32_ty, i32_ty]);
        let tr_fn = declare_fn(md, "aheui_trace", tr_fn_ty);

        // extern "C" fn aheui_push(idx: i8, v: i32)
        let pu_fn_ty = fn_ty(void_ty, [i8_ty, i32_ty]);
        let pu_fn = declare_fn(md, "aheui_push", pu_fn_ty);

        // extern "C" fn aheui_pop(idx: i8) -> i32
        let po_fn_ty = fn_ty(i32_ty, [i8_ty]);
        let po_fn = declare_fn(md, "aheui_pop", po_fn_ty);

        // extern "C" fn aheui_dup(idx: i8)
        let du_fn_ty = fn_ty(void_ty, [i8_ty]);
        let du_fn = declare_fn(md, "aheui_dup", du_fn_ty);

        // extern "C" fn aheui_swap(idx: i8)
        let sw_fn_ty = fn_ty(void_ty, [i8_ty]);
        let sw_fn = declare_fn(md, "aheui_swap", sw_fn_ty);

        let rt = AheuiRt {
            gc: gc_fn,
            pc: pc_fn,
            gi: gi_fn,
            pi: pi_fn,
            tr: tr_fn,
            pu: pu_fn,
            po: po_fn,
            du: du_fn,
            sw: sw_fn,
        };

        let b_pos = "aheui_top";
        let main_bb = do b_pos.with_c_str |buf| {
            unsafe { llvm::LLVMAppendBasicBlockInContext(cx, mf, buf) }
        };

        let mut b = ~[];
        for (y, line) in h.iter().enumerate() {
            let mut bl = ~[];
            for (x, hangul) in line.iter().enumerate() {
                let block = ~AheuiBlock::new(*hangul, x, y, cx, bld, mf);
                bl.push(block);
            }
            b.push(bl);
        }

        unsafe {
            llvm::LLVMPositionBuilderAtEnd(bld, main_bb);
        }

        let fl = new_var(bld, FlowDown as u8, i8_ty, "aheui_flow");
        let cur = new_var(bld, joNone as u8, i8_ty, "aheui_cur");
        let comp = new_var(bld, 0, i1_ty, "aheui_comp");

        let i8_arr_ty = unsafe { llvm::LLVMArrayType(i8_ty, 4 as c_uint) };
        let nfs = unsafe {
            let js = [
                ("\x01\x00\x02\x03", "fl0"),
                ("\x00\x01\x03\x02", "fl1"),
                ("\x01\x00\x03\x02", "fl2"),
            ];
            do js.map |&(j, n)| {
                do j.as_imm_buf |jb, l| {
                    let jb = jb as *i8;
                    let l = l as c_uint;
                    let v = do n.with_c_str |n| {
                        llvm::LLVMAddGlobal(md, i8_arr_ty, n)
                    };
                    llvm::LLVMSetGlobalConstant(v, True);
                    let c = llvm::LLVMConstStringInContext(cx, jb, l, True);
                    llvm::LLVMSetInitializer(v, c);
                    v
                }
            }
        };

        debug!("nfs: %?", nfs);

        let start_bb = b.get_bb(0, 0);
        unsafe {
            llvm::LLVMBuildBr(bld, start_bb);
        }

        Aheui {
            b: b,
            cx: cx,
            bld: bld,
            mf: mf,
            md: md,
            rt: rt,
            fl: fl,
            cur: cur,
            comp: comp,
            nfs: nfs,
            ty: Types {
                i1_ty: i1_ty,
                i8_ty: i8_ty,
                i32_ty: i32_ty,
                void_ty: void_ty,
            },
        }
    }

    #[fixed_stack_segment]
    fn gen_llvm(&self) {
        for bl in self.b.iter() {
            for b in bl.iter() {
                b.gen_bb(self);
            }
        }
    }

    #[fixed_stack_segment]
    fn print_module(&self, out_f: &str) {
        let cpm = unsafe { llvm::LLVMCreatePassManager() };
        do out_f.with_c_str |buf| {
            unsafe {
                llvm::LLVMRustPrintModule(cpm, self.md, buf)
            }
        };
    }
}

fn print_usage(prog: &str) {
    println!("Usage: {} [OPTIONS] INPUT", prog);
    println!("Options:");
    println!("\t-o [OUTPUT] (default to [INPUT].ll)");
    println!("\t-m [OUTPUT] (default to aheui_main)");
    println!("\t-h");
}

#[fixed_stack_segment]
fn main() {
    let args = std::os::args();

    let opts = ~[
        getopts::optopt("o"),
        getopts::optopt("m"),
        getopts::optflag("h"),
    ];
    let matches = match getopts::getopts(args.slice_from(1), opts) {
        Ok(a) => a,
        Err(e) => fail2!(e.to_err_msg()),
    };

    if matches.opt_present("h") {
        print_usage(args[0]);
        return;
    }

    let in_fn: &str = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(args[0]);
        return;
    };

    let out_fn = in_fn + ".ll";
    let out_fn = match matches.opt_str("o") {
        Some(o) => o,
        None => out_fn,
    };

    let fn_name = match matches.opt_str("m") {
        Some(m) => m,
        None => ~"aheui_main",
    };

    let path = Path(in_fn);
    let code = std::io::read_whole_file_str(&path).unwrap();
    let mut code_iter = do code.line_iter().map |line| {
        line.iter().map(Hangul::from_char).collect::<~[Hangul]>()
    };
    let code = code_iter.collect();
    let aheui = Aheui::new(code, in_fn, fn_name);
    aheui.gen_llvm();

    aheui.print_module(out_fn);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hangul() {
        let 가 = Hangul { cho: cㄱ, jung: ㅏ, jong: joNone };
        assert!(Hangul::from_char('가') == 가);

        let 아 = Hangul { cho: cㅇ, jung: ㅏ, jong: joNone };
        assert!(Hangul::from_char('아') == 아);

        let 힣 = Hangul { cho: cㅎ, jung: ㅣ, jong: jㅎ };
        assert!(Hangul::from_char('힣') == 힣);

        assert!(Hangul::from_char('A') == Hangul::none());
        assert!(Hangul::from_char('☆') == Hangul::none());
    }

    #[test]
    fn test_jong() {
        assert!(joNone.val() == 0);
        assert!(jㄱ.val() == 2);
        assert!(jㄿ.val() == 9);
    }

    #[test]
    fn test_next_pos() {
        let map = [
            "아희희아희",
            "아희아희",
            "아희희",
        ];
        let map = do map.map |&x| {
            x.iter().map(Hangul::from_char).collect::<~[Hangul]>()
        };
        let map = Aheui::new(map, "dummy", "dummy_main");

        assert!(map.next_pos(0, 0, FlowLeft) == (4, 0));
        assert!(map.next_pos(0, 0, FlowRight) == (1, 0));
        assert!(map.next_pos(0, 0, FlowUp) == (0, 2));
        assert!(map.next_pos(0, 0, FlowDown) == (0, 1));

        assert!(map.next_pos(4, 0, FlowLeft) == (3, 0));
        assert!(map.next_pos(4, 0, FlowRight) == (0, 0));
        assert!(map.next_pos(4, 0, FlowUp) == (4, 0));
        assert!(map.next_pos(4, 0, FlowDown) == (4, 0));

        assert!(map.next_pos(3, 1, FlowLeft) == (2, 1));
        assert!(map.next_pos(3, 1, FlowRight) == (0, 1));
        assert!(map.next_pos(3, 1, FlowUp) == (3, 0));
        assert!(map.next_pos(3, 1, FlowDown) == (3, 0));

        assert!(map.next_pos(2, 2, FlowLeft) == (1, 2));
        assert!(map.next_pos(2, 2, FlowRight) == (0, 2));
        assert!(map.next_pos(2, 2, FlowUp) == (2, 1));
        assert!(map.next_pos(2, 2, FlowDown) == (2, 0));
    }
}
