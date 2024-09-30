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
            func: String::from("Unknown"),
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

    let addr = get_loadaddr();
    println!("{:#x}", addr);
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
        let frame = StackFrame::new(
            sp as usize, 
            bp as usize, 
            ip as usize);

        stacktrace.push(frame);

        if bp.is_null() {
            break;
        }

        unsafe {
            ip = *(bp.add(1)) as *const usize;
            sp = bp.add(2);
            bp = *bp as *const usize;
        };
    }

    let load_address = get_loadaddr();
    let elf = elf::Elf::open("/proc/self/exe")?;

    let functions: Vec<&Symbol> = elf.get_symbols_by_type(SymbolType::Function).collect();

    for frame in &mut stacktrace {
        for func in &functions {
            let value = frame.ip - load_address as usize;
            if func.within_range(value) {
                frame.func = func.name.clone();
            }
        }
    }

    Ok(stacktrace)
}

#[inline(always)]
fn get_sp() -> *const usize {
    let sp: *const usize;

    unsafe { asm!("mov   {}, rsp", out(reg) sp); }
    sp
}

#[inline(always)]
fn get_bp() -> *const usize {
    let bp: *const usize;

    unsafe { asm!("mov   {}, rbp", out(reg) bp); }
    bp
}

#[inline(always)]
fn get_ip() -> *const usize {
    let ip: *const usize;

    unsafe { asm!("lea   {}, [rip]", out(reg) ip); }
    ip
}

#[inline(always)]
fn page_align(addr: usize) -> usize {
    addr & !(PAGE_SIZE - 1)
}

fn get_loadaddr() -> usize {
    let mut addr = page_align(get_ip() as usize) as *const u32;
    let magic  = 0x464c457f;         // ELF Magic bytes, little endian

    unsafe {
        while *addr != magic {
            addr = addr.byte_sub(PAGE_SIZE);
        }
    }

    addr as usize
}

