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

#[acmd_script(agent = "cloud", script = "game_specialnburst", category = ACMD_GAME)]
pub unsafe fn special_n_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CloudModule::change_sword(module_accessor,hash40("mythril_saber_r"));
    }
    sv_animcmd::frame(lua_state,23.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),2.0,95,10,0,10,5.0,0.0,13.0,-2.0,Some(0.0),Some(0.0),Some(-2.0),0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,15);
    }
    sv_animcmd::frame(lua_state,37.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),10.0,75,40,0,100,5.0,0.0,13.0,-2.0,Some(0.0),Some(0.0),Some(-2.0),1.2,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,39.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BURST_BEAM);
        ArticleModule::generate_article(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,-1);
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,0);
    }
    sv_animcmd::frame(lua_state,45.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialnburst", category = ACMD_EFFECT)]
pub unsafe fn effect_special_n_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    sv_animcmd::frame(lua_state,22.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 5, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 5, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,38.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,39.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,41.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_specialnburst", category = ACMD_SOUND)]
pub unsafe fn sound_special_n_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_swing_m"));
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_jump01"));
    }
    sv_animcmd::frame(lua_state,37.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter,Hash40::new("seq_cloud_rnd_attack"));
        PLAY_SE(fighter,Hash40::new("se_cloud_attackair_f01"));
    }
    sv_animcmd::frame(lua_state,38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_common_throw_02"));
    }
}

pub unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,0);
    FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON as u64 | *FIGHTER_LOG_MASK_FLAG_SHOOT as u64,*FIGHTER_STATUS_ATTR_START_TURN as u32,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,0);
    return L2CValue::I32(0)
}

unsafe fn motion_setup(fighter: &mut L2CFighterCommon, unk: bool) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if unk == false {
            KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_FALL);
            let air_speed_x_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_x_stable"),0);
            let control_limit_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("control_limit_x_mul"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,control_limit_x_mul*air_speed_x_stable,0.0);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_n"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_n"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if unk == false {
            KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_n"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("special_n"),0.0,1.0,false,0.0,false,false);
        }
    }
}

pub unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) == false {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BURST_BEAM);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(module_accessor,Hash40::new("special_n_burst"),0.0,1.0,false,0.0,false,false);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_burst_main_loop as *const () as _))
    }
    else {
        motion_setup(fighter,true);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
    }
}

unsafe fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(module_accessor) == false {
        if StatusModule::is_changing(module_accessor) == false {
            if fighter.global_table[SITUATION_KIND].get_i32() != fighter.global_table[PREV_SITUATION_KIND].get_i32() {
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

unsafe fn special_n_burst_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let sum_speed = KineticModule::get_sum_speed(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let air_speed_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("air_speed_x_mul"));
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N) {
            let air_speed_y_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("air_speed_y_mul"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,sum_speed.y * air_speed_y_mul,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,sum_speed.x * air_speed_x_mul,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            let air_speed_x_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_x_stable"),0);
            let control_limit_x_mul = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("control_limit_x_mul"));
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,control_limit_x_mul * air_speed_x_stable,0.0);
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::clear_speed_energy_id(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticModule::clear_speed_energy_id(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,sum_speed.x,0.0,0.0,0.0,0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::clear_speed_energy_id(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL) == false {
            let air_accel_y = WorkModule::get_param_float(module_accessor,hash40("air_accel_y"),0) * -1.0;
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            let air_speed_y_stable = WorkModule::get_param_float(module_accessor,hash40("air_speed_y_stable"),0);
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,air_speed_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
        else {
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP) == false {
                let hop_speed_y = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("hop_speed_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,hop_speed_y);
                sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                let attack_accel_y = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("attack_accel_y")) * -1.0;
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,attack_accel_y);
                sv_kinetic_energy::set_accel(fighter.lua_state_agent);
                let attack_max_y = WorkModule::get_param_float(module_accessor,hash40("param_special_n"),hash40("attack_max_y"));
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,attack_max_y);
                sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N_HOP);
            }
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "cloud_wave", status = WEAPON_CLOUD_WAVE_STATUS_KIND_REGULAR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wave_regular_pre(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if WorkModule::get_int(owner,*FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
        WorkModule::set_int(module_accessor,1,*WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO);
        WorkModule::on_flag(owner,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    }
    original!(weapon)
}

#[status_script(agent = "cloud_wave", status = WEAPON_CLOUD_WAVE_STATUS_KIND_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn wave_hit_end(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT)
    && WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO) != 0 {
        WorkModule::inc_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    }
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        WorkModule::off_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR);
    }
    original!(weapon)
}