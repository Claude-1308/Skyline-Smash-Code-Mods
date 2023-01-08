use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use skyline::hooks::InlineCtx;
use crate::FIGHTER_MANAGER;

use crate::moveset::*;

#[smashline::fighter_frame(agent = FIGHTER_KIND_CLOUD)]
pub fn cloud(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if StopModule::is_damage(module_accessor)
        && WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET) == false {
            CloudModule::reset_swords(module_accessor);
        }
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_CLOUD_WAVE)]
pub fn cloud_wave(weapon: &mut L2CFighterBase) {
    unsafe {
        let lua_state = weapon.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
            let effect = WorkModule::get_int64(module_accessor,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            EffectModule::kill_kind(module_accessor,Hash40::new_raw(effect),true,true);
            weapon.clear_lua_stack();
            lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("sys_damage_fire"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,true);
            sv_module_access::effect(weapon.lua_state_agent);
            WorkModule::set_int64(module_accessor,hash40("sys_damage_fire") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)
            && StatusModule::status_kind(module_accessor) != *WEAPON_CLOUD_WAVE_STATUS_KIND_HIT {
                weapon.change_status(WEAPON_CLOUD_WAVE_STATUS_KIND_HIT.into(),false.into())
            }
        }
    }
}