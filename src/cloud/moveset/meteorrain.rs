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

#[acmd_script(agent = "cloud", script = "game_meteorrain", category = ACMD_GAME)]
pub unsafe fn meteor_rain(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_AIR);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE);
        ArticleModule::generate_article(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,-1);
    }
    sv_animcmd::frame(lua_state,28.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

#[acmd_script(agent = "cloud", script = "game_meteorrainair", category = ACMD_GAME)]
pub unsafe fn meteor_rain_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE);
        ArticleModule::generate_article(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,-1);
    }
    sv_animcmd::frame(lua_state,28.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

#[acmd_script(agent = "cloud", script = "effect_meteorrain", category = ACMD_EFFECT)]
pub unsafe fn effect_meteor_rain(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.3);
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_meteorrainair", category = ACMD_EFFECT)]
pub unsafe fn effect_meteor_rain_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.3);
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_meteorrain", category = ACMD_SOUND)]
pub unsafe fn sound_meteor_rain(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_jump01"));
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_item_revengeshooter_shot_max"));
    }
}

#[acmd_script(agent = "cloud", script = "sound_meteorrainair", category = ACMD_SOUND)]
pub unsafe fn sound_meteor_rain_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_item_revengeshooter_shot_max"));
    }
}

pub unsafe fn meteorrain_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
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
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("meteor_rain_air"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("meteor_rain_air"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if unk == false {
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("meteor_rain"),-1.0,1.0,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("meteor_rain"),0.0,1.0,false,0.0,false,false);
        }
    }
}

pub unsafe fn meteorrain_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_AIR);
    motion_setup(fighter,true);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,4.5);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,1.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_FREE,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(meteorrain_main_loop as *const () as _))
}

unsafe fn meteorrain_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if CancelModule::is_enable_cancel(module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_AIR) {
        StatusModule::set_keep_situation_air(module_accessor,true);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_AIR);
    }
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY) {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_UNABLE_GRAVITY);
    }
    if MotionModule::is_end(module_accessor)  {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
    }
    return L2CValue::I32(0)
}

pub unsafe fn meteorrain_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE);
    WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    return L2CValue::I32(0)
}

#[status_script(agent = "cloud_wave", status = WEAPON_CLOUD_WAVE_STATUS_KIND_REGULAR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn regular_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if WorkModule::is_flag(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE) {
        WorkModule::on_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR);
        WorkModule::off_flag(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RAIN_WAVE);
        let vec3 = Vector3f{x: 45.0, y: 0.0, z: 0.0};
        PostureModule::set_rot(module_accessor,&vec3,0);
        original!(weapon)
    }
    else {
        if WorkModule::is_flag(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BURST_BEAM) {
            WorkModule::on_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_BURST_BEAM);
            WorkModule::off_flag(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_BURST_BEAM);
        }
        original!(weapon)
    }
}

