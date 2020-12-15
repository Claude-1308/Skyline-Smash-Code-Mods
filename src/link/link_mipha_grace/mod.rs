use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use acmd::acmd;
use smash::hash40;
use crate::FIGHTER_MANAGER;
use skyline::nn::ro::LookupSymbol;

static mut MIPHA_GRACE : [bool; 8] = [true; 8];
static mut MIPHA_INVINCIBILITY : [i32; 8] = [180; 8];
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
                    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
                    FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),true);
                }
            }
            if MIPHA_INVINCIBILITY[ENTRY_ID] > 0 && MIPHA_GRACE[ENTRY_ID] == false {
                MIPHA_INVINCIBILITY[ENTRY_ID] -= 1;
                DamageModule::set_damage_lock(module_accessor,true);
                DamageModule::set_reaction_mul(module_accessor,0.0);
                if MIPHA_INVINCIBILITY[ENTRY_ID] == 120 {
                    FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),false);
                    WorkModule::set_int(module_accessor,3,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                    HEAL[ENTRY_ID] = true;
                }
                if MIPHA_INVINCIBILITY[ENTRY_ID] == 0 {
                    WorkModule::set_int(module_accessor,2,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                    DamageModule::set_damage_lock(module_accessor,false);
                    DamageModule::set_reaction_mul(module_accessor,1.0);
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_fairybottle_navy"),true,false);
                }
                if MIPHA_INVINCIBILITY[ENTRY_ID] <= 120
                && MIPHA_INVINCIBILITY[ENTRY_ID] % 12 == 0 {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_fairybottle_navy"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*XRot*/ 10, /*YRot*/ 40, /*ZRot*/ 0, /*Size?*/ 0.6, true)
                    });
                }
            }
            if HEAL[ENTRY_ID] {
                let curr_damage = DamageModule::damage(module_accessor,0);
                let x = curr_damage/50.0;
                let heal_mul = (0.25 + 0.2*x) * -1.0;
                DamageModule::add_damage(module_accessor,curr_damage*heal_mul,0);
                HEAL[ENTRY_ID] = false;
                acmd!(lua_state, { PLAY_SE(hash40("se_common_swimdrown")) });
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                MIPHA_GRACE[ENTRY_ID] = true;
                MIPHA_INVINCIBILITY[ENTRY_ID] = 180;
                HEAL[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(mipha_grace);
}
