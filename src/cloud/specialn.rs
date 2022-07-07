use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

use crate::moveset::*;
use crate::moveset::modules::*;

/*#[acmd_script(agent = "cloud_wave", scripts = ["game_regular", "game_regularair"], category = ACMD_GAME)]
pub unsafe fn regular(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),12.0,55,70,0,20,3.0,0.0,11.0,-2.1,Some(0.0),Some(5.0),Some(0.0),0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,-8.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_NO_FLOOR,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_NONE);
        ATTACK(weapon,1,0,Hash40::new("top"),12.0,55,70,0,20,3.0,0.0,1.7,-0.8,Some(0.0),Some(5.0),Some(0.0),0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,-8.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_NO_FLOOR,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_NONE);
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["game_hit_lb", "game_hitair_lb"], category = ACMD_GAME)]
pub unsafe fn hit_lb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),1.5,366,100,40,0,14.0,0.0,6.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,2.0,0.0,4.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(lua_state,28.0);
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),5.0,48,226,0,50,14.0,0.0,6.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(lua_state,2.0);
    if is_excute(weapon) {
        AttackModule::clear_all(module_accessor);
    }
}

#[acmd_script(agent = "cloud_wave", scripts = ["effect_regular", "effect_regularair"], category = ACMD_EFFECT)]
pub unsafe fn effect_regular(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    for _ in 0..100 {
        if is_excute(weapon) {
            if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                FOOT_EFFECT(weapon ,Hash40::new("cloud_hakogeki_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
                LAST_EFFECT_SET_COLOR(weapon,1,0.55,0);
            }
            else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                FOOT_EFFECT(weapon ,Hash40::new("cloud_hakogeki_smoke_lb"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
            }
            else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                FOOT_EFFECT(weapon ,Hash40::new("cloud_hakogeki_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
                LAST_EFFECT_SET_COLOR(weapon,1,1,0);
            }
            else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                FOOT_EFFECT(weapon ,Hash40::new("cloud_hakogeki_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.9, 2, 0, 2, 0, 0, 0, false);
                LAST_EFFECT_SET_COLOR(weapon,0.5,1,0);
            }
        }
        sv_animcmd::wait(lua_state,4.0);
    }
}

#[acmd_script(agent = "cloud_wave", script = "game_fire", category = ACMD_GAME)]
pub unsafe fn fire(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),12.0,30,75,0,30,10.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_fire", category = ACMD_EFFECT)]
pub unsafe fn effect_fire(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    for _ in 0..25 {
        if is_excute(weapon) {
            EFFECT_FOLLOW(weapon,Hash40::new("sys_damage_fire"),Hash40::new("top"),0,0,0,0,0,0,3,true);
        }
        sv_animcmd::wait(lua_state,5.0);
    }
}

#[acmd_script(agent = "cloud_wave", script = "sound_fire", category = ACMD_SOUND)]
pub unsafe fn sound_fire(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        CloudModule::materia_sound(weapon,hash40("se_cloud_materia_fire_01"),hash40("se_cloud_materia_fire_02"));
    }
}

#[acmd_script(agent = "cloud_wave", script = "game_wind", category = ACMD_GAME)]
pub unsafe fn wind(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),5.0,135,40,0,75,10.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,12.0,false);
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_wind", category = ACMD_EFFECT)]
pub unsafe fn effect_wind(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        EFFECT_FOLLOW(weapon, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -5.0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(weapon, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -2.5, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(weapon, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(weapon, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(weapon, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.0, 0, 0, 0, 0, 1, false);
    }
}

#[acmd_script(agent = "cloud_wave", script = "sound_wind", category = ACMD_SOUND)]
pub unsafe fn sound_wind(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        PLAY_SE(weapon,Hash40::new("se_cloud_materia_aero"));
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_hit", category = ACMD_EFFECT)]
pub unsafe fn effect_hit_fire(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) {
        if WorkModule::is_flag(module_accessor,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_HIT_COLLISION_HIT) {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("cloud_hakogeki_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    LAST_EFFECT_SET_COLOR(weapon,1,0.55,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    LAST_EFFECT_SET_COLOR(weapon,0.89,1,1);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    LAST_EFFECT_SET_COLOR(weapon,1,1,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    LAST_EFFECT_SET_COLOR(weapon,0.5,1,0);
                }
            }
        }
        else {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("cloud_hakogeki_hit2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    LAST_EFFECT_SET_COLOR(weapon,1,0.55,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    LAST_EFFECT_SET_COLOR(weapon,0.89,1,1);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    LAST_EFFECT_SET_COLOR(weapon,1,1,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    LAST_EFFECT_SET_COLOR(weapon,0.5,1,0);
                }
            }
        }
    }
}

#[acmd_script(agent = "cloud_wave", script = "effect_hitair", category = ACMD_EFFECT)]
pub unsafe fn effect_hit_wind(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) {
        original!(weapon);
    }
    else {
        if is_excute(weapon) {
            EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
        }
    }
}

#[acmd_script(agent = "cloud", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME)]
pub unsafe fn special_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    FT_MOTION_RATE(fighter,0.8);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
        sv_animcmd::frame(lua_state,15.0);
        if is_excute(fighter) {
            ATTACK(fighter,0,0,Hash40::new("top"),12.0,70,60,0,30,10.0,0.0,15.0,25.0,Some(0.0),Some(15.0),Some(10.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-8.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_NONE);
        }
        sv_animcmd::frame(lua_state,20.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
    }
    else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
        sv_animcmd::frame(lua_state,15.0);
        if is_excute(fighter) {
            ATTACK(fighter,0,0,Hash40::new("top"),5.0,70,60,0,30,20.0,0.0,30.0,10.0,Some(0.0),Some(10.0),Some(10.0),1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-8.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_paralyze"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        }
        sv_animcmd::frame(lua_state,20.0);
        if is_excute(fighter) {
            AttackModule::clear(module_accessor,0,false);
            ATTACK(fighter,1,0,Hash40::new("top"),5.0,70,60,0,30,20.0,0.0,30.0,30.0,Some(0.0),Some(10.0),Some(30.0),1.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-8.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_paralyze"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        }
        sv_animcmd::frame(lua_state,25.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
    }
    else {
        sv_animcmd::frame(lua_state,15.0);
        if is_excute(fighter) {
            ArticleModule::generate_article(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,-1);
        }
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,*FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script(agent = "cloud", scripts = ["effect_specialn", "effect_specialairn"], category = ACMD_EFFECT)]
pub unsafe fn effect_special_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
    if magic_type == MAGIC_TYPE_FIRE {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 15, 5, 0, 0, 0, 3, false);
        }
    }
    else if magic_type == MAGIC_TYPE_ICE {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 10, 15, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter,0.6);
        }
    }
    else if magic_type == MAGIC_TYPE_ELEC {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        sv_animcmd::frame(lua_state,18.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else if magic_type == MAGIC_TYPE_WIND {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
        }
    }
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
        sv_animcmd::frame(lua_state,4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(agent = "cloud", scripts = ["sound_specialn", "sound_specialairn"], category = ACMD_SOUND)]
pub unsafe fn sound_special_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
    if magic_type == MAGIC_TYPE_ICE {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            CloudModule::materia_sound(fighter,hash40("se_cloud_materia_blizzard_01"),hash40("se_cloud_materia_blizzard_02"));
        }
    }
    else if magic_type == MAGIC_TYPE_ELEC {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            CloudModule::materia_sound(fighter,hash40("se_cloud_materia_thunder_01"),hash40("se_cloud_materia_thunder_02"));
        }
    }
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter,Hash40::new("se_cloud_rnd_special_n"));
    }
}

#[acmd_script(agent = "cloud", scripts = ["expression_specialn", "expression_specialairn"], category = ACMD_EXPRESSION)]
pub unsafe fn expression_special_n(_fighter: &mut L2CAgentBase) {}

#[status_script(agent = "cloud", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) == false {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_n"),0.0,1.0,false,0.0,false,false);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion(module_accessor,Hash40::new("special_n"),0.0,1.0,false,0.0,false,false);
        }
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_n"),-1.0,1.0,0.0,false,false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_n"),-1.0,1.0,0.0,false,false);
    }
    return L2CValue::I32(0)
}

unsafe fn sub_status(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::frame(module_accessor) < 1.0 {
        return L2CValue::I32(0)
    }
    if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        let mut l2c_agent = L2CAgent::new(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x199c462b5du64));
        sv_battle_object::notify_event_msc_cmd(lua_state);
    }
    else {
        WorkModule::dec_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    return L2CValue::I32(0)
}*/

