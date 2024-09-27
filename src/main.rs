use std::arch::asm;
use std::fmt;

#[derive(Debug)]
struct StackFrame {
    sp: usize,
    bp: usize,
    ip: usize,
    //func: String
}

impl StackFrame {
    fn new(sp: usize, bp: usize, ip: usize) -> Self {
        StackFrame {
            sp,
            bp,
            ip
        }
    }

}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            "{{\n\
                \tsp: {:#x},\n\
                \tbp: {:#x},\n\
                \tip: {:#x},\n\
            }}",
            self.sp, self.bp, self.ip)
    }
}

fn main() {
    let st = get_stacktrace();

    for frame in st {
        println!("{}", frame);
    }

    print!("debug");
}

fn get_stacktrace() -> Vec<StackFrame>{
    let mut sp = get_sp();
    let mut bp = get_bp();
    let mut ip = get_ip();
    let mut stacktrace: Vec<StackFrame> = Vec::new();

    while bp != 0 {
        let frame = StackFrame::new(sp, bp, ip);
        stacktrace.push(frame);

        unsafe {
            ip = *((bp + 8) as *const usize);
            sp = bp + 16;
            bp = *(bp as *const usize);
        };
    }

    stacktrace
}

#[inline(always)]
fn get_sp() -> usize {
    let sp: usize;

    unsafe { asm!("mov   {}, rsp", out(reg) sp); }
    sp
}

#[inline(always)]
fn get_bp() -> usize {
    let bp: usize;

    unsafe { asm!("mov   {}, rbp", out(reg) bp); }
    bp
}

#[inline(always)]
fn get_ip() -> usize {
    let ip: usize;

    unsafe { asm!("lea   {}, [rip]", out(reg) ip); }
    ip
}
