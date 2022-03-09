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
/*static mut FIGHTER_CUTIN_MANAGER_ADDR : usize = 0;

#[acmd_script(agent = "jack", script = "game_specialhi", category = ACMD_GAME)]
pub unsafe fn special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1)
            ArticleModule::change_motion(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), true, -1.0);
            SET_RATE_ARTICLE(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, 0)
        }
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR)
        }
        frame(Frame=19)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
            ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
        frame(Frame=20)
        if(is_excute){
            SEARCH(0, 0, hash40("throw"), 3.0, 1.5, 1.5, 0.7, 1.5, -15.0, -6.4, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
            SEARCH(1, 0, hash40("throw"), 5.5, 1.5, 1.5, 0.7, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
            SEARCH(2, 0, hash40("throw"), 5.5, 1.2, 1.5, 0.7, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear(ID=1, false)
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0)
        }
        frame(Frame=22)
        if(is_excute){
            SET_SEARCH_SIZE_EXIST(2, 8)
        }
        frame(Frame=23)
        if(is_excute){
            UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=27)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
            UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
        }
    });
}

#[acmd_script(agent = "jack", script = "game_specialairhi", category = ACMD_GAME)]
pub unsafe fn special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1)
        }
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE)
        }
        frame(Frame=13)
        if(is_excute){
            GroundModule::select_cliff_hangdata(FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO)
            WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE)
        }
        frame(Frame=15)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
        }
        frame(Frame=19)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
            ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
        frame(Frame=20)
        if(is_excute){
            SEARCH(0, 0, hash40("throw"), 3.0, 1.5, 1.5, 0.7, 1.5, -15.0, -6.4, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
            SEARCH(1, 0, hash40("throw"), 5.5, 1.5, 1.5, 0.7, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
            SEARCH(2, 0, hash40("throw"), 5.5, 1.2, 1.5, 0.7, COLLISION_KIND_MASK_HSR, HIT_STATUS_MASK_NORMAL, 0, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, COLLISION_PART_MASK_BODY, false)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear(ID=1, false)
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0)
        }
        frame(Frame=22)
        if(is_excute){
            SET_SEARCH_SIZE_EXIST(2, 8)
        }
        frame(Frame=23)
        if(is_excute){
            UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
        frame(Frame=24)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=26)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=27)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
            UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
        }
    });
}

#[acmd_script(agent = "jack", script = "game_specialairhithrow", category = ACMD_GAME)]
pub unsafe fn special_air_hi_throw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1)
            ArticleModule::change_motion(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi_throw"), true, -1.0)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=258, KBG=40, FKB=0, BKB=75, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=4)
        if(is_excute){
            CHECK_FINISH_CAMERA(-20, 10)
            rust {
                skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}".as_bytes().as_ptr());
                let cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
                smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(cutin_manager, 1.5);
            }
        }
        frame(Frame=6)
        if(is_excute){
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
        frame(Frame=8)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}*/

