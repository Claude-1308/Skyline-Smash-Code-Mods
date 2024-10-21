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

pub unsafe extern "C" fn timer(weapon: &mut L2CFighterBase, unk: L2CValue) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if unk.get_bool() {
        WorkModule::dec_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "jack_fire", status = WEAPON_JACK_FIRE_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fire_fly_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let angle = WorkModule::get_param_float(module_accessor,hash40("param_fire"),hash40("angle"));
    let mut speed = WorkModule::get_param_float(module_accessor,hash40("param_fire"),hash40("speed"));
    let owner_module_accessor = sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_CENDRILLON
    || WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_VANADIS
    || WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_ELLA
    || WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_KAGUYA {
        speed = 4.8
    }
    let corrected_speed_x = speed * PostureModule::lr(owner_module_accessor) * angle.to_radians().cos();
    let corrected_speed_y = speed * angle.to_radians().sin();
    let mut l2c_agent = L2CAgent::new(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(corrected_speed_x));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(corrected_speed_y));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(lua_state);
    let rot = Vector3f{x: angle.to_radians(), y: 0.0, z: 0.0};
    PostureModule::set_rot(module_accessor,&rot,0);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(timer as *const () as _));
    MotionModule::change_motion(module_accessor,Hash40::new("fly"),0.0,1.0,false,0.0,false,false);
    weapon.fastshift(L2CValue::Ptr(fire_fly_main_loop as *const () as _))
}

pub unsafe extern "C" fn fire_fly_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) > 0 {
        if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_RIGHT as u32 | *GROUND_TOUCH_FLAG_LEFT as u32
        | *GROUND_TOUCH_FLAG_DOWN as u32 | *GROUND_TOUCH_FLAG_DOWN_LEFT as u32 | *GROUND_TOUCH_FLAG_DOWN_RIGHT as u32) {
            MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_EFFECT,Hash40::new_raw(0x10d7892b94u64),-1);
            MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_SOUND,Hash40::new_raw(0xff2148eccu64),-1);
            acmd!(lua_state, {
                sv_battle_object::notify_event_msc_cmd(0x199c462b5du64)
            });
        }
    }
    else {
        MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_EFFECT,Hash40::new_raw(0xadad65bbeu64),-1);
        acmd!(lua_state, {
            sv_battle_object::notify_event_msc_cmd(0x199c462b5du64)
        });
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "jack_fire2", status = WEAPON_JACK_FIRE_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn fire2_fly_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut angle = WorkModule::get_param_float(module_accessor,hash40("param_fire2"),hash40("angle"));
    let mut speed = WorkModule::get_param_float(module_accessor,hash40("param_fire2"),hash40("speed"));
    let owner_module_accessor = sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_FORTUNA {
        angle = -11.0
    }
    if WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_SETANTA
    || WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_MESSIAH {
        angle = -5.0;
    }
    if WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_FORTUNA {
        speed = 6.0;
    }
    let mut life = WorkModule::get_param_int(module_accessor,hash40("param_fire2"),hash40("life"));
    if WorkModule::get_int(owner_owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) == PERSONA_KIND_FORTUNA {
        life = 14;
    }
    WorkModule::set_int(module_accessor,life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(module_accessor,life,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    let corrected_speed_x = speed * PostureModule::lr(owner_owner_module_accessor) * angle.to_radians().cos();
    let corrected_speed_y = speed * angle.to_radians().sin();
    let mut l2c_agent = L2CAgent::new(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(corrected_speed_x));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(corrected_speed_y));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(lua_state);
    let rot = Vector3f{x: angle.to_radians(), y: 0.0, z: 0.0};
    PostureModule::set_rot(module_accessor,&rot,0);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(timer as *const () as _));
    MotionModule::change_motion(module_accessor,Hash40::new("fly"),0.0,1.0,false,0.0,false,false);
    weapon.fastshift(L2CValue::Ptr(fire2_fly_main_loop as *const () as _))
}

pub unsafe extern "C" fn fire2_fly_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE) > 0 {
        if GroundModule::is_touch(module_accessor,*GROUND_TOUCH_FLAG_RIGHT as u32 | *GROUND_TOUCH_FLAG_LEFT as u32
        | *GROUND_TOUCH_FLAG_DOWN as u32 | *GROUND_TOUCH_FLAG_DOWN_LEFT as u32 | *GROUND_TOUCH_FLAG_DOWN_RIGHT as u32) {
            weapon.change_status(WEAPON_JACK_FIRE_STATUS_KIND_HIT.into(),false.into());
        }
    }
    else {
        MotionAnimcmdModule::call_script_single(module_accessor,*WEAPON_ANIMCMD_EFFECT,Hash40::new_raw(0xadad65bbeu64),-1);
        acmd!(lua_state, {
            sv_battle_object::notify_event_msc_cmd(0x199c462b5du64)
        });
    }
    return L2CValue::I32(0)
}