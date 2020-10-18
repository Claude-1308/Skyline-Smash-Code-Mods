use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use smash::app::lua_bind::*;
use acmd::acmd;

static mut DOYLE_FRAMES : [f32; 8] = [3000.0; 8];
static mut DOYLE_FLAG : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;
static mut GAUGE_FRAMES : [i32; 8] = [90; 8];
static mut GAUGE_FLAG : [bool; 8] = [false; 8];
static mut REBIRTH_FRAME : [i32; 8] = [1; 8];
static mut REBEL_GAUGE : [f32; 8] = [100.0; 8];
static mut SUSPEND_DOYLE : [bool; 8] = [false; 8];
static mut SUSPENSION_TRIGGER : [bool; 8] = [false; 8];

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if fighter_kind == *FIGHTER_KIND_JACK {
            WorkModule::set_flag(module_accessor,false,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
            if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_r") {
                CancelModule::enable_cancel(module_accessor);
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) && DOYLE_FRAMES[ENTRY_ID] > 0.0
            && GAUGE_FLAG[ENTRY_ID] == false && WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false {
                WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),100.0);
                DOYLE_FLAG[ENTRY_ID] = true;
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_LW) 
            && WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) && DOYLE_FLAG[ENTRY_ID] {
                SUSPEND_DOYLE[ENTRY_ID] = true;
                DOYLE_FLAG[ENTRY_ID] = false;
            }
            if DOYLE_FLAG[ENTRY_ID] {
                DOYLE_FRAMES[ENTRY_ID] -= 1.0;
                if DOYLE_FRAMES[ENTRY_ID] % 3.0 == 0.0 {
                    REBEL_GAUGE[ENTRY_ID] -= 0.1;
                }
                if DOYLE_FRAMES[ENTRY_ID] == 0.0 {
                    DOYLE_FLAG[ENTRY_ID] = false;
                    SUSPEND_DOYLE[ENTRY_ID] = true;
                }
            }
            if GAUGE_FLAG[ENTRY_ID] {
                if GAUGE_FRAMES[ENTRY_ID] >= 2 {
                    GAUGE_FRAMES[ENTRY_ID] -= 1;
                }
                else if GAUGE_FRAMES[ENTRY_ID] < 2 && GAUGE_FRAMES[ENTRY_ID] > 0 {
                    WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                    GAUGE_FRAMES[ENTRY_ID] -= 1;
                    smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),-100.0);
                }
                else {
                    WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                    GAUGE_FRAMES[ENTRY_ID] = 90;
                    GAUGE_FLAG[ENTRY_ID] = false;
                    smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),REBEL_GAUGE[ENTRY_ID]);
                }
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
                SUSPEND_DOYLE[ENTRY_ID] = true;
                DOYLE_FLAG[ENTRY_ID] = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),0.82);
                REBEL_GAUGE[ENTRY_ID] = 100.0;
                DOYLE_FRAMES[ENTRY_ID] = 3000.0;
                DOYLE_FLAG[ENTRY_ID] = false;
            }
            if StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_REBIRTH {
                WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                REBIRTH_FRAME[ENTRY_ID] -= 1;
                smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),-100.0);
            }
            if REBIRTH_FRAME[ENTRY_ID] == 0 {
                WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),99.9);
                REBEL_GAUGE[ENTRY_ID] = 100.0;
                DOYLE_FRAMES[ENTRY_ID] = 3000.0;
                DOYLE_FLAG[ENTRY_ID] = false;
                REBIRTH_FRAME[ENTRY_ID] = 1;
            }
        }
    }
}

pub fn once_per_weapon_frame(weapon: &mut L2CFighterBase) {
    unsafe{
        let weapon_module_accessor = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let weapon_kind = smash::app::utility::get_kind(weapon_module_accessor);
        ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if weapon_kind == *WEAPON_KIND_JACK_DOYLE {
            if DOYLE_FLAG[ENTRY_ID] {
                let current_doyle_life = WorkModule::get_float(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                if (DOYLE_FRAMES[ENTRY_ID] * 0.6) != current_doyle_life {
                    WorkModule::add_float(weapon_module_accessor,(DOYLE_FRAMES[ENTRY_ID] * 0.6) - current_doyle_life,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                }
                WorkModule::add_float(weapon_module_accessor,-0.6,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
            }
            if SUSPENSION_TRIGGER[ENTRY_ID] {
                WorkModule::add_float(weapon_module_accessor,-1.0,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                SUSPENSION_TRIGGER[ENTRY_ID] = false;
                SUSPEND_DOYLE[ENTRY_ID] = false;
                GAUGE_FLAG[ENTRY_ID] = true;
            }
            if SUSPEND_DOYLE[ENTRY_ID] {
                WorkModule::set_float(weapon_module_accessor,1.0,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                SUSPENSION_TRIGGER[ENTRY_ID] = true;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}
