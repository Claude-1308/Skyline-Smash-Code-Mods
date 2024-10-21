use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use skyline::hooks::InlineCtx;
use std::arch::asm;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use crate::FIGHTER_MANAGER;

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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FighterInfo {
    unk_ptr1: *const (),
    unk_ptr2: *const (),
    unk1: [u8; 0x20],
    unk2: [u8; 0x20],
    unk3: [u8; 0x8],
    fighter_id: u32,
    unk4: [u8; 0x8],
    fighter_slot: u32,
}

pub fn acquire_fighter_info(index: usize) -> FighterInfo {
	unsafe {
		let global_thingy = (skyline::hooks::getRegionAddress(Region::Text) as usize + 0x5322680) as *const *const usize;
		let player_info = *((*global_thingy).byte_add(index * 8 + 0xE8)) as *const FighterInfo;
		*player_info
	}
}

/*#[skyline::hook(offset = 0x2140d90, inline)]
unsafe fn prepare_movie(ctx: &mut InlineCtx) {
	let _fighter_manager = *ctx.registers[0].x.as_ref() as *const ();
	let fighter = *ctx.registers[19].x.as_ref() as *mut L2CFighterCommon;
	let hash = ctx.registers[1].x.as_mut();
	if *hash == hash40("prebuilt:/movie/fighter/jack/c00/final_00.h264") {
		let player_slot = WorkModule::get_int((*fighter).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		*hash = hash40(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_00.h264", player_slot));
	} else if *hash == hash40("prebuilt:/movie/fighter/jack/c00/final_01.h264") {
		let player_slot = WorkModule::get_int((*fighter).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		*hash = hash40(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_01.h264", player_slot));
	}
}*/

#[skyline::hook(offset = 0xb2fadc, inline)]
unsafe fn handle_joker_00(ctx: &mut InlineCtx) {
	let this_arg = *ctx.registers[19].x.as_ref() as *const ();
	let player_index = *(this_arg.byte_add(0x10) as *const i32);
	let player_slot = acquire_fighter_info(player_index as usize).fighter_slot;
	*ctx.registers[1].x.as_mut() = hash40(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_00.h264", player_slot));
}

#[skyline::hook(offset = 0xb2fba0, inline)]
unsafe fn handle_joker_01(ctx: &mut InlineCtx) {
	let this_arg = *ctx.registers[19].x.as_ref() as *const ();
	let player_index = *(this_arg.byte_add(0x10) as *const i32);
	let player_slot = acquire_fighter_info(player_index as usize).fighter_slot;
	*ctx.registers[1].x.as_mut() = hash40(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_01.h264", player_slot));
}

#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_FINAL_READY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn final_ready_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_JACK_STATUS_FINAL_FLAG_CLEAR_SLOW);
    AreaModule::set_whole(fighter.module_accessor,false);
    GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    ItemModule::set_have_item_visibility(fighter.module_accessor,false,0);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    MotionModule::change_motion(fighter.module_accessor,Hash40::new("final_hit"),0.0,1.0,false,0.0,false,false);
    WorkModule::set_int(fighter.module_accessor,MotionModule::end_frame(fighter.module_accessor) as i32,*FIGHTER_JACK_STATUS_FINAL_INT_DASH_FRAME);
    let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
    FighterSpecializer_Jack::call_final_module(fighter_ptr,*FIGHTER_JACK_FINAL_MODULE_READY_INIT);
    let color_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_FIXED_MOVIE) == false {
        let rnd_gen = sv_math::rand(hash40("fighter"),100);
        let rnd_num = WorkModule::get_param_int(fighter.module_accessor,hash40("param_final"),hash40("movie_01_prob"));
        if rnd_num < rnd_gen {
            smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_00.h264", color_id)));
        }
        else {
            smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_01.h264", color_id)));
        }
    }
    else {
        smash::app::lua_bind::FighterManager::prepare_movie(fighter_manager,Hash40::new(&format!("prebuilt:/movie/fighter/jack/c{:02}/final_01.h264", color_id)));
    }
    WorkModule::set_int(fighter.module_accessor,*FIGHTER_MOVIE_STATUS_KIND_PREPARE,*FIGHTER_JACK_STATUS_FINAL_INT_MOVIE_STATUS);
    fighter.sub_shift_status_main(L2CValue::Ptr(final_ready_main_loop as *const () as _))
}

unsafe fn final_ready_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        if smash::app::lua_bind::FighterManager::is_prepared_movie(fighter_manager) {
            fighter.change_status(FIGHTER_JACK_STATUS_KIND_FINAL_SCENE01.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
    }
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
        fire2_fly_main,
        //final_ready_main
    );
    install_agent_frames!(
        joker,
        arsene,
    );
    skyline::install_hooks!(
        play_se_jack_replace
        //handle_joker_00,
        //handle_joker_01
    );
    for i in 0..50 {
        let hash_word: String = "fighter/jack/model/body/c0".to_owned() + &i.to_string() + "/kasumi.nutexb";
        my_callback::install(arcropolis_api::Hash40(hash40(&hash_word)),0x989680);
    }
}