use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use std::arch::asm;

use crate::moveset::*;
use crate::moveset::modules::*;

#[acmd_script(agent = "cloud", script = "game_attackdash", category = ACMD_GAME)]
pub unsafe fn attack_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),10.0,361,60,0,80,5.0,0.0,10.0,15.0,Some(0.0),Some(10.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,26.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),7.0,361,40,0,70,5.0,0.0,10.0,25.0,Some(0.0),Some(10.0),Some(10.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackdash", category = ACMD_EFFECT)]
pub unsafe fn effect_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.3);
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), -5, 10, 0, 0, 0, 0, 0.8, true, 0.627, 1, 0.674);
        EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("cloud_speedline"), false, true);
    }
    sv_animcmd::frame(lua_state,26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,37.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_attackdash", category = ACMD_SOUND)]
pub unsafe fn sound_dash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_cloud_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_cloud_attackdash"));
    }
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,0.5,1.0);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.status_AttackDash()
}

#[acmd_script(agent = "cloud", script = "game_attacks4", category = ACMD_GAME)]
pub unsafe fn attack_s4_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,10);
    }
    sv_animcmd::frame(lua_state,33.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),13.0,50,100,0,40,5.0,0.0,15.0,-2.0,Some(0.0),Some(0.0),Some(-2.0),1.2,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),11.0,60,100,0,40,7.0,0.0,0.0,5.0,Some(0.0),Some(0.0),Some(15.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,0);
        AttackModule::clear_all(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attacks4", category = ACMD_EFFECT)]
pub unsafe fn effect_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,32.0);
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if is_excute(fighter) {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.3);
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.3);
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT) == false {
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
    sv_animcmd::frame(lua_state,52.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,66.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(agent = "cloud", script = "sound_attacks4", category = ACMD_SOUND)]
pub unsafe fn sound_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_swing_s"));
        PLAY_SE(fighter,Hash40::new("vc_cloud_attack01"));
    }
    sv_animcmd::frame(lua_state,15.0);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_common_smash_start_02"));
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_jump01"));
    }
    sv_animcmd::frame(lua_state,32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_attack06"));
        PLAY_SE(fighter,Hash40::new("se_cloud_attackair_f01"));
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_common_throw_02"));
    }
    sv_animcmd::frame(lua_state,66.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_step_left_m"));
    }
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_MOTION,0.8,0.5);
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    WorkModule::off_flag(fighter.module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BRAVER_NO_EFFECT);
    fighter.status_AttackS4()
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
    CloudModule::reset_swords(module_accessor);
    fighter.status_end_AttackS4()
}

#[acmd_script(agent = "cloud", script = "game_attacklw4", category = ACMD_GAME)]
pub unsafe fn attack_lw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CloudModule::change_sword(module_accessor,hash40("hardedge_r"));
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state,34.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        let vec2 = Vector2f{x: 12.0, y: 9.0};
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,0,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,1,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,2,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,3,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,4,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,22.0,false);
        AttackModule::set_vec_target_pos(module_accessor,5,Hash40::new("top"),&vec2,7,false);
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,45.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,10.0,false);
    }
    sv_animcmd::frame(lua_state,47.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,54.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,368,10,0,5,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,368,0,0,20,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        let vec2 = Vector2f{x: 12.0, y: 9.0};
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,0,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,1,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,2,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,3,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,4,Hash40::new("top"),&vec2,7,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,40.0,false);
        AttackModule::set_vec_target_pos(module_accessor,5,Hash40::new("top"),&vec2,7,false);
    }
    sv_animcmd::frame(lua_state,56.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,65.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,10.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,10.0,false);
    }
    sv_animcmd::frame(lua_state,67.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,15);
    }
    sv_animcmd::frame(lua_state,86.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),1.0,85,10,0,5,4.0,0.0,20.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("haver"),1.0,85,10,0,5,4.0,0.0,15.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("haver"),1.0,85,10,0,5,4.0,0.0,10.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("haver"),1.0,85,10,0,5,4.0,0.0,5.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,20.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,20.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,20.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,20.0,false);
    }
    sv_animcmd::frame(lua_state,96.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,99.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),20.0,50,90,0,35,4.0,0.0,20.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("haver"),20.0,50,90,0,35,4.0,0.0,15.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("haver"),20.0,50,90,0,35,4.0,0.0,10.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("haver"),20.0,50,90,0,35,4.0,0.0,5.0,0.0,None,None,None,1.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,10.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,102.0);
    if is_excute(fighter) {
        QUAKE(fighter,*CAMERA_QUAKE_KIND_M);
        ATTACK(fighter,0,0,Hash40::new("top"),20.0,70,90,0,35,9.0,0.0,0.0,6.0,Some(0.0),Some(0.0),Some(12.0),2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        AttackModule::clear(module_accessor,1,false);
        AttackModule::clear(module_accessor,2,false);
        AttackModule::clear(module_accessor,3,false);
    }
    sv_animcmd::frame(lua_state,105.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,0);
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,142.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attacklw4", category = ACMD_EFFECT)]
pub unsafe fn effect_lw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,32.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 5, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,45.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,54.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,65.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,69.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,102.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,105.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_attacklw4", category = ACMD_SOUND)]
pub unsafe fn sound_lw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_common_smash_start_02"));
    }
    sv_animcmd::frame(lua_state,34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,54.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,105.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_smash_l01"));
    }
}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn attack_lw4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
    CloudModule::reset_swords(module_accessor);
    fighter.status_end_AttackLw4()
}