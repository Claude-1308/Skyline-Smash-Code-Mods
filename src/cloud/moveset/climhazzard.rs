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

pub unsafe fn climhazzard_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    StatusModule::init_settings(module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,0);
    FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,0);
    return L2CValue::I32(0)
}

pub unsafe fn climhazzard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_2_SHIFT_INPUT);
    WorkModule::set_int64(module_accessor,hash40("special_hi_lb") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(module_accessor,hash40("special_air_hi_lb") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(module_accessor,0.25,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(module_accessor,0.625,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(module_accessor,12.0,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(module_accessor,1.6,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(module_accessor,0.5,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(module_accessor,0.67,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(module_accessor,1.85,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(module_accessor,0.8,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(module_accessor,20,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_int(module_accessor,*FIGHTER_STATUS_KIND_FALL,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(climhazzard_main_loop as *const () as _))
}

unsafe fn climhazzard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    fighter.super_jump_punch_main();
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL) {
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_speed_x_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_x_stable"),0);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,air_speed_x_stable * 0.67,0.0);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
    }
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
        WorkModule::off_flag(module_accessor,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            if speed_y < 0.0 {
                let speed_y2 = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if speed_y2 < 0.0 {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                }
            }
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn climhazzard_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.super_jump_punch_reset_common_condition();
    if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    return L2CValue::I32(0)
}