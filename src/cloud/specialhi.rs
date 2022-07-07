use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smashline::*;

use crate::moveset::*;
use crate::moveset::modules::*;

#[acmd_script(agent = "cloud", scripts = ["game_specialhi_lb", "game_specialairhi_lb"], category = ACMD_GAME)]
pub unsafe fn special_hi_lb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS) });
        ATTACK(fighter,0,0,Hash40::new("top"),7.0,40,100,80,0,2.0,0.0,7.0,3.0,Some(0.0),Some(7.0),Some(3.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),7.0,78,100,80,0,5.0,0.0,7.0,5.0,Some(0.0),Some(7.0),Some(6.5),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),7.0,102,100,80,0,3.5,0.0,7.0,15.0,Some(0.0),Some(7.0),Some(6.5),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        acmd!(lua_state, {SA_SET(SITUATION_KIND_AIR) });
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
        ATTACK(fighter,0,1,Hash40::new("haver"),9.0,88,142,0,50,5.0,0.0,6.0,3.0,Some(0.0),Some(6.0),Some(0.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,1,Hash40::new("haver"),9.0,88,142,0,50,5.0,0.0,14.5,0.0,Some(0.0),Some(6.0),Some(0.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,1,Hash40::new("top"),9.0,88,142,0,50,3.5,0.0,7.0,15.0,Some(0.0),Some(7.0),Some(6.5),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS) });
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        ATTACK(fighter,0,1,Hash40::new("haver"),10.0,88,110,0,50,5.0,0.0,6.0,3.0,Some(0.0),Some(6.0),Some(0.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,1,Hash40::new("haver"),10.0,88,110,0,50,5.0,0.0,14.5,0.0,Some(0.0),Some(6.0),Some(0.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,1,Hash40::new("haver"),10.0,88,40,0,98,3.5,0.0,14.5,-10.0,Some(0.0),Some(6.0),Some(-10.0),2.5,1.0,*ATTACK_SETOFF_KIND_THRU,*ATTACK_LR_CHECK_F,false,5.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
    }
    FT_MOTION_RATE(fighter,2.0);
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    FT_MOTION_RATE(fighter,1.0);
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    sv_animcmd::frame(lua_state,29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x25813802b6u64) });
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS) });
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialhi", category = ACMD_EFFECT)]
pub unsafe fn effect_special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
        let (r,g,b) = CloudModule::speedline_recolor(magic_type);
        EFFECT_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, r, g, b);
        CloudModule::flare_generator(fighter,0.0);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
	    LAST_EFFECT_SET_RATE(fighter,0.6);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,12);
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialairhi", category = ACMD_EFFECT)]
pub unsafe fn effect_special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
        let (r,g,b) = CloudModule::speedline_recolor(magic_type);
        EFFECT_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, r, g, b);
        CloudModule::flare_generator(fighter,0.0);
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,12);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialhi2end", category = ACMD_EFFECT)]
pub unsafe fn effect_special_hi2_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialhi2fall", category = ACMD_EFFECT)]
pub unsafe fn effect_special_hi2_fall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.0);
        CloudModule::trail_generator(fighter,6);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialhi2landing", category = ACMD_EFFECT)]
pub unsafe fn effect_special_hi2_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        CloudModule::flare_generator(fighter,0.0);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialhi_lb", category = ACMD_EFFECT)]
pub unsafe fn effect_special_hi_lb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        EFFECT_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, 0, 1, 1);
        EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
	    LAST_EFFECT_SET_RATE(fighter,0.6);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter,0.7);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialairhi_lb", category = ACMD_EFFECT)]
pub unsafe fn effect_special_air_hi_lb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        EFFECT_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, 0, 1, 1);
        EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
    }
}