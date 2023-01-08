use smash::lib::{L2CValue,lua_const::*};
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

#[acmd_script(agent = "cloud", scripts = ["game_specials1_lb", "game_specialairs1_lb"], category = ACMD_GAME)]
pub unsafe fn special_s1_lb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,10.0);
    if sv_animcmd::get_value_float(lua_state,*SO_VAR_FLOAT_LR) > 0.0 {
        if is_excute(fighter) {
            ATTACK(fighter,0,0,Hash40::new("top"),5.0,368,100,80,0,4.025,0.0,9.0,16.0,Some(0.0),Some(6.5),Some(16.0),0.3,0.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
            ATTACK(fighter,1,0,Hash40::new("top"),5.0,368,100,80,0,8.05,0.0,19.0,14.0,Some(0.0),Some(9.0),Some(16.0),0.3,0.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
            let vec2 = Vector2f{x: 12.0, y: 9.0};
            AttackModule::set_vec_target_pos(module_accessor,0,Hash40::new("top"),&vec2,7,false);
            AttackModule::set_vec_target_pos(module_accessor,1,Hash40::new("top"),&vec2,7,false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter,0,1.5);
            ATK_SET_SHIELD_SETOFF_MUL(fighter,1,1.5);
        }
    }
    else {
        if is_excute(fighter) {
            ATTACK(fighter,0,0,Hash40::new("top"),5.0,368,100,80,0,4.025,0.0,9.0,10.5,Some(0.0),Some(6.5),Some(10.0),0.3,0.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
            ATTACK(fighter,1,0,Hash40::new("top"),5.0,368,100,80,0,8.05,0.0,19.0,16.0,Some(0.0),Some(9.0),Some(14.0),0.3,0.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
            let vec2 = Vector2f{x: 12.0, y: 9.0};
            AttackModule::set_vec_target_pos(module_accessor,0,Hash40::new("top"),&vec2,7,false);
            AttackModule::set_vec_target_pos(module_accessor,1,Hash40::new("top"),&vec2,7,false);
            ATK_SET_SHIELD_SETOFF_MUL(fighter,0,1.5);
            ATK_SET_SHIELD_SETOFF_MUL(fighter,1,1.5);
        }
    }
    FT_MOTION_RATE(fighter,0.75);
    sv_animcmd::frame(lua_state,12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
}

pub unsafe fn crossslash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    StatusModule::init_settings(module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,0);
    FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64,*FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32 | *FIGHTER_STATUS_ATTR_START_TURN as u32,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,0);
    return L2CValue::I32(0)
}

unsafe fn motion_setup(fighter: &mut L2CFighterCommon, unk: bool) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if unk == false {
            let motion = WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR) as u64;
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new_raw(motion),-1.0,1.0,0.0,false,false);
        }
        else {
            let motion = WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR) as u64;
            MotionModule::change_motion(module_accessor,Hash40::new_raw(motion),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if unk == false {
            let motion = WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND) as u64;
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new_raw(motion),-1.0,1.0,0.0,false,false);
        }
        else {
            let motion = WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND) as u64;
            MotionModule::change_motion(module_accessor,Hash40::new_raw(motion),0.0,1.0,false,0.0,false,false);
        }
    }
}

unsafe fn status_change(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
        let status = WorkModule::get_int(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
        StatusModule::change_status_request(module_accessor,status,false);
    }
}

pub unsafe fn crossslash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sum_speed = KineticModule::get_sum_speed(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,sum_speed.x * 0.2,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,sum_speed.x * 0.2,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,sum_speed.y,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    WorkModule::set_int(module_accessor,fighter.global_table[SITUATION_KIND].get_i32(),*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_SITUATION_PREV);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    PostureModule::set_stick_lr(module_accessor,0.0);
    PostureModule::update_rot_y_lr(module_accessor);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    ControlModule::reset_trigger(module_accessor);
    WorkModule::set_int(module_accessor,FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_2,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
    WorkModule::set_int64(module_accessor,hash40("special_s1_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(module_accessor,hash40("special_air_s1_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    motion_setup(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(crossslash_main_loop as *const () as _))
}

pub unsafe fn crossslash2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sum_speed = KineticModule::get_sum_speed(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.1,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.01);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.1);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    }
    WorkModule::set_int(module_accessor,fighter.global_table[SITUATION_KIND].get_i32(),*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_SITUATION_PREV);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    ControlModule::reset_trigger(module_accessor);
    WorkModule::set_int(module_accessor,FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_3,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
    WorkModule::set_int64(module_accessor,hash40("special_s2_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(module_accessor,hash40("special_air_s2_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    motion_setup(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(crossslash_main_loop as *const () as _))
}

pub unsafe fn crossslash3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sum_speed = KineticModule::get_sum_speed(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.1,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.01);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.1);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    }
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::set_int64(module_accessor,hash40("special_s3_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(module_accessor,hash40("special_air_s3_lb") as i64,*FIGHTER_CLOUD_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    motion_setup(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(crossslash_main_loop as *const () as _))
}

unsafe fn crossslash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        status_change(fighter);
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

pub unsafe fn crossslash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionAnimcmdModule::enable_skip_delay_update(module_accessor);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_l_lb"),5);
    WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    return L2CValue::I32(0)
}

pub unsafe fn crossslash2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionAnimcmdModule::enable_skip_delay_update(module_accessor);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_l_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke2_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke2_l_lb"),5);
    return L2CValue::I32(0)
}

pub unsafe fn crossslash3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionAnimcmdModule::enable_skip_delay_update(module_accessor);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke1_l_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke2_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke2_l_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke3_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke3_l_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke4_r_lb"),5);
    EffectModule::end_kind(module_accessor,Hash40::new("cloud_kyogiri_stroke4_l_lb"),5);
    if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    return L2CValue::I32(0)
}