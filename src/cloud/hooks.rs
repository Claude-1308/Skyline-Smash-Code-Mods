use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;

use crate::moveset::*;

#[skyline::hook(offset = 0x3a6650)]
pub unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    if weapon_kind == *WEAPON_KIND_MASTER_SPEAR {
        return 1
    }
    else {
        original!()(weapon_kind,entry_id)
    }
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    if WorkModule::is_flag(defender_boma,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK)
    && utility::get_kind(&mut *defender_boma) == *FIGHTER_KIND_CLOUD {
        WorkModule::on_flag(defender_boma,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT);
        WorkModule::set_int64(defender_boma,attacker_id as i64,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_ATTACKER_ID);
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

pub static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}