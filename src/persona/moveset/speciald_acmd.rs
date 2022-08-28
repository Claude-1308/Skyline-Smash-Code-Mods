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

#[acmd_script(agent = "jack", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME)]
pub unsafe fn special_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    match kind {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
                ATTACK(fighter,0,0,Hash40::new("top"),12.0,70,80,0,50,6.0,0.0,0.0,15.0,Some(0.0),Some(15.0),Some(15.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_poison"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
                ATTACK(fighter,1,0,Hash40::new("top"),12.0,70,80,0,50,6.0,0.0,0.0,-15.0,Some(0.0),Some(15.0),Some(-15.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_poison"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
                AttackModule::set_poison_param(module_accessor,0,60,300,1.0,false);
                AttackModule::set_poison_param(module_accessor,1,60,300,1.0,false);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_HACHIMAKI => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                ATTACK(fighter,0,0,Hash40::new("top"),1.0,361,0,0,0,10.0,0.0,10.0,20.0,Some(0.0),Some(30.0),Some(20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_MAGIC);
                ATTACK(fighter,1,0,Hash40::new("top"),1.0,361,0,0,0,10.0,0.0,10.0,-20.0,Some(0.0),Some(30.0),Some(-20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_MESSIAH => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_ON,*COLLISION_KIND_SHIELD,0,*FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_ON,*COLLISION_KIND_REFLECTOR,*FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW,*FIGHTER_REFLECTOR_GROUP_EXTEND);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_OFF,*COLLISION_KIND_SHIELD,0,*FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_OFF,*COLLISION_KIND_REFLECTOR,*FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW,*FIGHTER_REFLECTOR_GROUP_EXTEND);
            }
        },
        PERSONA_KIND_FORTUNA => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
                ATTACK(fighter,0,0,Hash40::new("top"),1.0,88,390,0,60,20.0,0.0,20.0,-15.0,Some(0.0),Some(20.0),Some(15.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_AME => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                ATTACK(fighter,0,0,Hash40::new("top"),10.0,50,90,0,30,8.0,0.0,15.0,5.0,Some(0.0),Some(15.0),Some(15.0),1.0,1.0,*ATTACK_SETOFF_KIND_NO_STOP,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(fighter,1,0,Hash40::new("top"),10.0,50,90,0,30,8.0,0.0,15.0,-5.0,Some(0.0),Some(15.0),Some(-15.0),1.0,1.0,*ATTACK_SETOFF_KIND_NO_STOP,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_SETANTA => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_ALWAYS,0);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                ATTACK(fighter,0,0,Hash40::new("top"),5.0,361,50,0,20,5.0,0.0,0.0,5.0,Some(0.0),Some(0.0),Some(20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_slip"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
                ATTACK(fighter,1,0,Hash40::new("top"),5.0,361,50,0,20,5.0,0.0,0.0,-5.0,Some(0.0),Some(0.0),Some(-20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_slip"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_ON,*COLLISION_KIND_REFLECTOR,*FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW,*FIGHTER_REFLECTOR_GROUP_EXTEND);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
                smash_script::shield!(fighter,*MA_MSC_CMD_SHIELD_OFF,*COLLISION_KIND_REFLECTOR,*FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW,*FIGHTER_REFLECTOR_GROUP_EXTEND);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_ALWAYS,0);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_ALWAYS,0);
                DamageModule::set_damage_mul(module_accessor,0.001);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
                DamageModule::set_damage_mul(module_accessor,1.0);
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                let dmg_mul = JackModule::get_params("param_special_lw","yoshitsune_dmg_mul");
                DamageModule::set_damage_mul(module_accessor,dmg_mul);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_ALWAYS,0);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                DamageModule::set_damage_mul(module_accessor,1.0);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
            }
        },
        _ => {},
    };
}

#[acmd_script(agent = "jack", scripts = ["game_speciallwcounter", "game_specialairlwcounter"], category = ACMD_GAME)]
pub unsafe fn special_lw_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    match kind {
        PERSONA_KIND_HACHIMAKI => {
            if is_excute(fighter) {
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
                ATTACK(fighter,0,1,Hash40::new("top"),15.0,361,80,0,40,10.0,0.0,10.0,20.0,Some(0.0),Some(30.0),Some(20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_MAGIC);
                ATTACK(fighter,1,1,Hash40::new("top"),15.0,361,80,0,40,10.0,0.0,10.0,-20.0,Some(0.0),Some(30.0),Some(-20.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_MESSIAH => {
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(fighter) {
                ATTACK(fighter,1,0,Hash40::new("top"),12.0,361,51,0,80,20.0,0.0,10.5,-5.0,Some(0.0),Some(10.5),Some(10.0),0.75,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_ENERGY);
                AttackModule::set_force_reaction(module_accessor,1,true,false);
            }
            sv_animcmd::frame(lua_state,8.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(fighter) {
                WorkModule::set_float(module_accessor,20.0,*FIGHTER_JACK_STATUS_SPECIAL_LW2_FLOAT_SHIELD_ATTACK_POWER);
                QUAKE(fighter,*CAMERA_QUAKE_KIND_S);
                ATTACK(fighter,0,0,Hash40::new("top"),20.0,50,90,0,40,15.0,0.0,15.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_aura"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(fighter) {
                WorkModule::set_float(module_accessor,2.0,*FIGHTER_JACK_STATUS_SPECIAL_LW2_FLOAT_SHIELD_ATTACK_POWER);
                WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_GAUGE);
                ATTACK(fighter,0,0,Hash40::new("top"),2.0,50,70,0,40,10.0,0.0,10.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(fighter) {
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
                DamageModule::set_damage_mul(module_accessor,1.0);
                WorkModule::set_float(module_accessor,10.0,*FIGHTER_JACK_STATUS_SPECIAL_LW2_FLOAT_SHIELD_ATTACK_POWER);
                ATTACK(fighter,0,0,Hash40::new("top"),10.0,80,70,0,30,15.0,0.0,15.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,5.0);
            if is_excute(fighter) {
                AttackModule::clear_all(module_accessor);
                smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_NORMAL,0);
            }
        },
        _ => {},
    };
}

#[acmd_script(agent = "jack", scripts = ["effect_speciallw", "effect_specialairlw"], category = ACMD_EFFECT)]
pub unsafe fn effect_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        FLASH(fighter,1,1,1,0.75);
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        FLASH(fighter,0.7,0.7,0.7,0.5);
    }
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    match kind {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter,Hash40::new("jack_eihaon"),Hash40::new("top"),0,0,15,0,45,0,1,true);
                EFFECT_FOLLOW(fighter,Hash40::new("jack_eihaon"),Hash40::new("top"),0,0,-15,0,45,0,1,true);
            }
        },
        PERSONA_KIND_HACHIMAKI => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,20,0,0,0,1,true);
                EFFECT_FOLLOW(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,-20,0,0,0,1,true);
            }
            sv_animcmd::frame(lua_state,32.0);
            if is_excute(fighter) {
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_thunder"),true,false);
            }
        },
        PERSONA_KIND_FORTUNA => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_club_tornado"),Hash40::new("top"),0,15,10,0,0,0,1.25,true);
                EFFECT_FOLLOW(fighter,Hash40::new("sys_club_tornado"),Hash40::new("top"),0,15,0,0,0,0,1.25,true);
                EFFECT_FOLLOW(fighter,Hash40::new("sys_club_tornado"),Hash40::new("top"),0,15,-10,0,0,0,1.25,true);
            }
        },
        PERSONA_KIND_AME => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 15, 10, 0, 0, 0, 1, false);
                LAST_EFFECT_SET_RATE(fighter,0.3);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 15, -10, 0, 0, 0, 1, false);
                LAST_EFFECT_SET_RATE(fighter,0.3);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_COLOR(fighter,Hash40::new("sys_windwave"),Hash40::new("top"),0,2,0,0,0,0,3,true,1,0,1);
            }
        },
        _ => {},
    };
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        FLASH(fighter,0.67,0,0.78,0.31);
    }
    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_MESSIAH {
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter,Hash40::new("jack_counter_start"),Hash40::new("top"),0,10,0,0,0,0,0.7,true);
            LAST_EFFECT_SET_RATE(fighter,1.7);
        }
    }
    for _ in 0..4 {
        if is_excute(fighter) {
            FLASH(fighter,0.7,0.7,0.7,0.5);
        }
        sv_animcmd::wait(lua_state,2.0);
        if is_excute(fighter) {
            FLASH(fighter,0.67,0,0.78,0.31);
        }
        sv_animcmd::wait(lua_state,2.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
        }
        sv_animcmd::wait(lua_state,2.0);
    }
    if is_excute(fighter) {
        FLASH(fighter,0.7,0.7,0.7,0.5);
    }
    sv_animcmd::wait(lua_state,1.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

#[acmd_script(agent = "jack", scripts = ["effect_speciallwcounter", "effect_specialairlwcounter"], category = ACMD_EFFECT)]
pub unsafe fn effect_lw_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    match kind {
        PERSONA_KIND_HACHIMAKI => {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),0,0,20,0,0,0,1,true);
                EFFECT_FOLLOW(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),0,0,-20,0,0,0,1,true);
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_thunder"),true,false);
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(fighter) {
                EFFECT_FOLLOW_NO_STOP(fighter,Hash40::new("jack_tetra"),Hash40::new("top"),0,10,0,0,0,0,0.7,true);
                LAST_EFFECT_SET_RATE(fighter,1.4);
            }
            if WorkModule::is_flag(module_accessor,*FIGHTER_JACK_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
                if is_excute(fighter) {
                    EFFECT(fighter,Hash40::new("sys_counter_flash"),Hash40::new("top"),0,11,0,0,0,0,1,0,0,0,0,0,0,false);
                }
            }
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_NO_STOP(fighter,Hash40::new("sys_attack_speedline"),Hash40::new("top"),7,15,-2,180,0,0,1.1,true);
                LAST_EFFECT_SET_COLOR(fighter,0.47,0.84,1.1);
                LAST_EFFECT_SET_RATE(fighter,0.5);
                EFFECT(fighter,Hash40::new("jack_tetra_attack_line"),Hash40::new("top"),7,15,-5,0,0,0,0.8,0,0,0,0,0,0,true);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1, true);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW_COLOR(fighter,Hash40::new("sys_whirlwind_l"),Hash40::new("top"),0,5,0,0,0,0,1,true,1,1,0);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            sv_animcmd::frame(lua_state,4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,15,0,0,0,0,1,true);
            }
        },
        _ => {},
    };
}