#[acmd_script(agent = "jack", scripts = ["game_specials2",  "game_specialairs2"], category = ACMD_GAME)]
pub unsafe fn special_s2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_HACHIMAKI {
        if is_excute(fighter) {
            EFFECT(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,0,0,0,0,1.0,0,0,0,0,0,0,true);
            EFFECT(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,10,0,0,0,1.0,0,0,0,0,0,0,true);
            EFFECT(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,20,0,0,0,1.0,0,0,0,0,0,0,true);
            EFFECT(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,30,0,0,0,1.0,0,0,0,0,0,0,true);
            EFFECT(fighter,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,40,0,0,0,1.0,0,0,0,0,0,0,true);
        }
        sv_animcmd::frame(lua_state,15.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter,Hash40::new("sys_thunder"),true,false);
            EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),10,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
            PLAY_SE(fighter, Hash40::new("se_jack_special_s08"));
        }
        sv_animcmd::frame(lua_state,16.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_jack_special_s08"));
            EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),20,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        }
        sv_animcmd::frame(lua_state,17.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_jack_special_s08"));
            EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),20,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        }
        sv_animcmd::frame(lua_state,18.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_jack_special_s08"));
            EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),30,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        }
        sv_animcmd::frame(lua_state,19.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_jack_special_s08"));
            EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),40,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        }
        sv_animcmd::frame(lua_state,20.0);
        if is_excute(fighter) {
            ATTACK(fighter,0,0,Hash40::new("top"),15.0,45,70,0,40,20.0,0.0,25.0,15.0,Some(0.0),Some(25.0),Some(30.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
            ATTACK(fighter,1,0,Hash40::new("top"),15.0,45,70,0,40,20.0,0.0,10.0,15.0,Some(0.0),Some(10.0),Some(30.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        }
        sv_animcmd::frame(lua_state,26.0);
        if is_excute(fighter) {
            AttackModule::clear_all(module_accessor);
        }
    }
}

#[acmd_script(agent = "jack", script = "effect_specials1", category = ACMD_EFFECT)]
pub unsafe fn effect_special_s1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            FOOT_EFFECT(hash40("null"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("jack_magic_aura"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, true)
        }
        if(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SATANAEL
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_RAOUL){
            frame(Frame=13)
            if(is_excute){
                EFFECT(hash40("jack_eiha_finger"), hash40("havel"), 1, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            }
        }
    });
}

#[acmd_script(agent = "jack", script = "effect_specialairs1", category = ACMD_EFFECT)]
pub unsafe fn effect_air_special_s1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("jack_magic_aura"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, true)
        }
        if(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SATANAEL
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_RAOUL){
            frame(Frame=13)
            if(is_excute){
                EFFECT(hash40("jack_eiha_finger"), hash40("havel"), 1, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            }
        }
        frame(Frame=18)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("jack_magic_aura"), -1)
        }
        frame(Frame=28)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("jack_magic_aura"), false, true)
        }
    });
}

#[acmd_script(agent = "jack", script = "effect_specials2", category = ACMD_EFFECT)]
pub unsafe fn effect_special_s2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            FOOT_EFFECT(hash40("null"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("jack_magic_aura"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.25, true)
        }
        if(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SATANAEL
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_RAOUL){
            frame(Frame=16)
            if(is_excute){
                EFFECT(hash40("jack_doyle_magic_flash2"), hash40("top"), 0, 17, 5, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            }
        }
    });
}

#[acmd_script(agent = "jack", script = "effect_specialairs2", category = ACMD_EFFECT)]
pub unsafe fn effect_air_special_s2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("jack_magic_aura"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.25, true)
        }
        if(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SATANAEL
        || WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_RAOUL){
            frame(Frame=16)
            if(is_excute){
                EFFECT(hash40("jack_doyle_magic_flash2"), hash40("top"), 0, 17, 5, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
            }
        }
        frame(Frame=18)
        if(is_excute){
            EFFECT_DETACH_KIND(hash40("jack_magic_aura"), -1)
        }
        frame(Frame=28)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("jack_magic_aura"), false, true)
        }
    });
}

#[acmd_script(agent = "jack_doyle", script = "game_specials2", category = ACMD_GAME)]
pub unsafe fn doyle_special_s2(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_HACHIMAKI {
        sv_animcmd::frame(lua_state,15.0);
        if is_excute(weapon) {
            ArticleModule::generate_article(module_accessor,*WEAPON_JACK_DOYLE_GENERATE_ARTICLE_FIRE2,false,-1);
        }
    }
}

#[acmd_script(agent = "jack", scripts = ["game_specialairhistart", "game_specialhistart"], category = ACMD_GAME)]
pub unsafe fn special_hi_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE {
        if is_excute(fighter) {
            ArticleModule::generate_article(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_WING,false,-1);
            ArticleModule::change_motion(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_WING,Hash40::new("special_hi2"),false,0.0);
        }
    }
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_REVERSE_LR);
    }
}