#[status_script(agent = "cloud_wave", status = WEAPON_CLOUD_WAVE_STATUS_KIND_REGULAR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn regular_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if WorkModule::is_flag(owner,Cloud::FSMASH_BEAM as i32) {
        WorkModule::on_flag(module_accessor,CloudWave::FSMASH_BEAM as i32);
    }
    original!(weapon)
    /*let mut life = 0;
    if WorkModule::is_flag(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) {
        WorkModule::on_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM);
        life = CloudModule::get_params("param_wave","life") as i32;
    }
    else {
        life = WorkModule::get_param_int(module_accessor,hash40("param_wave"),hash40("life"));
    }
    wave_setup(weapon,true);
    if StopModule::is_stop(module_accessor) == false {
        sub_status(weapon,false.into());
    }
    WorkModule::set_int(module_accessor,life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(regular_loop as *const () as _))
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) == false
    && WorkModule::is_flag(owner,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) == false {
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let magic_type = WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
        if magic_type == MAGIC_TYPE_FIRE {
            let speed = CloudModule::get_params("param_wave","fire_speed") * PostureModule::lr(owner);
            weapon.clear_lua_stack();
            smash_script::lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed,0.0);
            sv_kinetic_energy::set_speed(weapon.lua_state_agent);
            let life = CloudModule::get_params("param_wave","fire_life") as i32;
            WorkModule::set_int(module_accessor,life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            MotionModule::change_motion(module_accessor,Hash40::new("fire"),0.0,1.0,false,0.0,false,false);
        }
        else if magic_type == MAGIC_TYPE_WIND {
            let speed = CloudModule::get_params("param_wave","aero_speed") * PostureModule::lr(owner);
            weapon.clear_lua_stack();
            smash_script::lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed,0.0);
            sv_kinetic_energy::set_speed(weapon.lua_state_agent);
            let life = CloudModule::get_params("param_wave","aero_life") as i32;
            WorkModule::set_int(module_accessor,life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            MotionModule::change_motion(module_accessor,Hash40::new("wind"),0.0,1.0,false,0.0,false,false);
        }
        let pos_y = CloudModule::get_params("param_wave","pos_y");
        let pos = Vector2f{x: 0.0, y: pos_y};
        PostureModule::add_pos_2d(module_accessor,&pos);
        weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_status as *const () as _));
        weapon.fastshift(L2CValue::Ptr(regular_loop as *const () as _))
    }
    else {
        wave_setup(weapon,true);
        if StopModule::is_stop(module_accessor) == false {
            sub_status_2(weapon,false.into());
        }
        weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_status_2 as *const () as _));
        weapon.fastshift(L2CValue::Ptr(regular_2_loop as *const () as _))
    }*/
}