unsafe fn meteor_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) < 1.0 {
        MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_GAME,Hash40::new("game_regular_lb"),-1);
        MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_EFFECT,Hash40::new("effect_regular_lb"),-1);
        MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_SOUND,Hash40::new("sound_regular_lb"),-1);
        return L2CValue::I32(0)
    }
    if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_ALL as u32)
    || AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_SHIELD) {
        weapon.change_status(WEAPON_CLOUD_WAVE_STATUS_KIND_HIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[acmd_script(agent = "cloud_wave", scripts = ["game_regular","game_regularair"], category = ACMD_GAME)]
pub unsafe fn regular(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_BURST_BEAM) {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 12.0, 55, 60, 0, 38, 5.0, 0.0, 11.0, -2.1, Some(0.0), Some(5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 12.0, 55, 60, 0, 38, 5.0, 0.0, 1.7, -0.8, Some(0.0), Some(5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
    else {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 55, 60, 0, 38, 5.0, 0.0, 11.0, -2.1, Some(0.0), Some(5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 55, 60, 0, 38, 5.0, 0.0, 1.7, -0.8, Some(0.0), Some(5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["game_regular_lb","game_regularair_lb"], category = ACMD_GAME)]
pub unsafe fn regular_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) == false {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 55, 60, 0, 38, 7.0, 0.0, 13.5, -7.2, Some(0.0), Some(5.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
            ATTACK(weapon, 1, 0, Hash40::new("top"), 6.0, 55, 60, 0, 38, 7.0, 0.0, 0.0, -5.7, Some(0.0), Some(5.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
    }
    else {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 75, 10, 0, 20, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0.0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["game_hit_lb","game_hitair_lb"], category = ACMD_GAME)]
pub unsafe fn hit_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) == false {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 14.0, 0.0, 6.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        sv_animcmd::wait(weapon.lua_state_agent, 28.0);
        if is_excute(weapon) {
            ATTACK(weapon, 0, 1, Hash40::new("top"), 3.0, 48, 226, 0, 50, 14.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        }
        sv_animcmd::wait(weapon.lua_state_agent, 2.0);
        if is_excute(weapon) {
            AttackModule::clear_all(module_accessor);
        }
    }
    else {
        if is_excute(weapon) {
            ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 366, 20, 0, 5, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 9.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        sv_animcmd::wait(weapon.lua_state_agent, 30.0);
        if is_excute(weapon) {
            ATTACK(weapon, 0, 1, Hash40::new("top"), 6.0, 50, 100, 0, 40, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 9.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
        sv_animcmd::wait(weapon.lua_state_agent, 5.0);
        if is_excute(weapon) {
            AttackModule::clear_all(module_accessor);
        }
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_regular_lb", category = ACMD_EFFECT)]
pub unsafe fn effect_regular_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        for _ in 0..100 {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_damage_fire"),Hash40::new("top"),0,0,0,0,0,0,3,true);
            }
            sv_animcmd::wait(lua_state,3.0);
        }
    }
    else {
        for _ in 0..10000 {
            if is_excute(weapon) {
                FOOT_EFFECT(weapon, Hash40::new("cloud_hakogeki_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
            }
            sv_animcmd::wait(lua_state, 4.0);
        }
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_regularair_lb", category = ACMD_EFFECT)]
pub unsafe fn effect_regular_air_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        for _ in 0..100 {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_damage_fire"),Hash40::new("top"),0,0,0,0,0,0,3,true);
            }
            sv_animcmd::wait(lua_state,3.0);
        }
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["effect_hit_lb","effect_hitair_lb"], category = ACMD_EFFECT)]
pub unsafe fn effect_hit_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        if is_excute(weapon) {
            EFFECT(weapon,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
        }
        sv_animcmd::wait(lua_state,9.0);
        if is_excute(weapon) {
            EFFECT(weapon,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
        }
        sv_animcmd::wait(lua_state,9.0);
        if is_excute(weapon) {
            EFFECT(weapon,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
        }
        sv_animcmd::wait(lua_state,3.0);
        if is_excute(weapon) {
            EFFECT(weapon,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,0,0,0,0,0,1.2,0,0,0,0,0,0,false);
        }
    }
    else {
        if is_excute(weapon) {
            EFFECT(weapon, Hash40::new("cloud_hakogeki_shockwave"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script(agent = "cloud_wave", script = "sound_regular_lb", category = ACMD_SOUND)]
pub unsafe fn sound_regular_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        if is_excute(weapon) {
            PLAY_SE(weapon,Hash40::new("se_item_killer_attack"));
        }
    }
    else {
        if is_excute(weapon) {
            PLAY_STATUS(weapon,Hash40::new("se_cloud_special_n02"));
        }
    }
}

#[acmd_script(agent = "cloud_wave", script = "sound_regularair_lb", category = ACMD_SOUND)]
pub unsafe fn sound_regular_air_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        if is_excute(weapon) {
            PLAY_SE(weapon,Hash40::new("se_item_killer_attack"));
        }
    }
    else {
        if is_excute(weapon) {
            PLAY_STATUS(weapon,Hash40::new("se_cloud_special_n06"));
        }
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["sound_hit_lb","sound_hitair_lb"], category = ACMD_SOUND)]
pub unsafe fn sound_hit_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_METEOR) {
        if is_excute(weapon) {
            PLAY_SE(weapon,Hash40::new("se_item_smartbomb_exp"));
        }
    }
    else {
        if is_excute(weapon) {
            PLAY_STATUS(weapon,Hash40::new("se_cloud_special_n04"));
        }
    }
}