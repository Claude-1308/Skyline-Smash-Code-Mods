use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

use crate::moveset::*;
use crate::moveset::modules::*;

#[acmd_script(agent = "cloud", script = "effect_attack13", category = ACMD_EFFECT)]
pub unsafe fn effect_13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,3);
        CloudModule::flare_generator(fighter,0.4);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("top"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackdash", category = ACMD_EFFECT)]
pub unsafe fn effect_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        //let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
        //let (r,g,b) = CloudModule::speedline_recolor(magic_type);
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
            EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.8, true, 0.0, 1.0, 1.0);
        }
        else {
            EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.8, true, 0.627, 1, 0.674);
        }
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.3);
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
            EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0, 1, 1);
        }
        else {
            /*if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_COLOR(fighter,1,0.55,0);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_COLOR(fighter, 0, 1, 1);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_COLOR(fighter,1,1,0);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_COLOR(fighter,0.5,1,0);
            }*/
            EFFECT_FOLLOW(fighter, Hash40::new("cloud_dash_slash"), Hash40::new("top"), 1.5, 1.5, 6, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.627, 1, 0.674);
        }
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("cloud_speedline"), false, true);
    }
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attacks3", category = ACMD_EFFECT)]
pub unsafe fn effect_s3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if PostureModule::lr(module_accessor) > 0.0 {
        sv_animcmd::frame(lua_state,8.0);
        if is_excute(fighter) {
            CloudModule::trail_generator(fighter,7);
            CloudModule::flare_generator(fighter,0.4);
        }
    }
    else {
        sv_animcmd::frame(lua_state,7.0);
        if is_excute(fighter) {
            CloudModule::trail_generator(fighter,7);
            CloudModule::flare_generator(fighter,0.4);
        }
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackhi3", category = ACMD_EFFECT)]
pub unsafe fn effect_hi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,5);
        CloudModule::flare_generator(fighter,0.39);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "game_attacks4", category = ACMD_GAME)]
pub unsafe fn attack_s4_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        CloudModule::change_sword(module_accessor,hash40("mythril_saber_r"));
        MotionModule::set_rate(module_accessor,1.4);
    }
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,1.4);
    }
    sv_animcmd::frame(lua_state,21.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,1.0);
        ATTACK(fighter,0,0,Hash40::new("top"),5.0,368,0,0,20,5.0,0.0,8.0,3.0,Some(0.0),Some(2.0),Some(12.0),1.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CLOUD_HIT,*ATTACK_REGION_SWORD);
        let vec2 = Vector2f{x: 25.0, y: 30.0};
        AttackModule::set_vec_target_pos(module_accessor,0,Hash40::new("top"),&vec2,15,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,13.0,false);
    }
    sv_animcmd::frame(lua_state,23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,0.8);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,8);
    }
    sv_animcmd::frame(lua_state,37.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,1.0);
        ATTACK(fighter,0,0,Hash40::new("haver"),15.0,60,75,0,70,5.0,0.0,15.0,0.0,None,None,None,1.2,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("haver"),15.0,60,75,0,70,5.0,0.0,10.0,0.0,None,None,None,1.2,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("haver"),15.0,60,75,0,70,5.0,0.0,5.0,0.0,None,None,None,1.2,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,0);
        AttackModule::clear_all(module_accessor);
        ArticleModule::set_flag(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,true,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM);
        ArticleModule::generate_article(module_accessor,*FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,-1);
    }
    sv_animcmd::frame(lua_state,47.0);
    if is_excute(fighter) {
        //WorkModule::off_flag(module_accessor,Cloud::FSMASH_BEAM as i32);
    }
    sv_animcmd::frame(lua_state,69.0);
    if is_excute(fighter) {
        CloudModule::reset_swords(module_accessor);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attacks4", category = ACMD_EFFECT)]
pub unsafe fn effect_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize;
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 17, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        if SWORDS[color_id] {
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
            else {
                /*if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_red"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }*/
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
        }
        else {
            CloudModule::trail_generator(fighter,4);
            CloudModule::flare_generator(fighter,0.5);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state,23.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        if SWORDS[color_id] {
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
            else {
                /*if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_red"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }
                else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                    AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                }*/
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0.0, 1.5, 1.0, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 330, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
        }
        else {
            CloudModule::trail_generator(fighter,4);
            CloudModule::flare_generator(fighter,0.5);
        }
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,42.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_attacks4", category = ACMD_SOUND)]
pub unsafe fn sound_s4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_common_smash_start_02"));
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_swing_m"));
        PLAY_SEQUENCE(fighter,Hash40::new("seq_cloud_rnd_attack_smash_s"));
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_cloud_attack07"));
        PLAY_SE(fighter,Hash40::new("se_cloud_attackair_f01"));
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_smash_l01"));
    }
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
        MotionModule::set_rate(module_accessor,0.6);
        CloudModule::change_sword(module_accessor,hash40("hardedge_r"));
    }
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,0.6);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,17.0,false);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,23.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,17.0,false);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,28.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,17.0,false);
    }
    sv_animcmd::frame(lua_state,29.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,32.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,150,0,0,90,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,0.0,-20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,3.0,-15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,150,0,0,50,5.0,0.0,6.0,-10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_B,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,17.0,false);
    }
    sv_animcmd::frame(lua_state,34.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,38.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("top"),0.5,85,10,0,5,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,0.0,20.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,4,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,3.0,15.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        ATTACK(fighter,5,0,Hash40::new("top"),0.5,85,0,0,20,5.0,0.0,6.0,10.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,true,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_A,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(module_accessor,0,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,1,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,2,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,3,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,4,17.0,false);
        AttackModule::set_add_reaction_frame_revised(module_accessor,5,17.0,false);
    }
    sv_animcmd::frame(lua_state,40.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,8);
    }
    sv_animcmd::frame(lua_state,47.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("haver"),20.0,70,90,0,35,4.0,0.0,20.0,0.0,None,None,None,2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,1,0,Hash40::new("haver"),20.0,70,90,0,35,4.0,0.0,15.0,0.0,None,None,None,2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,2,0,Hash40::new("haver"),20.0,70,90,0,35,4.0,0.0,10.0,0.0,None,None,None,2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        ATTACK(fighter,3,0,Hash40::new("haver"),20.0,70,90,0,35,4.0,0.0,5.0,0.0,None,None,None,2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state,50.0);
    if is_excute(fighter) {
        MotionModule::set_rate(module_accessor,0.33);
        QUAKE(fighter,*CAMERA_QUAKE_KIND_M);
        ATTACK(fighter,0,0,Hash40::new("top"),20.0,70,90,0,35,9.0,0.0,0.0,6.0,Some(0.0),Some(0.0),Some(12.0),2.0,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_G,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_PUNCH,*ATTACK_REGION_SWORD);
        AttackModule::clear(module_accessor,1,false);
        AttackModule::clear(module_accessor,2,false);
        AttackModule::clear(module_accessor,3,false);
    }
    sv_animcmd::frame(lua_state,52.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET);
        smash_script::damage!(fighter,*MA_MSC_DAMAGE_DAMAGE_NO_REACTION,*DAMAGE_NO_REACTION_MODE_DAMAGE_POWER,0);
        MotionModule::set_rate(module_accessor,0.5);
        AttackModule::clear_all(module_accessor);
    }
    sv_animcmd::frame(lua_state,69.0);
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
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,8);
        if SWORDS[color_id] == false {
            CloudModule::flare_generator(fighter,0.5);
        }
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,23.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, 15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,32.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,38.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, -15, 0, 0, 0, 0.5, true);
    }
    sv_animcmd::frame(lua_state,50.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sand_landing"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1, true);
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,52.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "sound_attacklw4", category = ACMD_SOUND)]
pub unsafe fn sound_lw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_common_smash_start_02"));
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_cloud_attackhard_s01"));
    }
    sv_animcmd::frame(lua_state,50.0);
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

