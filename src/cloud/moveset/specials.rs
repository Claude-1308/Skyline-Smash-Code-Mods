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

#[acmd_script(agent = "cloud", scripts = ["game_specials1", "game_specialairs1"], category = ACMD_GAME)]
pub unsafe fn special_s1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CloudModule::change_sword(module_accessor,hash40("iron_sword_r"));
    }
}

#[acmd_script(agent = "cloud", scripts = ["effect_specials1", "effect_specialairs1"], category = ACMD_EFFECT)]
pub unsafe fn effect_special_s1(_fighter: &mut L2CAgentBase) {}

#[acmd_script(agent = "cloud", scripts = ["sound_specials1", "sound_specialairs1"], category = ACMD_SOUND)]
pub unsafe fn sound_special_s1(_fighter: &mut L2CAgentBase) {}

#[acmd_script(agent = "cloud", scripts = ["game_specials2", "game_specialairs2"], category = ACMD_GAME)]
pub unsafe fn special_s2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.0,361,0,0,0,8.0,0.0,9.0,16.0,Some(0.0),Some(9.0),Some(8.0),0.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_SWORD);
    }
}

#[acmd_script(agent = "cloud", scripts = ["effect_specials2", "effect_specialairs2"], category = ACMD_EFFECT)]
pub unsafe fn effect_special_s2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.2);
        }
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), -3, 2, 0, 0, 0, 0, 1, true, 0.627, 1, 0.674);
        LAST_EFFECT_SET_RATE(fighter,0.25);
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), -3, 6, 0, 0, 0, 0, 1, true, 0.627, 1, 0.674);
        LAST_EFFECT_SET_RATE(fighter,0.25);
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), -3, 10, 0, 0, 0, 0, 1, true, 0.627, 1, 0.674);
        LAST_EFFECT_SET_RATE(fighter,0.25);
    }
}

#[acmd_script(agent = "cloud", scripts = ["sound_specials2", "sound_specialairs2"], category = ACMD_SOUND)]
pub unsafe fn sound_special_s2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_dash_start"));
    }
}

#[acmd_script(agent = "cloud", script = "game_specials3", category = ACMD_GAME)]
pub unsafe fn special_s3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),10.0,50,90,0,60,8.0,0.0,9.0,16.0,Some(0.0),Some(9.0),Some(8.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
    }
    sv_animcmd::frame(lua_state,44.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "game_specialairs3", category = ACMD_GAME)]
pub unsafe fn special_air_s3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),10.0,50,90,0,60,8.0,0.0,9.0,16.0,Some(0.0),Some(9.0),Some(8.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    sv_animcmd::frame(lua_state,30.0);
    if is_excute(fighter) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specials3", category = ACMD_EFFECT)]
pub unsafe fn effect_special_s3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("cloud_speedline"),false,true);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialairs3", category = ACMD_EFFECT)]
pub unsafe fn effect_special_air_s3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("cloud_speedline"),false,true);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", scripts = ["expression_specials3", "expression_specialairs3"], category = ACMD_EXPRESSION)]
pub unsafe fn expression_special_s3(_fighter: &mut L2CAgentBase) {}

#[acmd_script(agent = "cloud", scripts = ["game_specials4", "game_specialairs4"], category = ACMD_GAME)]
pub unsafe fn special_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,2.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),10.0,50,90,0,60,8.0,0.0,9.0,16.0,Some(0.0),Some(9.0),Some(8.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL(fighter,0,0.5);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
    }
    sv_animcmd::frame(lua_state,30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB);
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specials4", category = ACMD_EFFECT)]
pub unsafe fn effect_special_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("cloud_speedline"),false,true);
    }
    sv_animcmd::frame(lua_state,1.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialairs4", category = ACMD_EFFECT)]
pub unsafe fn effect_special_air_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("cloud_speedline"),false,true);
    }
    sv_animcmd::frame(lua_state,1.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", scripts = ["sound_specials3", "sound_specialairs3", "sound_specials4", "sound_specialairs4"], category = ACMD_SOUND)]
pub unsafe fn sound_special_s3_4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter,Hash40::new("seq_cloud_rnd_attack"));
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
}

pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::set_int(module_accessor,1,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT);
    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_CLOUD_SPECIAL_HI_AIR);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(module_accessor,Hash40::new("special_s1"),0.0,2.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_s1"),0.0,2.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe fn special_s_main_loop(fighter: &mut L2CFighterBase) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_s1"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_s1"),-1.0,1.0,0.0,false,false);
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_s2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    CloudModule::change_sword(module_accessor,hash40("iron_sword_r"));
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB);
    if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) < -0.3 {
        PostureModule::reverse_lr(module_accessor);
        PostureModule::update_rot_y_lr(module_accessor);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_DASH);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.clear_lua_stack();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,5.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    }
    else {
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_AIR,3.5 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
    }
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let limiting_speed = 5.0;
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT) == 2 {
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_s2"),0.0,1.65,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("special_s2"),0.0,1.65,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s2_main_loop as *const () as _))
}

unsafe fn special_s2_main_loop(fighter: &mut L2CFighterBase) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,3.5 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        let limiting_speed = 5.0;
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,*ENERGY_STOP_RESET_TYPE_GROUND,5.0 * PostureModule::lr(module_accessor),0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        let limiting_speed = 5.0;
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,limiting_speed,0.0);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    if GroundModule::can_entry_cliff(module_accessor) == 1 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
    }
    if MotionModule::is_end(module_accessor)
    || AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_SHIELD)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_LEFT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_UP_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_UP_LEFT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_DOWN_LEFT as u32) {
        KineticModule::clear_speed_all(module_accessor);
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT) == 2 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(module_accessor,Hash40::new("special_s4"),0.0,1.0,false,0.0,false,false);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_s4"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(module_accessor,Hash40::new("special_s3"),0.0,1.0,false,0.0,false,false);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_s3"),0.0,1.0,false,0.0,false,false);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s3_main_loop as *const () as _))
}

unsafe fn special_s3_main_loop(fighter: &mut L2CFighterBase) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK)
    && WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT) < 3 {
        if ControlModule::get_stick_x(module_accessor).abs() > 0.3
        || ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_SPECIAL_OR_STICK);
            WorkModule::inc_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT);
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2.into(),false.into());
            return L2CValue::I32(0)
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT) == 2 {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_s4"),-1.0,1.0,0.0,false,false);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_s3"),-1.0,1.0,0.0,false,false);
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SLASH_COUNT) == 2 {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_s4"),-1.0,1.0,0.0,false,false);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_s3"),-1.0,1.0,0.0,false,false);
        }
    }
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_ENABLE_LEDGE_GRAB) {
        if GroundModule::can_entry_cliff(module_accessor) == 1 {
            fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
        }
    }
    if MotionModule::is_end(module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub unsafe fn special_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    CloudModule::reset_swords(module_accessor);
    return L2CValue::I32(0)
}