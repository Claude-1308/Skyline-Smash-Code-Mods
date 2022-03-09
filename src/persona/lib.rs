#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod moveset;

static mut CONSTANT_OFFSET : usize = 0x37266f0; //13.0.0

use skyline::hooks::{getRegionAddress, Region};
use skyline::libc::*;
use std::ffi::CStr;

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_JACK_INSTANCE_WORK_ID_INT_TERM") {
        value = 0x100000c4;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x200000f0;
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "joker")]
pub fn main() {
    skyline::install_hook!(const_allot_hook);
    moveset::install();
}