#[acmd_script(agent = "jack", scripts = ["sound_speciallw", "sound_specialairlw"], category = ACMD_SOUND)]
pub unsafe fn sound_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    sv_animcmd::frame(lua_state,4.0);
    match kind {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s06"));
            }
        },
        PERSONA_KIND_MESSIAH | PERSONA_KIND_OKUNINUSHI => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_l04"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s20"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s19"));
            }
        },
        _ => {},
    };
}

#[acmd_script(agent = "jack", scripts = ["sound_speciallwcounter", "sound_specialairlwcounter"], category = ACMD_SOUND)]
pub unsafe fn sound_lw_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
    sv_animcmd::frame(lua_state,4.0);
    match kind {
        PERSONA_KIND_HACHIMAKI => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s08"));
            }
        },
        PERSONA_KIND_MESSIAH => {
            sv_animcmd::frame(lua_state,1.0);
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_l07"));
            }
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(fighter) {
                PLAY_SEQUENCE(fighter,Hash40::new("seq_jack_rnd_special_l02"));
                PLAY_SE(fighter,Hash40::new("se_jack_special_l05"));
                PLAY_SE(fighter,Hash40::new("se_jack_special_l06"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s06"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s07"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(fighter) {
                PLAY_SE(fighter,Hash40::new("se_jack_special_s24"));
            }
        },
        _ => {},
    };
}