#[acmd_script(agent = "cloud", script = "effect_attackhi4", category = ACMD_EFFECT)]
pub unsafe fn effect_hi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 17, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,13.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,4);
        CloudModule::flare_generator(fighter,0.2);
    }
    sv_animcmd::frame(lua_state,15.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackairn", category = ACMD_EFFECT)]
pub unsafe fn effect_air_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,7);
        CloudModule::flare_generator(fighter,0.2);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackairf", category = ACMD_EFFECT)]
pub unsafe fn effect_air_f(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.2);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,5);
    }
    sv_animcmd::frame(lua_state,25.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackairb", category = ACMD_EFFECT)]
pub unsafe fn effect_air_b(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.48);
        CloudModule::trail_generator(fighter,4);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter,4);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackairhi", category = ACMD_EFFECT)]
pub unsafe fn effect_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.14);
    }
    sv_animcmd::frame(lua_state,7.0);
    if is_excute(fighter) {
        CloudModule::trail_generator(fighter,3);
    }
    sv_animcmd::frame(lua_state,8.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    sv_animcmd::frame(lua_state,24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_attackairlw", category = ACMD_EFFECT)]
pub unsafe fn effect_air_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.5);
    }
    sv_animcmd::frame(lua_state,11.0);
    if is_excute(fighter) {
        //let magic_type = WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
        //let (r,g,b) = CloudModule::speedline_recolor(magic_type);
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
            EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 4, 0, -90, 0, 0, 0.8, true, 0.0, 1.0, 1.0);
        }
        else {
            EFFECT_FOLLOW_COLOR(fighter, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 4, 0, -90, 0, 0, 0.8, true, 0.627, 1, 0.674);
        }
    }
    sv_animcmd::frame(lua_state,36.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_cliffattack", category = ACMD_EFFECT)]
pub unsafe fn effect_cliff_attack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 12, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.46);
        CloudModule::trail_generator(fighter,5);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,22.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,26.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_downattackd", category = ACMD_EFFECT)]
pub unsafe fn effect_down_attack_d(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.46);
        CloudModule::trail_generator(fighter,6);
    }
    sv_animcmd::frame(lua_state,26.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_downattacku", category = ACMD_EFFECT)]
pub unsafe fn effect_down_attack_u(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.46);
        CloudModule::trail_generator(fighter,6);
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,27.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,29.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}

#[acmd_script(agent = "cloud", script = "effect_slipattack", category = ACMD_EFFECT)]
pub unsafe fn effect_slip_attack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state,18.0);
    if is_excute(fighter) {
        CloudModule::flare_generator(fighter,0.46);
        CloudModule::trail_generator(fighter,6);
    }
    sv_animcmd::frame(lua_state,28.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    sv_animcmd::frame(lua_state,30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), true, true);
    }
}