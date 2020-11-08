#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

pub static mut FIGHTER_MANAGER: usize = 0;

mod link_elemental_arrows;
mod link_bullet_time;
mod link_misc;
mod link_mipha_grace;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    link_elemental_arrows::install();
    link_bullet_time::install();
    link_misc::install();
    link_mipha_grace::install();
}
