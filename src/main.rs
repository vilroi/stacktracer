use std::{
    arch::asm,
    fmt,
    thread,
    time,
};

const PAGE_SIZE: usize = 0x1000;

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
    let load_addr = get_loadaddr();

    for frame in st {
        println!("{}", frame);
    }

    println!("load addr: {:#x}", load_addr);
    //sleep(1000);
}

fn get_stacktrace() -> Vec<StackFrame>{
    let mut sp = get_sp();
    let mut bp = get_bp();
    let mut ip = get_ip();
    let mut stacktrace: Vec<StackFrame> = Vec::new();

    loop {
        let frame = StackFrame::new(sp, bp, ip);
        stacktrace.push(frame);

        if bp == 0 {
            break;
        }

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

fn get_loadaddr() -> usize {
    let ip = get_ip() & !(PAGE_SIZE -1);
    let mut p = ip as *const u32;
    let magic = 0x464c457f;         // ELF Magic bytes, little endian

    loop {
        unsafe {
            if *p == magic {
                return p as usize;
            }
        }
        p = (p as usize - PAGE_SIZE) as *const u32;
    }
}

fn sleep(secs: u64) {
    let dur = time::Duration::from_secs(secs);
    thread::sleep(dur);
}
