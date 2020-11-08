use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use acmd::acmd;
use crate::FIGHTER_MANAGER;
use skyline::nn::ro::LookupSymbol;

static mut MIPHA_GRACE : [bool; 8] = [true; 8];
static mut MIPHA_INVINCIBILITY : [i32; 8] = [90; 8];
static mut HEAL : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;

pub fn mipha_grace(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        if fighter_kind == *FIGHTER_KIND_LINK {
            if WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DEATH_PREDICTION) {
                if MIPHA_GRACE[ENTRY_ID] {
                    MIPHA_GRACE[ENTRY_ID] = false;
                    acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0) });
                    DamageModule::set_damage_lock(module_accessor,true);
                    FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),true);
                }
            }
            if MIPHA_INVINCIBILITY[ENTRY_ID] > 0 && MIPHA_GRACE[ENTRY_ID] == false {
                MIPHA_INVINCIBILITY[ENTRY_ID] -= 1;
                if MIPHA_INVINCIBILITY[ENTRY_ID] == 0 {
                    acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0) });
                    DamageModule::set_damage_lock(module_accessor,false);
                    FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),false);
                    WorkModule::set_int(module_accessor,0,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                    HEAL[ENTRY_ID] = true;
                }
            }
            if HEAL[ENTRY_ID] {
                if DamageModule::damage(module_accessor,0) <= 50.0 {
                    DamageModule::add_damage(module_accessor,DamageModule::damage(module_accessor,0) * -0.25,0);
                }
                else if DamageModule::damage(module_accessor,0) > 50.0 {
                    DamageModule::add_damage(module_accessor,DamageModule::damage(module_accessor,0) * -0.5,0);
                }
                else if DamageModule::damage(module_accessor,0) > 100.0 {
                    DamageModule::add_damage(module_accessor,DamageModule::damage(module_accessor,0) * -0.75,0);
                }
                else if DamageModule::damage(module_accessor,0) > 150.0 {
                    DamageModule::add_damage(module_accessor,DamageModule::damage(module_accessor,0) * -0.9,0);
                }
                HEAL[ENTRY_ID] = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                MIPHA_GRACE[ENTRY_ID] = true;
                MIPHA_INVINCIBILITY[ENTRY_ID] = 90;
                HEAL[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(mipha_grace);
}
