use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

mod api;
use api::*;
mod modules;
use modules::*;
mod jabs;
use jabs::*;
mod tilts;
use tilts::*;
mod aerials;
use aerials::*;
mod smashes;
use smashes::*;
mod specials_acmd;
use specials_acmd::*;
mod specials_status;
use specials_status::*;
mod frame;
use frame::*;
mod speciald_acmd;
use speciald_acmd::*;

pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA : i32 = 0x200000ed;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES : i32 = 0x200000ee;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DISMISS_PERSONA : i32 = 0x200000ef;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_GAUGE : i32 = 0x200000f0;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_SETUP_PERSONAS : i32 = 0x200000f1;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND : i32 = 0x100000c2;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND : i32 = 0x100000c3;
pub const PERSONA_KIND_ARSENE : i32 = 9;
pub const PERSONA_KIND_SATANAEL : i32 = 10;
pub const PERSONA_KIND_RAOUL : i32 = 11;
pub const PERSONA_KIND_HACHIMAKI : i32 = 0;
pub const PERSONA_KIND_MESSIAH : i32 = 1;
pub const PERSONA_KIND_FORTUNA : i32 = 2;
pub const PERSONA_KIND_AME : i32 = 3;
pub const PERSONA_KIND_SETANTA : i32 = 4;
pub const PERSONA_KIND_OKUNINUSHI : i32 = 5;
pub const PERSONA_KIND_KAGUYA : i32 = 6;
pub const PERSONA_KIND_ORPHEOUS : i32 = 7;
pub const PERSONA_KIND_YOSHITSUNE : i32 = 8;
pub const PERSONA_KIND_CENDRILLON : i32 = 12;
pub const PERSONA_KIND_VANADIS : i32 = 13;
pub const PERSONA_KIND_ELLA : i32 = 14;

pub const SUB_STATUS : i32 = 0x15;
pub const SITUATION_KIND : i32 = 0x16;

pub static mut SUMI : Vec<i32> = Vec::new();

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if WorkModule::is_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_SETUP_PERSONAS) == false {
        if JackModule::sumi_id(module_accessor) {
            if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
                WorkModule::set_int(module_accessor,PERSONA_KIND_VANADIS,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
            else if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
                WorkModule::set_int(module_accessor,PERSONA_KIND_ELLA,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
            else {
                WorkModule::set_int(module_accessor,PERSONA_KIND_CENDRILLON,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
        }
        else {
            if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
                WorkModule::set_int(module_accessor,PERSONA_KIND_SATANAEL,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                WorkModule::set_int(module_accessor,PERSONA_KIND_SATANAEL,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
            else if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
                WorkModule::set_int(module_accessor,PERSONA_KIND_RAOUL,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                WorkModule::set_int(module_accessor,PERSONA_KIND_RAOUL,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
            else {
                WorkModule::set_int(module_accessor,PERSONA_KIND_ARSENE,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PRESET_PERSONA_KIND);
                WorkModule::set_int(module_accessor,PERSONA_KIND_ARSENE,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND);
            }
        }
        WorkModule::on_flag(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_SETUP_PERSONAS);
    }
    fighter.status_pre_Wait()
}

extern "C" {
    #[link_name = "\u{1}_ZN3app27WeaponSpecializer_JackDoyle12play_se_jackERNS_6WeaponEN3phx6Hash40E"]
    pub fn play_se_jack(weapon: *mut smash::app::Weapon, sound: Hash40) -> u64;
}

#[skyline::hook(replace=play_se_jack)]
unsafe fn play_se_jack_replace(weapon: *mut smash::app::Weapon, mut sound: Hash40) -> u64 {
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
    if sound.hash == hash40("vc_jack_doyle_call01")
    || sound.hash == hash40("vc_jack_doyle_call02") {
        sound.hash = match WorkModule::get_int(owner_boma,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) { //added vc entries for persona name calls
            PERSONA_KIND_ARSENE => hash40("vc_jack_arsene"),
            PERSONA_KIND_ELLA => hash40("vc_jack_ella"),
            PERSONA_KIND_SATANAEL => hash40("vc_jack_satanael"),
            PERSONA_KIND_RAOUL => hash40("vc_jack_raoul"),
            PERSONA_KIND_HACHIMAKI => hash40("vc_jack_arsene"),
            PERSONA_KIND_MESSIAH => hash40("vc_jack_messiah"),
            PERSONA_KIND_CENDRILLON => hash40("vc_jack_cendrillon"),
            PERSONA_KIND_FORTUNA => hash40("vc_jack_fortuna"),
            PERSONA_KIND_AME => hash40("vc_jack_ame"),
            PERSONA_KIND_SETANTA => hash40("vc_jack_setanta"),
            PERSONA_KIND_OKUNINUSHI => hash40("vc_jack_okuninushi"),
            PERSONA_KIND_VANADIS => hash40("vc_jack_vanadis"),
            PERSONA_KIND_KAGUYA => hash40("vc_jack_kaguya"),
            PERSONA_KIND_ORPHEOUS => hash40("vc_jack_orpheous"),
            PERSONA_KIND_YOSHITSUNE => hash40("vc_jack_yoshitsune"),
            _ => hash40("vc_jack_cendrillon"),
        };
    }
    original!()(weapon,sound)
}

pub fn install() {
    install_acmd_scripts!(
        attack_11,
        doyle_effect_11,
        attack_12,
        doyle_effect_12,
        attack_13,
        doyle_effect_13,
        attack_s3_hi,
        attack_s3_s,
        attack_s3_lw,
        effect_s3,
        attack_hi3,
        attack_s4_s,
        attack_hi4,
        attack_air_n,
        attack_air_b,
        special_s1,
        special_s2,
        effect_special_s1,
        effect_air_special_s1,
        effect_special_s2,
        effect_air_special_s2,
        doyle_special_s2,
        special_hi_start,
        fire_fly,
        fire_hit,
        effect_fire_end,
        effect_fire_hitground,
        effect_fire_fly,
        effect_fire_hit,
        sound_fire_fly,
        sound_fire_hit,
        sound_fire_hitground,
        fire2_fly,
        fire2_hit,
        effect_fire2_end,
        effect_fire2_fly,
        effect_fire2_hit,
        sound_fire2_fly,
        sound_fire2_hit,
        special_lw,
        special_lw_counter,
        effect_lw,
        effect_lw_counter,
        sound_lw,
        sound_lw_counter
    );
    install_status_scripts!(
        wait_pre,
        fire_fly_main,
        fire2_fly_main
    );
    install_agent_frames!(
        joker,
        arsene,
    );
    skyline::install_hook!(play_se_jack_replace);
    unsafe {
        let slots = JackModule::get_params("param_private","tot_slots") as i32;
        for i in 0..slots {
            let hash_word: String = "fighter/jack/model/body/c0".to_owned() + &i.to_string() + "/kasumi.nutexb";
            my_callback::install(arcropolis_api::Hash40(hash40(&hash_word)),0x989680);
        }
    }
}