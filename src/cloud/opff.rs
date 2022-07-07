use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash_script::macros::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};

use crate::moveset::*;

#[smashline::fighter_frame(agent = FIGHTER_KIND_CLOUD)]
pub fn cloud(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        /*if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
            CloudModule::materia_change(module_accessor,MAGIC_TYPE_FIRE);
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
            CloudModule::materia_change(module_accessor,MAGIC_TYPE_ICE);
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
            CloudModule::materia_change(module_accessor,MAGIC_TYPE_ELEC);
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_LW) {
            CloudModule::materia_change(module_accessor,MAGIC_TYPE_WIND);
        }*/
        if StopModule::is_damage(module_accessor)
        && WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_DISABLE_SWORD_RESET) == false {
            CloudModule::reset_swords(module_accessor);
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI)
        && WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB) {
            WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
            WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
            let vec3 = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            EffectModule::req_follow(module_accessor,Hash40::new("cloud_limitbreak_aura"),Hash40::new("hip"),&vec3,&vec3,2.0,false,0x8000000,0,-1,0,0,false,false);
            EffectModule::req_common(module_accessor,Hash40::new("cloud_limitbreak"),0.0);
            EffectModule::req_follow(module_accessor,Hash40::new_raw(0x16102a334bu64),Hash40::new("top"),&vec3,&vec3,1.0,true,0,0,-1,0,0,false,false);
            PLAY_SE(fighter,Hash40::new("se_cloud_special_l03"));
            WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB);
        }
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_CLOUD_WAVE)]
pub fn cloud_wave(weapon: &mut L2CFighterBase) {
    unsafe {
        let lua_state = weapon.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if WorkModule::is_flag(module_accessor,WEAPON_CLOUD_WAVE_INSTANCE_WORK_ID_FLAG_FSMASH_BEAM) {
            if StatusModule::status_kind(module_accessor) == *WEAPON_CLOUD_WAVE_STATUS_KIND_HIT {
                acmd!(lua_state, { sv_battle_object::notify_event_msc_cmd(0x199c462b5du64) });
            }
            if AttackModule::is_attack(module_accessor,0,false) {
                AttackModule::set_power(module_accessor,0,15.0,false);
                AttackModule::set_power(module_accessor,0,15.0,false);
            }
        }
    }
}

#[skyline::hook(offset=0x8dc140)]
pub unsafe fn limit_manager(amount: f32, module_accessor: *mut BattleObjectModuleAccessor, unk: u32) {
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB as i32) == false {
        let gauge_val = WorkModule::get_float(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        if gauge_val + amount >= 100.0 {
            WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_CHECK_LB);
        }
        WorkModule::add_float(module_accessor,amount,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        if unk > 0 {
            WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE);
        }
    }
}