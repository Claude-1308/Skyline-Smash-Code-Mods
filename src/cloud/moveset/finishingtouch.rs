use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use std::arch::asm;

use crate::moveset::*;
use crate::moveset::modules::*;

unsafe fn motion_setup(fighter: &mut L2CFighterCommon, unk: bool) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,-0.02);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,1);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if unk == false {
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_lw"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if unk == false {
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_lw"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("special_lw"),0.0,1.0,false,0.0,false,false);
        }
    }
}

pub unsafe fn finishing_touch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    motion_setup(fighter,true);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.4 * speed_x,0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW_BRAKE) == false {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.2 * speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW_BRAKE);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.4 * speed_x,0.2 * speed_y);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    ControlModule::set_add_jump_mini_button_life(module_accessor,8);
    fighter.sub_shift_status_main(L2CValue::Ptr(finishing_touch_main_loop as *const () as _))
}

unsafe fn finishing_touch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(module_accessor) == false {
        if StatusModule::is_changing(module_accessor) == false {
            if (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND)
            || (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR) {
                motion_setup(fighter,false);
            }
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn finishing_touch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    return L2CValue::I32(0)
}