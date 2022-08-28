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

#[smashline::fighter_frame(agent = FIGHTER_KIND_JACK)]
pub fn joker(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if smashball::is_training_mode() {
            if JackModule::sumi_id(module_accessor) {
                WorkModule::set_int(module_accessor,PERSONA_KIND_CENDRILLON,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
            else {
                WorkModule::set_int(module_accessor,PERSONA_KIND_ARSENE,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
            }
        }
        if JackModule::sumi_id(module_accessor) == false { //joker
            if WorkModule::is_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA) == false
            && ArticleModule::is_flag(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) == false {
                if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) == true { //choose forward
                    let preset_persona = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == preset_persona {
                        WorkModule::set_int(module_accessor,-1,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    }
                    WorkModule::add_int(module_accessor,1,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) >= PERSONA_KIND_ARSENE {
                        WorkModule::set_int(module_accessor,preset_persona,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    }
                    WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA);
                }
                else if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) == true { //choose backwards
                    let preset_persona = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == preset_persona {
                        WorkModule::set_int(module_accessor,9,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    }
                    WorkModule::add_int(module_accessor,-1,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    if WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) < 0 {
                        WorkModule::set_int(module_accessor,preset_persona,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    }
                    WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA);
                }
                else if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) == true { //reset
                    let preset_persona = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                    WorkModule::set_int(module_accessor,preset_persona,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
                    WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA);
                }
            }
            if WorkModule::is_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA) {
                match WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) { //added vc entries for persona name calls
                    PERSONA_KIND_ARSENE => {
                        PLAY_SE(fighter,Hash40::new("vc_jack_arsene"));
                        WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES);
                    },
                    PERSONA_KIND_SATANAEL => PLAY_SE(fighter,Hash40::new("vc_jack_satanael")),
                    PERSONA_KIND_RAOUL => PLAY_SE(fighter,Hash40::new("vc_jack_raoul")),
                    PERSONA_KIND_HACHIMAKI => PLAY_SE(fighter,Hash40::new("vc_jack_hachimaki")),
                    PERSONA_KIND_MESSIAH => PLAY_SE(fighter,Hash40::new("vc_jack_messiah")),
                    PERSONA_KIND_FORTUNA => PLAY_SE(fighter,Hash40::new("vc_jack_fortuna")),
                    PERSONA_KIND_AME => PLAY_SE(fighter,Hash40::new("vc_jack_ame")),
                    PERSONA_KIND_SETANTA => PLAY_SE(fighter,Hash40::new("vc_jack_setanta")),
                    PERSONA_KIND_OKUNINUSHI => PLAY_SE(fighter,Hash40::new("vc_jack_okuninushi")),
                    PERSONA_KIND_KAGUYA => PLAY_SE(fighter,Hash40::new("vc_jack_kaguya")),
                    PERSONA_KIND_ORPHEOUS => PLAY_SE(fighter,Hash40::new("vc_jack_orpheous")),
                    PERSONA_KIND_YOSHITSUNE => PLAY_SE(fighter,Hash40::new("vc_jack_yoshitsune")),
                    _ => {
                        PLAY_SE(fighter,Hash40::new("vc_jack_arsene"));
                        WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES);
                    },
                };
                PLAY_SE_REMAIN(fighter,Hash40::new("se_jack_doyle_entry02"));
                WorkModule::off_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA);
            }
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let kind = WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            match kind {
                PERSONA_KIND_HACHIMAKI => {
                    if AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT)
                    || AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_SHIELD) {
                        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER.into(),false.into());
                    }
                },
                PERSONA_KIND_SETANTA | PERSONA_KIND_KAGUYA | 
                PERSONA_KIND_ORPHEOUS | PERSONA_KIND_CENDRILLON | 
                PERSONA_KIND_ELLA | PERSONA_KIND_VANADIS => {
                    if StopModule::is_damage(module_accessor) {
                        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER.into(),false.into());
                    }
                },
                PERSONA_KIND_YOSHITSUNE => {
                    if StopModule::is_damage(module_accessor) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                },
                _ => {},
            };
        }
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_JACK_DOYLE)]
pub fn arsene(weapon: &mut L2CFighterBase) {
    unsafe {
        let weapon_module_accessor = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_INVISIBLE) {
            ArticleModule::set_visibility_whole(owner_module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE,false,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else {
            ArticleModule::set_visibility_whole(owner_module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE,true,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if WorkModule::is_flag(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_GAUGE) {
            let current = WorkModule::get_float(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
            let mut add = current / 2.0;
            if add < 800.0 {
                add = 800.0;
            }
            if add + current > 1800.0 {
                add = 1800.0 - current;
            }
            WorkModule::add_float(weapon_module_accessor,add,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
            WorkModule::off_flag(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_GAUGE);
        }
        //manual mesh visibility control based on persona flag activated
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ARSENE {
            if WorkModule::is_flag(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_neck"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_wing"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_bust"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand00_l"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand00_r"),true);
                WorkModule::off_flag(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_ARSENE {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("neck"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_neck"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("neck_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_neck_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("bust"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_bust"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("bust_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_bust_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("wing"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_wing"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("wing_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("light_wing_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand00_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand00_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand01_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand01_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand02_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand02_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand03_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand03_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand04_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand04_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand05_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand05_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand06_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand06_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand07_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand07_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand08_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand08_r"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand09_l"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("doyle_hand09_r"),false);
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_SATANAEL {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_light_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_light_main_low"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SATANAEL {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_light_main"),true);
            }
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("satanael_light_main_low"),false);
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_RAOUL {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_RAOUL {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("raoul_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_HACHIMAKI {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_light_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_light_main_low"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_HACHIMAKI {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_light_main"),true);
            }
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_main_low"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("hachimaki_light_main_low"),false);
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_MESSIAH {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_MESSIAH {
            ModelModule::set_scale(weapon_module_accessor,0.95);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("messiah_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_FORTUNA {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_FORTUNA {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("fortuna_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_AME {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_AME {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ame_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_SETANTA {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SETANTA {
            ModelModule::set_scale(weapon_module_accessor,0.9);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("setanta_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_OKUNINUSHI {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_OKUNINUSHI {
            ModelModule::set_scale(weapon_module_accessor,0.9);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ookuninushi_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_KAGUYA {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_light_main_low"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_KAGUYA {
            ModelModule::set_scale(weapon_module_accessor,0.9);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_light_main_low"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("kaguya_light_main_low"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_ORPHEOUS {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ORPHEOUS {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("orpheus_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_YOSHITSUNE {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_YOSHITSUNE {
            ModelModule::set_scale(weapon_module_accessor,1.0);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("yoshitsune_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_CENDRILLON {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_CENDRILLON {
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("cendrillon_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_VANADIS {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_VANADIS {
            ModelModule::set_scale(weapon_module_accessor,0.9);
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("vanadis_light_main"),true);
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) != PERSONA_KIND_ELLA {
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_main"),false);
            ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_light_main"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ELLA {
            if WorkModule::is_flag(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLAG_SOLID) {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_main"),true);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_light_main"),false);
            }
            else {
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_main"),false);
                ModelModule::set_mesh_visibility(weapon_module_accessor,Hash40::new("ella_light_main"),true);
            }
        }
    }
}