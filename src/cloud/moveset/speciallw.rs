use smash::lib::{L2CValue,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use std::arch::asm;

use crate::moveset::*;

#[acmd_script(agent = "cloud", scripts = ["game_speciallwstance", "game_specialairlwstance"], category = ACMD_GAME)]
pub unsafe fn special_lw_stance(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        CloudModule::change_sword(module_accessor,hash40("twin_stingers_r"));
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK);
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", scripts = ["effect_speciallwstance", "effect_specialairlwstance"], category = ACMD_EFFECT)]
pub unsafe fn effect_lw_stance(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.75);
    }
    sv_animcmd::wait(lua_state,1.0);
    for _ in 0..4 {
        if is_excute(fighter) {
            FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
        }
        sv_animcmd::wait(lua_state,2.0);
        if is_excute(fighter) {
            FLASH(fighter, 0.67, 0, 0.78, 0.31);
        }
        sv_animcmd::wait(lua_state,2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        sv_animcmd::wait(lua_state,2.0);
    }
}

#[acmd_script(agent = "cloud", scripts = ["sound_speciallwstance", "sound_specialairlwstance"], category = ACMD_SOUND)]
pub unsafe fn sound_lw_stance(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_special_l04"));
    }
}

#[acmd_script(agent = "cloud", scripts = ["game_speciallwstrike", "game_specialairlwstrike"], category = ACMD_GAME)]
pub unsafe fn special_lw_strike(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),1.0,50,70,0,40,5.0,0.0,15.0,0.0,Some(0.0),Some(15.0),Some(-8.0),1.2,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("haver"),1.0,50,70,0,40,5.0,0.0,10.0,0.0,Some(0.0),Some(10.0),Some(-8.0),1.2,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("haver"),1.0,50,70,0,40,5.0,0.0,5.0,0.0,Some(0.0),Some(5.0),Some(-8.0),1.2,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        acmd!(lua_state, { sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0) });
    }
    sv_animcmd::frame(lua_state,50.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_speciallwstrike", category = ACMD_EFFECT)]
pub unsafe fn effect_lw_strike(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
    sv_animcmd::frame(lua_state,35.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(agent = "cloud", script = "effect_specialairlwstrike", category = ACMD_EFFECT)]
pub unsafe fn effect_air_lw_strike(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        if SWORDS[color_id] == false {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.5);
        }
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 6, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    sv_animcmd::frame(lua_state,21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", scripts = ["sound_speciallwstrike", "sound_specialairlwstrike"], category = ACMD_SOUND)]
pub unsafe fn sound_lw_strike(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_swing_l"));
        PLAY_SE(fighter,Hash40::new("vc_cloud_attack03"));
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_special_s01"));
    }
}

unsafe fn air_stall_start(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.clear_lua_stack();
    let speed_y = KineticModule::get_sum_speed_y(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,speed_y,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.6);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_x = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(module_accessor); 
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,*ENERGY_CONTROLLER_RESET_TYPE_MOVE_AIR,speed_x/3.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL,1.0,0.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub unsafe fn special_lw_stance_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK);
    WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT);
    fighter.global_table["damage_prev"] = DamageModule::damage(module_accessor,0).into();
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        air_stall_start(fighter);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw_stance"),0.0,0.8,false,0.0,false,false);
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(module_accessor,Hash40::new("special_lw_stance"),0.0,0.8,false,0.0,false,false);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_stance_main_loop as *const () as _))
}

unsafe fn special_lw_stance_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT) {
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END.into(),false.into());
    }
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_CHECK) {
        DamageModule::set_damage_mul(module_accessor,0.5);
        acmd!(lua_state, { sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0) });
        if StopModule::is_hit(module_accessor) && StopModule::is_damage(module_accessor) {
            WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT);
        }
        if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_COUNTER_HIT)
        && StopModule::is_damage(module_accessor) {
            fighter.global_table["damage_curr"] = DamageModule::damage(module_accessor,0).into();
            let mut attacker_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
            let mut mask: u32 = 0x1;
            let mut counter = 0;
            loop {
                if ((attacker_id as u32) & !mask) == 0 {
                    break;
                }
                counter += 1;
                mask |= mask << 1;
            }
            attacker_id &= 1 << counter;
            let array_id_f = (attacker_id as f32).log2();
            let mut array_id_i: i32 = -1;
            if array_id_f.is_normal() || array_id_f == 0.0 {
                array_id_i = array_id_f as i32;
            }
            if array_id_i >= 0 && array_id_i < 8 {
                let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
                let entry = smash::app::lua_bind::FighterManager::get_fighter_entry(fighter_manager,FighterEntryID(array_id_i));
                let battle_object = *(entry as *mut u64).add(0x4160/8) as *mut u64;
                let attacker_boma = *battle_object.add(0x20/8) as *mut BattleObjectModuleAccessor;
                if PostureModule::pos_x(module_accessor) - PostureModule::pos_x(attacker_boma) < 0.0 {
                    PostureModule::set_lr(module_accessor,1.0);
                }
                else {
                    PostureModule::set_lr(module_accessor,-1.0);
                }
            }
            EFFECT_FOLLOW(fighter,Hash40::new("sys_counter_flash"),Hash40::new("top"),0,7,3,0,0,0,1,false);
            LAST_EFFECT_SET_RATE(fighter,0.5);
            PLAY_SE(fighter,Hash40::new("se_cloud_special_l01"));
        }
    }
    else {
        DamageModule::set_damage_mul(module_accessor,1.0);
        acmd!(lua_state, { sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0) });
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_lw_stance"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        air_stall_start(fighter);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_lw_stance"),-1.0,1.0,0.0,false,false);
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

unsafe fn air_stall(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY,0.0,0.0,0.0,0.0,0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,0.5);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub unsafe fn special_lw_strike_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        air_stall(fighter);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw_strike"),0.0,1.0,false,0.0,false,false);
    }
    else {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(module_accessor,Hash40::new("special_lw_strike"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_strike_main_loop as *const () as _))
}

unsafe fn special_lw_strike_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if AttackModule::is_attack(module_accessor,0,false) {
        let damage_taken = (fighter.global_table["damage_curr"].get_f32() - fighter.global_table["damage_prev"].get_f32()) * (1.2/0.5);
        AttackModule::set_power(module_accessor,0,damage_taken,false);
        AttackModule::set_power(module_accessor,1,damage_taken,false);
        AttackModule::set_power(module_accessor,2,damage_taken,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_lw_strike"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_lw_strike"),-1.0,1.0,0.0,false,false);
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