/*unsafe fn regular_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_RIGHT as u32)
    || GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_LEFT as u32) {
        acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x199c462b5du64) });
    }
    return L2CValue::I32(0)
}

unsafe fn regular_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if StatusModule::is_changing(module_accessor) == false {
        let flag = GroundModule::get_touch_moment_flag(module_accessor);
        if flag != 0 {
            if (PostureModule::lr(module_accessor) < 0.0
            &&  flag == *GROUND_TOUCH_FLAG_LEFT as u32)
            || (PostureModule::lr(module_accessor) > 0.0
            &&  flag == *GROUND_TOUCH_FLAG_RIGHT as u32) {
                weapon.change_status(WEAPON_CLOUD_WAVE_STATUS_KIND_HIT.into(),false.into());
                return L2CValue::I32(0)
            }
            if flag != *GROUND_TOUCH_FLAG_DOWN as u32 {
                let vec2 = GroundModule::get_touch_normal(module_accessor,*GROUND_TOUCH_FLAG_DOWN as u32);
                let angle = (vec2.y/vec2.x).atan().to_degrees();
                WorkModule::set_float(module_accessor,angle,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLOAT_GROUND_ANGLE);
                let along_angle = WorkModule::get_param_float(module_accessor,hash40("param_wave"),hash40("along_angle"));
                if angle < along_angle {
                    weapon.change_status(WEAPON_CLOUD_WAVE_STATUS_KIND_HIT.into(),false.into());
                    return L2CValue::I32(0)
                }
                else {
                    if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        weapon.set_situation(SITUATION_KIND_AIR.into());
                    }
                }
            }
        }
        if weapon.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            wave_setup(weapon,false);
        }
    }
    return L2CValue::I32(0)
}

unsafe fn sub_status(weapon: &mut L2CFighterBase, unk: L2CValue) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if weapon.global_table[MOTION_FRAME].get_f32() < 1.0 {
        return L2CValue::I32(0)
    }
    if unk.get_bool() == false {
        if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
            acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x199c462b5du64) });
        }
        if weapon.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            return L2CValue::I32(0)
        }
        let angle = WorkModule::get_float(module_accessor,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLOAT_GROUND_ANGLE).to_radians().abs();
        let along_angle = WorkModule::get_param_float(module_accessor,hash40("param_wave"),hash40("along_angle"));
        if angle < along_angle {
            let rot_x = PostureModule::rot_x(module_accessor,0);
            let mut rot = PostureModule::rot(module_accessor,0);
            rot.x = rot_x;
            PostureModule::set_rot(module_accessor,&rot,0);
        }
        let speed_ground = WorkModule::get_param_float(module_accessor,hash40("param_wave"),hash40("speed_ground")) * along_angle.cos() * PostureModule::lr(module_accessor);
        weapon.clear_lua_stack();
        lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed_ground,0.0);
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    }
    else {
        WorkModule::dec_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    return L2CValue::I32(0)
}

unsafe fn wave_setup(weapon: &mut L2CFighterBase, unk: bool) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    //let owner = sv_battle_object::module_accessor(WorkModule::get_int64(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32);
    if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) {
        if weapon.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let rot_x = PostureModule::rot_x(module_accessor,0).to_radians();
            let lr = PostureModule::lr(module_accessor);
            let speed_wave = CloudModule::get_params("param_wave","speed_wave");
            let speed_x = speed_wave * rot_x.cos() * lr;
            let speed_y = speed_wave * rot_x.sin();
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed_x,speed_y);
            sv_kinetic_energy::set_speed(lua_state);
            if unk {
                MotionModule::change_motion(module_accessor,Hash40::new("regular_air"),0.0,1.0,false,0.0,false,false);
            }
            else {
                MotionModule::change_motion_force_inherit_frame(module_accessor,Hash40::new("regular_air"),-1.0,1.0,0.0);
            }
            EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave1"),true,true);
            weapon.clear_lua_stack();
            lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave2"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
            sv_module_access::effect(lua_state);
            WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave2") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            let lr = PostureModule::lr(module_accessor);
            let speed_ground = CloudModule::get_params("param_wave","speed_wave") * lr;
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed_ground,0.0);
            sv_kinetic_energy::set_speed(lua_state);
            if unk {
                MotionModule::change_motion(module_accessor,Hash40::new("regular"),0.0,1.0,false,0.0,false,false);
            }
            else {
                MotionModule::change_motion_force_inherit_frame(module_accessor,Hash40::new("regular"),-1.0,1.0,0.0);
            }
            EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave2"),true,true);
            weapon.clear_lua_stack();
            lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave1"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
            sv_module_access::effect(lua_state);
            WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave1") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
        }
    }
    else {
        if weapon.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let rot_x = PostureModule::rot_x(module_accessor,0).to_radians();
            let lr = PostureModule::lr(module_accessor);
            let speed_air = WorkModule::get_param_float(module_accessor,hash40("param_wave"),hash40("speed_air"));
            let speed_x = speed_air * rot_x.cos() * lr;
            let speed_y = speed_air * rot_x.sin();
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed_x,speed_y);
            sv_kinetic_energy::set_speed(lua_state);
            if unk {
                MotionModule::change_motion(module_accessor,Hash40::new("regular_air"),0.0,1.0,false,0.0,false,false);
            }
            else {
                MotionModule::change_motion_force_inherit_frame(module_accessor,Hash40::new("regular_air"),-1.0,1.0,0.0);
            }
            if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO) != 0 {
                EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave1_lb"),true,true);
                weapon.clear_lua_stack();
                lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave2_lb"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
                sv_module_access::effect(lua_state);
                WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave2_lb") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            }
            else {
                EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave1"),true,true);
                weapon.clear_lua_stack();
                lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave2"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
                sv_module_access::effect(lua_state);
                /*if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    LAST_EFFECT_SET_COLOR(weapon,1,0.55,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    LAST_EFFECT_SET_COLOR(weapon,0.89,1,1);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    LAST_EFFECT_SET_COLOR(weapon,1,1,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    LAST_EFFECT_SET_COLOR(weapon,0.5,1,0);
                }*/
                WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave2") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            }
        }
        else {
            GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            let lr = PostureModule::lr(module_accessor);
            let speed_ground = WorkModule::get_param_float(module_accessor,hash40("param_wave"),hash40("speed_ground")) * lr;
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,speed_ground,0.0);
            sv_kinetic_energy::set_speed(lua_state);
            if unk {
                MotionModule::change_motion(module_accessor,Hash40::new("regular"),0.0,1.0,false,0.0,false,false);
            }
            else {
                MotionModule::change_motion_force_inherit_frame(module_accessor,Hash40::new("regular"),-1.0,1.0,0.0);
            }
            if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO) != 0 {
                EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave2_lb"),true,true);
                weapon.clear_lua_stack();
                lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave1_lb"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
                sv_module_access::effect(lua_state);
                WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave1_lb") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            }
            else {
                EffectModule::kill_kind(module_accessor,Hash40::new("cloud_hakogeki_wave2"),true,true);
                weapon.clear_lua_stack();
                lua_args!(weapon,*MA_MSC_CMD_EFFECT_EFFECT_FOLLOW,hash40("cloud_hakogeki_wave1"),hash40("top"),0.0,0.0,0.0,0.0,0.0,0.0,1.0,false);
                sv_module_access::effect(lua_state);
                /*if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    LAST_EFFECT_SET_COLOR(weapon,1,0.55,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    LAST_EFFECT_SET_COLOR(weapon,0.89,1,1);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    LAST_EFFECT_SET_COLOR(weapon,1,1,0);
                }
                else if WorkModule::get_int(owner,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    LAST_EFFECT_SET_COLOR(weapon,0.5,1,0);
                }*/
                WorkModule::set_int64(module_accessor,hash40("cloud_hakodeki_wave1") as i64,*WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND);
            }
        }
    }
}*/

#[status_script(agent = "cloud_wave", status = WEAPON_CLOUD_WAVE_STATUS_KIND_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn hit_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    weapon.clear_lua_stack();
    lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,0.0,0.0);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO) != 0
    && WorkModule::is_flag(module_accessor,CloudWave::FSMASH_BEAM as i32) == false {
        if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(module_accessor,Hash40::new("hit_lb"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("hit_air_lb"),0.0,1.0,false,0.0,false,false);
        }
    }
    else {
        if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(module_accessor,Hash40::new("hit"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion(module_accessor,Hash40::new("hit_air"),0.0,1.0,false,0.0,false,false);
        }
    }
    weapon.fastshift(L2CValue::Ptr(hit_main_loop as *const () as _))
}

unsafe fn hit_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::motion_kind(module_accessor) == hash40("hit_lb")
    || MotionModule::motion_kind(module_accessor) == hash40("hit_air_lb") {
        if AttackModule::is_attack(module_accessor,0,false) {
            return L2CValue::I32(0)
        }
    }
    else {
        if MotionModule::frame(module_accessor) < 1.0 {
            return L2CValue::I32(0)
        }
    }
    acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x199c462b5du64) });
    return L2CValue::I32(0)
}