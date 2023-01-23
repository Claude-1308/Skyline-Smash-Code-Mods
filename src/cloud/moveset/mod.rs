use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use skyline::nn::ro::LookupSymbol;

mod normals;
use normals::*;
mod specialn;
use specialn::*;
mod speciallw;
use speciallw::*;
mod specials;
use specials::*;
mod modules;
use modules::*;
mod opff;
use opff::*;
mod api;
use api::*;
mod braver;
use braver::*;
mod hooks;
use hooks::*;
mod crossslash;
use crossslash::*;
mod climhazzard;
use climhazzard::*;
mod finishingtouch;
use finishingtouch::*;
mod meteorrain;
use meteorrain::*;
use crate::FIGHTER_MANAGER;

pub const FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_2: i32 = 0x1f7;
pub const FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_3: i32 = 0x1f8;

pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK: i32 = 0x200000ec;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT: i32 = 0x200000ed;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK: i32 = 0x200000ee;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB: i32 = 0x200000ef;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET: i32 = 0x200000f0;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BURST_BEAM: i32 = 0x200000f1;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_AIR: i32 = 0x200000f2;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY: i32 = 0x200000f3;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE: i32 = 0x200000f4;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT: i32 = 0x200000f5;

pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT: i32 = 0x100000c6;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL: i32 = 0x100000c7;

pub const WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_BURST_BEAM: i32 = 0x20000008;
pub const WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR: i32 = 0x20000009;

pub const LIMIT_BREAK_LEVEL_1 : i32 = 0;
pub const LIMIT_BREAK_LEVEL_2 : i32 = 1;
pub const LIMIT_BREAK_LEVEL_3 : i32 = 2;
pub const LIMIT_BREAK_LEVEL_4 : i32 = 3;

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;
pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3d;

pub static mut SWORDS: [bool; 256] = [false; 256];

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn entry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    fighter.status_pre_Entry()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    fighter.status_pre_Wait()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    fighter.status_pre_Fall()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn dead_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    let lua_module = *(module_accessor as *mut smash::app::BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
    waza_customize(lua_module,*FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1);
    if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL) == LIMIT_BREAK_LEVEL_4 {
        WorkModule::set_int(module_accessor,LIMIT_BREAK_LEVEL_1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    fighter.status_pre_Dead()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_DEMO, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn demo_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    fighter.status_pre_Demo()
}

#[status_script(agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_FINAL_DASH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn final_dash_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::set_int(module_accessor,LIMIT_BREAK_LEVEL_1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE_NOTICE);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    send_limit_gauge_event(entry_id);
    original!(fighter)
}

#[status_script(agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_FINAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn final_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::set_int(module_accessor,LIMIT_BREAK_LEVEL_1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE_NOTICE);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    send_limit_gauge_event(entry_id);
    original!(fighter)
}

#[status_script(agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_FINAL2_DASH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn final2_dash_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::set_int(module_accessor,LIMIT_BREAK_LEVEL_1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE_NOTICE);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    send_limit_gauge_event(entry_id);
    original!(fighter)
}

#[status_script(agent = "cloud", status = FIGHTER_CLOUD_STATUS_KIND_FINAL2_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn final2_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    WorkModule::set_int(module_accessor,LIMIT_BREAK_LEVEL_1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    WorkModule::set_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE_NOTICE);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    send_limit_gauge_event(entry_id);
    original!(fighter)
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
    install_acmd_scripts!(
        attack_dash,
        effect_dash,
        sound_dash,
        attack_s4_s,
        effect_s4,
        sound_s4,
        attack_lw4,
        effect_lw4,
        sound_lw4,
        special_n_burst,
        effect_special_n_burst,
        sound_special_n_burst,
        special_lw_stance,
        effect_lw_stance,
        sound_lw_stance,
        special_lw_strike,
        effect_lw_strike,
        effect_air_lw_strike,
        sound_lw_strike,
        special_s1_lb,
        special_s1,
        effect_special_s1,
        sound_special_s1,
        special_s2,
        effect_special_s2,
        sound_special_s2,
        special_s3,
        special_air_s3,
        effect_special_s3,
        effect_special_air_s3,
        expression_special_s3,
        special_s4,
        effect_special_s4,
        effect_special_air_s4,
        sound_special_s3_4,
        meteor_rain,
        meteor_rain_air,
        effect_meteor_rain,
        effect_meteor_rain_air,
        sound_meteor_rain,
        sound_meteor_rain_air,
        regular,
        regular_lb,
        hit_lb,
        effect_regular_lb,
        effect_regular_air_lb,
        effect_hit_lb,
        sound_regular_lb,
        sound_regular_air_lb,
        sound_hit_lb
    );
    install_status_scripts!(
        attack_dash_main,
        attack_s4_main,
        attack_s4_end,
        attack_lw4_end,
        wave_regular_pre,
        regular_main,
        wave_hit_end,
        entry_pre,
        wait_pre,
        fall_pre,
        dead_pre,
        demo_pre,
        final_dash_end_end,
        final_end_end,
        final2_dash_end_end,
        final2_end_end
    );
    install_agent_frames!(
        cloud,
        cloud_wave
    );
    install_agent_reset!(cloud_status_init);
    skyline::install_hooks!(
        limit_manager,
        cloud_waza_setup
    );
    my_callback::install("fighter/cloud/model/body/c00/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c02/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c04/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c06/sword_cloud_001_col.nutexb", 0x1583D0);
}
