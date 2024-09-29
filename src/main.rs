use std::{
    arch::asm,
    fmt,
    io,
};

use frieren::{
    elf,
    symbols::*,
};

const PAGE_SIZE: usize = 0x1000;

#[derive(Debug)]
struct StackFrame {
    sp: usize,
    bp: usize,
    ip: usize,
    func: String
}

impl StackFrame {
    fn new(sp: usize, bp: usize, ip: usize) -> Self {
        StackFrame {
            sp,
            bp,
            ip,
            func: String::new(),
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
                \tfunction: {},\n\
            }}",
            self.sp, self.bp, self.ip, self.func)
    }
}

fn main() -> io::Result<()> {
    let st = get_stacktrace()?;

    for frame in st {
        println!("{}", frame);
    }
    Ok(())
}

fn get_stacktrace() -> io::Result<Vec<StackFrame>> {
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

    let load_address = get_loadaddr();
    let elf = elf::Elf::open("/proc/self/exe")?;

    for frame in &mut stacktrace {
        for func in elf.get_symbols_by_type(SymbolType::Function) { // TODO: inefficient
            let value = frame.ip - load_address as usize;
            if func.within_range(value) {
                frame.func = func.name.clone();
            }
        }
    }

    Ok(stacktrace)
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

fn get_loadaddr() -> *mut usize {
    let ip = get_ip() & !(PAGE_SIZE -1);
    let mut p = ip as *const u32;
    let magic = 0x464c457f;         // ELF Magic bytes, little endian

    unsafe {
        while *p != magic {
            p = (p as usize - PAGE_SIZE) as *const u32;
        }
    }

    p as *mut usize
}
