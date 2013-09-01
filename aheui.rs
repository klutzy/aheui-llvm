extern mod rustc;

use rustc::lib::llvm::{ContextRef, BuilderRef, BasicBlockRef, ValueRef};
use rustc::lib::llvm::{ModuleRef, TypeRef};
use rustc::lib::llvm::llvm;

#[deriving(Eq)]
pub enum Cho {
    cㄱ, cㄲ, cㄴ, cㄷ, cㄸ, cㄹ, cㅁ, cㅂ,
    cㅃ, cㅅ, cㅆ, cㅇ, cㅈ, cㅉ, cㅊ, cㅋ,
    cㅌ, cㅍ, cㅎ, cNone
}

#[deriving(Eq)]
pub enum Jung {
    ㅏ, ㅐ, ㅑ, ㅒ, ㅓ, ㅔ, ㅕ, ㅖ,
    ㅗ, ㅘ, ㅙ, ㅚ, ㅛ, ㅜ, ㅝ, ㅞ,
    ㅟ, ㅠ, ㅡ, ㅢ, ㅣ, juNone
}

#[deriving(Eq)]
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

#[deriving(Eq)]
struct Hangul {
    cho: Cho,
    jung: Jung,
    jong: Jong,
}

impl Hangul {
    fn none() -> Hangul {
        Hangul {
            cho: cNone,
            jung: juNone,
            jong: joNone,
        }
    }

    fn from_char(c: char) -> Hangul {
        let c = c as uint;
        let ga = '가' as uint;
        if (c < ga) {
            return Hangul::none();
        }
        let c = c - ga;
        let cho = c / 28 / 21;
        let jung = (c / 28) % 21;
        let jong = c % 28;
        unsafe {
            Hangul {
                cho: std::cast::transmute(cho),
                jung: std::cast::transmute(jung),
                jong: std::cast::transmute(jong),
            }
        }
    }
}

fn bb_name(x: uint, y: uint) -> ~str {
    fmt!("aheui_bb_%u_%u", x, y)
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
        let this_bb = do bb_name(x, y).with_c_str |buf| {
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
    fn gen_bb(&self, a: &Aheui) {
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(self.bld, self.bb);
        }
        match self.h.cho {
            cㅂ => match self.h.jong {
                jㅇ => {
                    a.call_rt(a.rt.gi, [], "gi_ret");
                },
                jㅎ => {
                    a.call_rt(a.rt.gc, [], "gc_ret");
                },
                _ => fail!("unimplemented: %?", self.h.jong),
            },
            cㅇ => {
            },
            cㅎ => {
                unsafe {
                    llvm::LLVMBuildRetVoid(self.bld);
                }
            },
            _ => {
                fail!("unimplemented: %?", self.h.cho);
            },
        }

        let (nx, ny) = match self.h.jung {
            ㅏ => {
                a.next_pos(self.x, self.y, FlowRight)
            },
            ㅓ => {
                a.next_pos(self.x, self.y, FlowLeft)
            },
            _ => {
                fail!("unimplemented: %?", self.h.jung)
            },
        };
        let dest_bb = a.get_bb(nx, ny);
        unsafe {
            if self.h.cho != cㅎ {
                llvm::LLVMBuildBr(self.bld, dest_bb);
            }
        };
    }
}

struct AheuiRt {
    gc: ValueRef,
    pc: ValueRef,
    gi: ValueRef,
    pi: ValueRef,
}

struct Aheui {
    b: ~[~[~AheuiBlock]],
    cx: ContextRef,
    bld: BuilderRef,
    mf: ValueRef,
    md: ModuleRef,
    rt: AheuiRt,
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

        let rt = AheuiRt {
            gc: gc_fn,
            pc: pc_fn,
            gi: gi_fn,
            pi: pi_fn,
        };

        let mut ah = Aheui {
            b: ~[],
            cx: cx,
            bld: bld,
            mf: mf,
            md: md,
            rt: rt,
        };

        let b_pos = "aheui_top";
        let main_bb = do b_pos.with_c_str |buf| {
            unsafe { llvm::LLVMAppendBasicBlockInContext(cx, ah.mf, buf) }
        };

        for (y, line) in h.iter().enumerate() {
            let mut bl = ~[];
            for (x, hangul) in line.iter().enumerate() {
                let block = ~AheuiBlock::new(*hangul, x, y, cx, bld, ah.mf);
                bl.push(block);
            }
            ah.b.push(bl);
        }

        let start_bb = ah.get_bb(0, 0);
        unsafe {
            llvm::LLVMPositionBuilderAtEnd(bld, main_bb);
            llvm::LLVMBuildBr(ah.bld, start_bb);
        }

        ah
    }

    #[fixed_stack_segment]
    fn gen_llvm(&self) {
        for bl in self.b.iter() {
            for b in bl.iter() {
                b.gen_bb(self);
            }
        }
    }

    fn get_bb(&self, x: uint, y: uint) -> BasicBlockRef {
        self.b[y][x].bb
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

#[fixed_stack_segment]
fn main() {
    let args = std::os::args();
    let alen = args.len();
    if alen != 2 {
        println(fmt!("Usage: %s INPUT", args[0]));
        return;
    }

    let md_name = args[1];
    let fn_name = "aheui_main";

    let path = Path(md_name);
    let code = std::io::read_whole_file_str(&path).unwrap();
    let mut code_iter = do code.line_iter().map |line| {
        line.iter().map(Hangul::from_char).collect::<~[Hangul]>()
    };
    let code = code_iter.collect();
    let aheui = Aheui::new(code, md_name, fn_name);
    aheui.gen_llvm();

    let out_f = md_name + ".ll";
    aheui.print_module(out_f);
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