#[acmd_script(agent = "jack_fire", script = "game_fly", category = ACMD_GAME)]
pub unsafe fn fire_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let (effect,sound) = decide_box_property(WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),1.0,90,20,0,35,1.5,0.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        ATTACK(weapon,1,0,Hash40::new("top"),1.0,90,20,0,35,1.5,3.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        ATTACK(weapon,2,0,Hash40::new("top"),1.0,90,20,0,35,1.5,-3.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
            PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
                AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
            },
            PERSONA_KIND_MESSIAH | PERSONA_KIND_FORTUNA => {
                AttackModule::set_size(module_accessor,0,3.0);
                AttackModule::set_size(module_accessor,1,0.0);
                AttackModule::set_size(module_accessor,2,0.0);
            },
            PERSONA_KIND_SETANTA | PERSONA_KIND_YOSHITSUNE => {
                AttackModule::set_size(module_accessor,0,3.0);
                AttackModule::set_size(module_accessor,1,0.0);
                AttackModule::set_size(module_accessor,2,0.0);
            },
            _ => {},
        }
    }
}

#[acmd_script(agent = "jack_fire", script = "game_hit", category = ACMD_GAME)]
pub unsafe fn fire_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            acmd!(lua_state, {
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=75, KBG=210, FKB=0, BKB=20, Size=8.5, X=0.0, Y=1.5, Z=0.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
                    AttackModule::set_poison_param(ID=0, Frames=361, Rehit=45, Damage=1.0, Unk=false)
                    ControlModule::set_rumble(Hash40::new("rbkind_explosion"), 0, false, 0u32)
                }
            });
        },
        PERSONA_KIND_HACHIMAKI => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,45,100,0,10,5.0,0.0,1.5,0.0,Some(0.0),Some(4.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame_revised(module_accessor,1,5.0,false);
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,90,100,0,40,10.0,0.0,0.0,0.0,None,None,None,1.5,0.6,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_BOMB);
                ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0u32);
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,150,80,0,50,4.5,0.0,3.0,1.3,Some(0.0),Some(3.0),Some(-1.3),0.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,1,0,Hash40::new("top"),4.0,150,80,0,50,4.5,0.0,10.0,1.5,Some(0.0),Some(10.0),Some(-1.5),0.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame_revised(module_accessor,1,5.0,false);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,361,100,0,35,2.0,0.0,0.0,1.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,1,0,Hash40::new("top"),4.0,361,100,0,35,2.0,0.0,2.0,3.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,2,0,Hash40::new("top"),4.0,361,100,0,35,2.0,0.0,0.0,5.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,3,0,Hash40::new("top"),4.0,361,100,0,35,2.0,0.0,-2.0,7.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                QUAKE(weapon, *CAMERA_QUAKE_KIND_S);
                let damage = DamageModule::damage(owner_module_accessor,0);
                let kbg = (60.0 * damage/100.0) as i32;
                ATTACK(weapon,0,0,Hash40::new("top"),6.0,45,kbg,0,30,5.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_aura"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_NONE);
            }
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(weapon) {
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_explosion"),0,false,0u32);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),1.0,361,140,0,0,2.3,0.0,-1.7,2.5,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_bind"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
                ATTACK(weapon,1,0,Hash40::new("top"),3.0,361,150,0,30,3.0,0.0,-1.7,2.5,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_jack_bullet"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
                AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                ATTACK(weapon,1,0,Hash40::new("top"),4.0,70,60,0,30,3.5,0.0,7.0,0.0,Some(0.0),Some(3.0),Some(0.0),1.5,0.8,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,45,100,0,10,5.0,0.0,1.5,0.0,Some(0.0),Some(4.0),Some(0.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame_revised(module_accessor,1,5.0,false);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),4.0,58,100,0,60,5.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-5.0,-1.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_MAGIC);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "effect_end", category = ACMD_EFFECT)]
pub unsafe fn effect_fire_end(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("jack_eiha_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "effect_hitground", category = ACMD_EFFECT)]
pub unsafe fn effect_fire_hitground(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("jack_eiha_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "effect_fly", category = ACMD_EFFECT)]
pub unsafe fn effect_fire_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("jack_eiha_bullet"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, true)
                }
            });
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_windwave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.25, true);
            }
        },
        PERSONA_KIND_HACHIMAKI => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, false);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.1, true);
                LAST_EFFECT_SET_COLOR(weapon,1,0,1);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.125, true);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, true);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_normal"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "effect_hit", category = ACMD_EFFECT)]
pub unsafe fn effect_fire_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT(hash40("jack_eiha"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, true);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_hit_magic"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
                EFFECT(weapon, Hash40::new("sys_hit_magic"), Hash40::new("top"), 0, -10, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "sound_fly", category = ACMD_SOUND)]
