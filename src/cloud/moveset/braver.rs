use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smashline::*;
use smash_script::lua_args;
use std::arch::asm;
use skyline::libc::c_void;

use crate::moveset::*;
use crate::moveset::modules::*;

pub unsafe fn braver_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    StatusModule::init_settings(module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,0);
    FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,0);
    return L2CValue::I32(0)
}

pub unsafe fn braver_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT);
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT);
    }
    MotionModule::change_motion(module_accessor,Hash40::new("attack_s4_s"),0.0,1.0,false,0.0,false,false);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,1.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_MOTION,1.5,1.0);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.sub_shift_status_main(L2CValue::Ptr(braver_main_loop as *const () as _))
}

unsafe fn braver_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("attack_s4_s"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("attack_s4_s"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if MotionModule::frame(module_accessor) >= 50.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
    }
    if AttackModule::is_attack(module_accessor,0,false) {
        AttackModule::set_power(module_accessor,0,20.0,false);
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe fn braver_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    return L2CValue::I32(0)
}