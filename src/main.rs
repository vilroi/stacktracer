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
    sp: *const usize,
    bp: *const usize,
    ip: *const usize,
    func: String,
    data: Vec<u8>,
}

impl StackFrame {
    fn new(sp: *const usize, bp: *const usize, ip: *const usize) -> Self {
        let mut data: Vec<u8> = Vec::new();

        if !bp.is_null() {
            let stack_size = bp as usize - sp as usize;
            let mut p = sp as *const u8;

            for _ in 0..stack_size {
                unsafe {
                    data.push(*p);
                    p = p.add(1);
                }
            }
        }

        StackFrame {
            sp,
            bp,
            ip,
            func: String::from("Unknown"),
            data: Vec::new(),
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
            self.sp as usize, self.bp as usize, self.ip as usize, self.func)
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
    let mut frames: Vec<StackFrame> = Vec::new();

    loop {
        let frame = StackFrame::new(sp, bp, ip);
        frames.push(frame);

        if bp.is_null() {
            break;
        }

        unsafe {
            ip = *(bp.add(1)) as *const usize;
            sp = bp.add(2);
            bp = *bp as *const usize;
        };
    }

    resolve_addresses(&mut frames)?;

    Ok(frames)
}

fn resolve_addresses(frames: &mut Vec<StackFrame>) -> io::Result<()> {
    let load_address = get_loadaddr();
    let elf = elf::Elf::open("/proc/self/exe")?;
    let functions: Vec<&Symbol> = elf.get_symbols_by_type(SymbolType::Function).collect();

    for frame in frames {
        for func in &functions {
            let value = frame.ip as usize - load_address;
            if func.within_range(value) {
                frame.func = func.name.clone();
                break;
            }
        }
    }

    Ok(())
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