pub unsafe fn sound_fire_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_YOSHITSUNE | PERSONA_KIND_HACHIMAKI => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s02"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s14"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s15"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s16"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s17"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s12"));
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "sound_hit", category = ACMD_SOUND)]
pub unsafe fn sound_fire_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s04"));
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s18"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s20"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s21"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s23"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s22"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s13"));
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s11"));
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire", script = "sound_hitground", category = ACMD_SOUND)]
pub unsafe fn sound_fire_hitground(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s04"));
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s18"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s20"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s21"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s23"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s22"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s13"));
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s11"));
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "game_fly", category = ACMD_GAME)]
pub unsafe fn fire2_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let (effect,sound) = decide_box_property(WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    if is_excute(weapon) {
        ATTACK(weapon,0,0,Hash40::new("top"),1.0,90,20,0,35,1.5,0.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        ATTACK(weapon,1,0,Hash40::new("top"),1.0,90,20,0,35,1.5,3.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        ATTACK(weapon,2,0,Hash40::new("top"),1.0,90,20,0,35,1.5,-3.0,-0.5,0.7,None,None,None,0.8,0.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_SPEED,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new_raw(effect),*ATTACK_SOUND_LEVEL_S,sound,*ATTACK_REGION_MAGIC);
        match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
            PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
                AttackModule::set_poison_param(module_accessor,0,321,40,1.5,false);
                AttackModule::set_poison_param(module_accessor,1,321,40,1.5,false);
                AttackModule::set_poison_param(module_accessor,2,321,40,1.5,false);
            },
            PERSONA_KIND_MESSIAH | PERSONA_KIND_FORTUNA => {
                AttackModule::set_size(module_accessor,0,6.0);
                AttackModule::set_size(module_accessor,1,0.0);
                AttackModule::set_size(module_accessor,2,0.0);
            },
            PERSONA_KIND_SETANTA | PERSONA_KIND_YOSHITSUNE => {
                AttackModule::set_size(module_accessor,0,8.0);
                AttackModule::set_size(module_accessor,1,0.0);
                AttackModule::set_size(module_accessor,2,0.0);
            },
            _ => {},
        }
    }
}

