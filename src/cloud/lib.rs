#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

pub static mut FIGHTER_MANAGER : usize = 0;

mod moveset;

static mut CONSTANT_OFFSET : usize = 0x3727390; //13.0.1

use skyline::libc::*;
use std::ffi::CStr;

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_CLOUD_STATUS_KIND_NUM") {
        value = 0x1f9;
    }
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_TERM") {
        value = 0x2000000a;
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "cloud")]
pub fn main() {
    skyline::install_hook!(const_allot_hook);
    moveset::install();
}