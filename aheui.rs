extern mod rustc;

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

pub trait Aheui {
    fn next_pos(&self, x: uint, y: uint, flow: Flow) -> (uint, uint);
}

impl Aheui for ~[~[Hangul]] {
    fn next_pos(&self, x: uint, y: uint, flow: Flow) -> (uint, uint) {
        match flow {
            FlowLeft | FlowRight => {
                let len = self[y].len();
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
                    let it = self.iter().enumerate().skip(ly);
                    let it = it.chain(self.iter().enumerate().take(ly));
                    let mut it = it;
                    for (cur_y, line) in it {
                        if x < line.len() {
                            return (x, cur_y);
                        }
                    }
                    fail!("Failed to find next position");
                } else {
                    let ly = self.len() - y;
                    let it = self.rev_iter().enumerate().skip(ly);
                    let it = it.chain(self.rev_iter().enumerate().take(ly));
                    let mut it = it;
                    for (cur_y, line) in it {
                        if x < line.len() {
                            return (x, self.len() - 1 - cur_y);
                        }
                    }
                    fail!("Failed to find next position");
                }
            }
        }
    }
}

#[fixed_stack_segment]
fn main() {
    let md_name = "hello.ah";
    let fn_name = "aheui_main";

    let cx = unsafe { llvm::LLVMContextCreate() };
    let md = do md_name.with_c_str |buf| {
        unsafe { llvm::LLVMModuleCreateWithNameInContext(buf, cx) }
    };
    let bld = unsafe { llvm::LLVMCreateBuilderInContext(cx) };

    let void_ty = unsafe { llvm::LLVMVoidTypeInContext(cx) };
    let main_ty = unsafe {
        llvm::LLVMFunctionType(
            void_ty, std::ptr::null(), 0 as std::libc::c_uint, 0
        )
    };

    let main_fn = do fn_name.with_c_str |buf| {
        unsafe { llvm::LLVMAddFunction(md, buf, main_ty) }
    };

    let b_pos = "top";
    let main_bb = do b_pos.with_c_str |buf| {
        unsafe { llvm::LLVMAppendBasicBlockInContext(cx, main_fn, buf) }
    };
    unsafe {
        llvm::LLVMPositionBuilderAtEnd(bld, main_bb);
        llvm::LLVMBuildRetVoid(bld);
    }

    let out_f = "hello.bc";
    do out_f.with_c_str |buf| {
        unsafe {
            llvm::LLVMWriteBitcodeToFile(md, buf)
        }
    };
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