#[acmd_script(agent = "jack_fire2", script = "game_hit", category = ACMD_GAME)]
pub unsafe fn fire2_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            acmd!(lua_state, {
                frame(Frame=1)
                if(is_excute){
                    QUAKE(CAMERA_QUAKE_KIND_S)
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=0, BKB=5, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
                    AttackModule::set_poison_param(ID=0, Frames=321, Rehit=40, Damage=1.5, Unk=false)
                }
                frame(Frame=5)
                if(is_excute){
                    ControlModule::set_rumble(Hash40::new("rbkind_explosion"),0,false,0u32)
                }
                frame(Frame=6)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=0, BKB=5, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
                    AttackModule::set_poison_param(ID=0, Frames=321, Rehit=40, Damage=1.5, Unk=false)
                }
                frame(Frame=11)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=0, BKB=5, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
                    AttackModule::set_poison_param(ID=0, Frames=321, Rehit=40, Damage=1.5, Unk=false)
                }
                frame(Frame=16)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=73, KBG=105, FKB=0, BKB=105, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_curse_poison"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
                    AttackModule::set_poison_param(ID=0, Frames=321, Rehit=40, Damage=1.5, Unk=false)
                }
            });
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),1.5,160,120,0,50,4.8,0.0,0.0,0.0,None,None,None,1.0,0.4,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,-0.7,0.0,5.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_BOMB);
                AttackModule::set_no_damage_fly_smoke_all(module_accessor, true, false);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,6.0);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,7.2);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,8.4);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,9.6);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,10.8);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::set_size(module_accessor,0,12.0);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
                AREA_WIND_2ND_RAD(weapon,0,1,0.02,1000,1,0,0,29);
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_explosion"),0,false,0u32);
            }
            sv_animcmd::wait(lua_state,1.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,1,Hash40::new("top"),5.5,84,141,0,60,15.5,0.0,0.0,0.0,None,None,None,1.5,0.4,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,-2.7,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_BOMB);
            }
            sv_animcmd::wait(lua_state,1.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),1.0,130,100,60,0,7.5,0.0,5.0,2.3,Some(0.0),Some(5.0),Some(-2.3),0.5,0.5,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,true,-1.0,0.0,7.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,1,0,Hash40::new("top"),1.0,367,100,40,0,8.5,0.0,16.0,2.5,Some(0.0),Some(16.0),Some(-2.5),0.5,0.5,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,true,-1.0,0.0,7.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,21.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,1,Hash40::new("top"),5.0,135,50,0,80,7.5,0.0,5.0,2.3,Some(0.0),Some(5.0),Some(-2.3),0.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,-1.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,1,1,Hash40::new("top"),5.0,135,50,0,80,8.5,0.0,16.0,2.5,Some(0.0),Some(16.0),Some(-2.5),0.5,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,true,-1.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame_revised(module_accessor,1,10.0,false);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,100,0,35,2.0,0.0,0.0,1.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,1,0,Hash40::new("top"),5.0,361,100,0,35,2.0,0.0,2.0,3.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,2,0,Hash40::new("top"),5.0,361,100,0,35,2.0,0.0,0.0,5.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
                ATTACK(weapon,3,0,Hash40::new("top"),5.0,361,100,0,35,2.0,0.0,-2.0,7.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_MAGIC);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                QUAKE(weapon, *CAMERA_QUAKE_KIND_S);
                let damage = DamageModule::damage(owner_owner_module_accessor,0);
                let kbg = (90.0 * damage/100.0) as i32;
                ATTACK(weapon,0,0,Hash40::new("top"),12.0,45,kbg,0,30,15.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_aura"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_NONE);
            }
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(weapon) {
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_explosion"),0,false,0u32);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),1.0,361,140,0,0,2.3,0.0,-1.7,2.5,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_bind_extra"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
                ATTACK(weapon,1,0,Hash40::new("top"),3.0,361,150,0,30,3.0,0.0,-1.7,2.5,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_jack_bullet"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
                AttackModule::set_no_finish_camera(module_accessor, 0, true, false);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                AREA_WIND_2ND_RAD_arg9(weapon, 0, 1, 0.05, 200, 0.8, 0, 9, 15, 60);
                ATTACK(weapon,0,0,Hash40::new("top"),2.5,85,100,15,0,3.8,0.0,8.0,0.0,Some(0.0),Some(1.0),Some(0.0),0.5,0.8,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-0.6,0.0,8.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),2.5,85,100,15,0,4.0,0.0,9.0,0.0,Some(0.0),Some(1.0),Some(0.0),0.5,0.8,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-0.6,0.0,8.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),2.5,110,100,15,0,4.0,0.0,9.0,0.0,Some(0.0),Some(1.0),Some(0.0),0.5,0.8,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-0.6,0.0,8.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                ATTACK(weapon,1,1,Hash40::new("top"),5.0,70,80,0,50,6.5,0.0,11.0,0.0,Some(0.0),Some(3.0),Some(0.0),2.0,0.8,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-2.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::wait(lua_state,3.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            sv_animcmd::frame(lua_state,1.0);
            if is_excute(weapon) {
                QUAKE(weapon, *CAMERA_QUAKE_KIND_S);
                ATTACK(weapon,0,0,Hash40::new("top"),0.5,367,100,0,5,4.0,0.0,1.5,0.0,Some(0.0),Some(8.0),Some(0.0),0.7,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,5.0);
            if is_excute(weapon) {
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_explosion"),0,false,0u32);
            }
            sv_animcmd::frame(lua_state,10.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
            sv_animcmd::frame(lua_state,11.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),0.5,367,100,0,5,4.0,0.0,1.5,0.0,Some(0.0),Some(8.0),Some(0.0),0.7,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,20.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
            sv_animcmd::frame(lua_state,21.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),0.5,367,100,0,5,4.0,0.0,1.5,0.0,Some(0.0),Some(8.0),Some(0.0),0.7,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,30.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
            sv_animcmd::frame(lua_state,31.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),0.5,367,100,0,5,4.0,0.0,1.5,0.0,Some(0.0),Some(8.0),Some(0.0),0.7,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,40.0);
            if is_excute(weapon) {
                AttackModule::clear_all(module_accessor);
            }
            sv_animcmd::frame(lua_state,41.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),3.0,45,80,0,50,5.0,0.0,1.5,0.0,Some(0.0),Some(8.0),Some(0.0),1.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,true,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_magic"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_MAGIC,*ATTACK_REGION_MAGIC);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_furafura"),14,false,0u32);
            }
            sv_animcmd::frame(lua_state,9.0);
            if is_excute(weapon) {
                ControlModule::set_rumble(module_accessor,Hash40::new("rbkind_explosionm"),0,false,0u32);
                QUAKE(weapon, *CAMERA_QUAKE_KIND_M);
                AttackModule::disable_tip(module_accessor);
                ATTACK(weapon,0,0,Hash40::new("top"),3.7,366,100,50,0,19.3,0.0,0.0,0.0,None,None,None,0.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,true,-1.0,-1.0,3.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,11.0);
            if is_excute(weapon) {
                ATTACK(weapon,0,0,Hash40::new("top"),3.7,366,100,60,0,19.3,0.0,0.0,0.0,None,None,None,1.5,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,true,-1.0,-1.0,3.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_MAGIC);
            }
            sv_animcmd::frame(lua_state,18.0);
            if is_excute(weapon) {
                DamageModule::add_damage(owner_owner_module_accessor,6.0,0);
                ATTACK(weapon,0,1,Hash40::new("top"),9.0,58,96,0,84,19.8,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,-5.0,-1.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_MAGIC);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "effect_end", category = ACMD_EFFECT)]
pub unsafe fn effect_fire2_end(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("jack_eiha_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "effect_fly", category = ACMD_EFFECT)]
pub unsafe fn effect_fire2_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("jack_eihaon_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_windwave"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.5, true);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.2, false);
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.2, true);
                LAST_EFFECT_SET_COLOR(weapon,1,0,1);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, true);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_normal"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "effect_hit", category = ACMD_EFFECT)]
