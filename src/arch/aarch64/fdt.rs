use core::{convert::TryInto, mem, slice, str};

use crate::{
    println,
    utils::{align_to, to_cstr, PointerExt as _, BE},
};

#[repr(C)]
#[derive(Debug)]
struct FdtHeader {
    magic: BE<u32>,
    totalsize: BE<u32>,
    off_dt_struct: BE<u32>,
    off_dt_strings: BE<u32>,
    off_mem_rsvmap: BE<u32>,
    version: BE<u32>,
    last_comp_version: BE<u32>,
    boot_cpuid_phys: BE<u32>,
    size_dt_strings: BE<u32>,
    size_dt_struct: BE<u32>,
}

static mut FDT: Option<*const FdtHeader> = None;

#[no_mangle]
static mut __EXT_FDT_PTR: u64 = 0;

pub unsafe fn fdt_init() {
    unsafe {
        FDT = Some(&*(__EXT_FDT_PTR as *const FdtHeader));
    }
    assert_eq!(u32::from(fdt_header().magic), 0xd00dfeed);
    println!("Initialized FDT.");
}

fn fdt_header() -> &'static FdtHeader {
    unsafe { &*FDT.unwrap() }
}

fn fdt_struct_range() -> (*const BE<u32>, *const BE<u32>) {
    let header = fdt_header();
    let offset: u32 = header.off_dt_struct.into();
    let size: u32 = header.size_dt_struct.into();
    unsafe {
        let header_begin = FDT.unwrap() as *const u8;
        let struct_begin = header_begin.add(offset as usize);
        let struct_end = struct_begin.add(size as usize);
        (struct_begin as *const _, struct_end as *const _)
    }
}

fn fdt_get_string(offset: u32) -> &'static [u8] {
    let header = fdt_header();
    let section_offset: u32 = header.off_dt_strings.into();
    unsafe {
        let header_begin = FDT.unwrap() as *const u8;
        let strings_begin = header_begin.add(section_offset as usize);
        let string_ptr = strings_begin.add(offset as usize);
        to_cstr(string_ptr)
    }
}

struct FdtStructIter {
    ptr: *const BE<u32>,
    end: *const BE<u32>,
}

#[derive(Debug)]
enum FdtStructEntry {
    FdtBeginNode {
        name: &'static [u8],
    },
    FdtEndNode,
    FdtProp {
        len: u32,
        name: &'static [u8],
        value: &'static [u8],
    },
    FdtNop,
    FdtEnd,
}

impl Iterator for FdtStructIter {
    type Item = FdtStructEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        unsafe {
            use FdtStructEntry::*;
            let typ: u32 = self.ptr.next().into();

            let result = match typ {
                0x1 => {
                    let ptr = self.ptr as *const u8;
                    let name = to_cstr(ptr);
                    let name_len = name.len();
                    self.ptr = align_to(ptr.add(name_len + 1), mem::size_of::<u32>()) as *const _;

                    FdtBeginNode { name }
                }
                0x2 => FdtEndNode,
                0x3 => {
                    let len = self.ptr.next().into();
                    let name_offset = self.ptr.next().into();
                    let name = fdt_get_string(name_offset);
                    let ptr = self.ptr as *const u8;
                    let value = core::slice::from_raw_parts(ptr, len as usize);
                    self.ptr = align_to(ptr.add(len as usize), mem::size_of::<u32>()) as *const _;

                    FdtProp { len, name, value }
                }
                0x4 => FdtNop,
                0x9 => FdtEnd,
                _ => {
                    panic!("FDT malformed. Get token = {:x}", typ)
                }
            };
            Some(result)
        }
    }
}

fn fdt_struct_iter() -> FdtStructIter {
    let (start, end) = fdt_struct_range();
    FdtStructIter { ptr: start, end }
}

//pub fn

pub fn get_memory_size() -> usize {
    0
}

pub fn fdt_get_memory() -> (u64, u64) {
    use FdtStructEntry::*;
    let mut iter = fdt_struct_iter();

    unsafe fn to_be_u64(slice: &[u8]) -> BE<u64> {
        let arr: [u8; 8] = slice.try_into().unwrap();
        unsafe { mem::transmute(arr) }
    }

    while let Some(entry) = iter.next() {
        if let FdtBeginNode { name } = entry {
            if name.len() >= 6 && &name[0..6] == b"memory" {
                let ent2 = iter.next().unwrap();
                match ent2 {
                    FdtProp { value, .. } => {
                        let start: u64 = unsafe { to_be_u64(&value[0..8]) }.into();
                        let size: u64 = unsafe { to_be_u64(&value[8..16]) }.into();

                        return (start, size);
                    }
                    _ => {
                        panic!("data malformed");
                    }
                }
            }
        }
    }

    panic!("memory info not found");
}
