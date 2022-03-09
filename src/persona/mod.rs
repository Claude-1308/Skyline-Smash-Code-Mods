use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

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

pub unsafe fn sumi_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool { //to determine if it is violet or joker
    return match WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
        1 | 7 => true,
        _ => false,
    }
}

pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_CHANGE_PERSONA : i32 = 0x200000ed;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESET_MESHES : i32 = 0x200000ee;
pub const FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_PRESET_CHOSEN : i32 = 0x200000ef;
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

pub fn decide_box_property(persona_kind: i32) -> (u64,i32) {
    match persona_kind {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
            return (hash40("collision_attr_curse_poison"),*COLLISION_SOUND_ATTR_FIRE);
        },
        PERSONA_KIND_HACHIMAKI => {
            return (hash40("collision_attr_elec"),*COLLISION_SOUND_ATTR_ELEC);
        },
        PERSONA_KIND_AME => {
            return (hash40("collision_attr_ice"),*COLLISION_SOUND_ATTR_FREEZE);
        },
        PERSONA_KIND_SETANTA => {
            return (hash40("collision_attr_aura"),*COLLISION_SOUND_ATTR_FIRE);
        },
        PERSONA_KIND_OKUNINUSHI => {
            return (hash40("collision_attr_purple"),*COLLISION_SOUND_ATTR_FIRE);
        },
        PERSONA_KIND_ORPHEOUS => {
            return (hash40("collision_attr_fire"),*COLLISION_SOUND_ATTR_FIRE);
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            return (hash40("collision_attr_magic"),*COLLISION_SOUND_ATTR_MAGIC);
        },
        _ => {
            return (hash40("collision_attr_cutup"),*COLLISION_SOUND_ATTR_CUTUP);
        },
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app27WeaponSpecializer_JackDoyle12play_se_jackERNS_6WeaponEN3phx6Hash40E"]
    fn play_se_jack(weapon: *mut smash::app::Weapon, sound: Hash40) -> u64;
}

#[skyline::hook(replace=play_se_jack)]
unsafe fn play_se_jack_replace(weapon: *mut smash::app::Weapon, mut sound: Hash40) -> u64 {
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
    if sound.hash == hash40("vc_jack_doyle_call01")
    || sound.hash == hash40("vc_jack_doyle_call02") {
        if WorkModule::is_flag(owner_boma,FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_PRESET_CHOSEN) == false {
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
        else {
            sound.hash = match WorkModule::get_int(owner_boma,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) { //added vc entries for persona name calls
                PERSONA_KIND_ARSENE => hash40("vc_jack_arsene"),
                PERSONA_KIND_ELLA => hash40("vc_jack_ella"),
                PERSONA_KIND_SATANAEL => hash40("vc_jack_satanael"),
                PERSONA_KIND_RAOUL => hash40("vc_jack_raoul"),
                PERSONA_KIND_HACHIMAKI => hash40("vc_jack_hachimaki"),
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
    }
    original!()(weapon,sound)
}

#[skyline::hook(replace = MotionModule::change_motion)] //change motion from appeal to summon
pub unsafe fn change_motion_replace(module_accessor: &mut BattleObjectModuleAccessor, mut motion_kind: Hash40, frame: f32, rate: f32, unk1: bool, unk2: f32, unk3: bool, unk4: bool) -> u64 {
    let kind = utility::get_kind(module_accessor);
    if kind == *FIGHTER_KIND_JACK && WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)
    && sumi_id(module_accessor) == false {
        if motion_kind.hash == hash40("appeal_hi_r") || motion_kind.hash == hash40("appeal_hi_l")
        || motion_kind.hash == hash40("appeal_lw_l") || motion_kind.hash == hash40("appeal_lw_r")
        || motion_kind.hash == hash40("appeal_s_l") || motion_kind.hash == hash40("appeal_s_r") {
            motion_kind.hash = hash40("summon");
        }
    }
    original!()(module_accessor,motion_kind,frame,rate,unk1,unk2,unk3,unk4)
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
        sound_fire2_hit
    );
    install_status_scripts!(
        fire_fly_main,
        fire2_fly_main,
    );
    install_agent_frames!(
        joker,
        arsene,
    );
    skyline::install_hook!(change_motion_replace);
    skyline::install_hook!(play_se_jack_replace);
}