pub unsafe fn effect_fire2_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("jack_eihaon"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,4.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,1.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
            }
            sv_animcmd::frame(lua_state,6.0);
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 30, 0, 0, 1.2, true);
            }
            sv_animcmd::frame(lua_state,12.0);
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, -30, 0, 0, 1.2, true);
            }
            sv_animcmd::frame(lua_state,18.0);
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 30, 0, 0, 1.2, true);
            }
            sv_animcmd::frame(lua_state,21.0);
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.5, true);
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        },
        PERSONA_KIND_OKUNINUSHI => {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_damage_purple"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, true);
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::wait(lua_state,10.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::frame(lua_state,20.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::frame(lua_state,40.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            }
            sv_animcmd::frame(lua_state,60.0);
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("sys_hit_magic"), Hash40::new("top"), 0, 10, 5, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
                EFFECT(weapon, Hash40::new("sys_hit_magic"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
                EFFECT(weapon, Hash40::new("sys_hit_magic"), Hash40::new("top"), 0, -10, -5, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "sound_fly", category = ACMD_SOUND)]
pub unsafe fn sound_fire2_fly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s05"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s14"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s15"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s16"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s17"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s09"));
            }
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack_fire2", script = "sound_hit", category = ACMD_SOUND)]
pub unsafe fn sound_fire2_hit(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s06"));
            }
        },
        PERSONA_KIND_MESSIAH => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s18"));
            }
        },
        PERSONA_KIND_FORTUNA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s20"));
            }
        },
        PERSONA_KIND_AME => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s21"));
            }
        },
        PERSONA_KIND_SETANTA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s23"));
            }
        },
        PERSONA_KIND_ORPHEOUS => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s22"));
            }
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s10"));
            }
            sv_animcmd::frame(lua_state,20.0);
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s10"));
            }
            sv_animcmd::frame(lua_state,40.0);
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s10"));
            }
            sv_animcmd::frame(lua_state,60.0);
            if is_excute(weapon) {
                PLAY_SE_REMAIN(weapon, Hash40::new("se_jack_special_s10"));
            }
        },
        PERSONA_KIND_YOSHITSUNE => {
            if is_excute(weapon) {
                PLAY_SE(weapon, Hash40::new("se_jack_special_s11"));
            }
        },
        _ => {},
    }
}