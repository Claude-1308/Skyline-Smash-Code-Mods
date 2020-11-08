use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::acmd;
use smash::hash40;
use smash::app::lua_bind::*;

static mut GLOBAL_BULLET_TIME : bool = false;
static mut BULLET_TIME : [bool; 8] = [false; 8];
static mut STAMINA : [f32; 8] = [210.0; 8];
static mut STAMINA_EXHAUSTED : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;

pub fn link_bullet_time(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
            GLOBAL_BULLET_TIME = false;
        }
        if fighter_kind == *FIGHTER_KIND_LINK {
            let mut caller_id : usize = 0;
            if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && ControlModule::get_stick_x(module_accessor) == 0.0 && ControlModule::get_stick_y(module_accessor) == 0.0
            && BULLET_TIME[ENTRY_ID] == false && STAMINA_EXHAUSTED[ENTRY_ID] == false && GLOBAL_BULLET_TIME == false {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                    acmd!(lua_state, {
                        sv_animcmd::SLOW_OPPONENT(20, 210)
                    });
                    BULLET_TIME[ENTRY_ID] = true;
                    GLOBAL_BULLET_TIME = true;
                    caller_id = ENTRY_ID;
                }
            }
            if ControlModule::get_stick_y(module_accessor) == 0.0 && STAMINA[ENTRY_ID] > 0.0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && BULLET_TIME[ENTRY_ID]
            && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP) == false {
                let fall_speed  = smash::phx::Vector3f { x: 0.4, y: 0.4, z: 0.4 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK)
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_hi")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_blast")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_s1")
            || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP)
            || CaptureModule::is_capture(module_accessor)
            || STAMINA[ENTRY_ID] == 0.0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && caller_id == ENTRY_ID {
                acmd!(lua_state, {
                    sv_animcmd::SLOW_OPPONENT(0, 0)
                });
                let fall_speed  = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                BULLET_TIME[ENTRY_ID] = false;
                GLOBAL_BULLET_TIME = false;
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
            }
            if BULLET_TIME[ENTRY_ID] == true {
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
                STAMINA[ENTRY_ID] -= 1.0;
                acmd!(lua_state, {
                    FT_SET_FINAL_FEAR_FACE(1)
                });
                if STAMINA[ENTRY_ID] % 21.0 == 0.0 {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_dragoon_bg_lightning"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.35, true)
                    });
                }
                if STAMINA[ENTRY_ID] <= 60.0 {
                    if STAMINA[ENTRY_ID] % 4.0 == 0.0 {
                        acmd!(lua_state, {
                            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_timer"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.35, true)
                        });
                    }
                }
            }
            if BULLET_TIME[ENTRY_ID] == false && STAMINA[ENTRY_ID] == 0.0 {
                STAMINA_EXHAUSTED[ENTRY_ID] = true;
            }
            if STAMINA_EXHAUSTED[ENTRY_ID] == false {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    if STAMINA[ENTRY_ID] < 210.0 {
                        STAMINA[ENTRY_ID] += 1.0;
                        if STAMINA[ENTRY_ID] % 10.0 == 0.0 {
                            acmd!(lua_state, {
                                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_recovery"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
                            });
                        }
                    }
                }
                else {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
                if STAMINA[ENTRY_ID] == 210.0 {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
            }
            else {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    if STAMINA[ENTRY_ID] < 210.0 {
                        STAMINA[ENTRY_ID] += 0.5;
                        if STAMINA[ENTRY_ID] % 20.0 == 0.0 {
                            acmd!(lua_state, {
                                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_recovery"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
                            });
                        }
                    }
                }
                else {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
                if STAMINA[ENTRY_ID] == 210.0 {
                    STAMINA_EXHAUSTED[ENTRY_ID] = false;
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
            }
            if StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_REBIRTH {
                STAMINA[ENTRY_ID] = 210.0;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
                BULLET_TIME[ENTRY_ID] = false;
                GLOBAL_BULLET_TIME = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                GLOBAL_BULLET_TIME = false;
                STAMINA[ENTRY_ID] = 210.0;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
                BULLET_TIME[ENTRY_ID] = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
                acmd!(lua_state, {
                    sv_animcmd::SLOW_OPPONENT(30, 100)
                });
                STAMINA[ENTRY_ID] = 210.0;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(link_bullet_time);
}
