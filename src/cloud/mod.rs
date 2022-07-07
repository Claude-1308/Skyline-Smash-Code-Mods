use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::app::lua_bind::*;
use smashline::*;
use skyline::nn::ro::LookupSymbol;

mod normals;
use normals::*;
//mod specialn;
//use specialn::*;
mod speciallw;
use speciallw::*;
//mod specialhi;
//use specialhi::*;
mod specials;
use specials::*;
mod modules;
use modules::*;
mod opff;
use opff::*;
mod api;
use api::*;
use crate::FIGHTER_MANAGER;

pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK: i32 = 0x200000ec;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT: i32 = 0x200000ed;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK: i32 = 0x200000ee;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB: i32 = 0x200000ef;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB: i32 = 0x200000f0;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET: i32 = 0x200000f1;
pub const FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT: i32 = 0x100000c6;
pub const WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM: i32 = 0x200000f8;

//pub const MAGIC_TYPE_FIRE : i32 = 0;
//pub const MAGIC_TYPE_ICE : i32 = 1;
//pub const MAGIC_TYPE_ELEC : i32 = 2;
//pub const MAGIC_TYPE_WIND : i32 = 3;

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;

pub static mut SWORDS: [bool; 8] = [false; 8];

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
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB);
    fighter.status_pre_Dead()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_DEMO, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn demo_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    fighter.status_pre_Demo()
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
        effect_13,
        effect_dash,
        effect_s3,
        effect_hi3,
        attack_s4_s,
        effect_s4,
        sound_s4,
        effect_hi4,
        attack_lw4,
        effect_lw4,
        sound_lw4,
        effect_air_n,
        effect_air_f,
        effect_air_b,
        effect_air_hi,
        effect_air_lw,
        effect_cliff_attack,
        effect_down_attack_d,
        effect_down_attack_u,
        effect_slip_attack,
        /*regular,
        hit_lb,
        effect_regular,
        fire,
        effect_fire,
        sound_fire,
        wind,
        effect_wind,
        sound_wind,
        effect_hit_fire,
        effect_hit_wind,
        special_n,
        effect_special_n,
        sound_special_n,
        expression_special_n,
        special_lw,*/
        special_lw_start,
        effect_lw_start,
        sound_lw_start,
        /*special_lw_hit,
        special_air_lw_hit,
        effect_lw_hit,
        effect_air_lw_hit,
        sound_lw_hit,
        sound_air_lw_hit,*/
        special_lw_hit_extra,
        effect_lw_hit_extra,
        effect_air_lw_hit_extra,
        sound_lw_hit_extra,
        /*special_hi_lb,
        effect_special_hi,
        effect_special_air_hi,
        effect_special_hi2_end,
        effect_special_hi2_fall,
        effect_special_hi2_landing,
        effect_special_hi_lb,
        effect_special_air_hi_lb,*/
        special_s1_lb,
        //special_s3_lb,
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
        sound_special_s3_4,
        special_s4,
        effect_special_s4,
        effect_special_air_s4,
    );
    install_status_scripts!(
        attack_s4_end,
        attack_lw4_end,
        //regular_main,
        //special_n_main,
        //hit_main,
        special_s_main,
        special_s2_main,
        special_s3_main,
        special_s3_end,
        special_lw_main,
        //special_lw_counter_main,
        special_lw_end_main,
        entry_pre,
        wait_pre,
        fall_pre,
        dead_pre,
        demo_pre,
    );
    install_agent_frames!(
        cloud,
        cloud_wave,
    );
    skyline::install_hook!(
        limit_manager
    );
    my_callback::install("fighter/cloud/model/body/c00/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c02/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c04/sword_cloud_001_col.nutexb", 0x1583D0);
    my_callback::install("fighter/cloud/model/body/c06/sword_cloud_001_col.nutexb", 0x1583D0);
}