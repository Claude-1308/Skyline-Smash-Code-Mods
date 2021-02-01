use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use smash::app::lua_bind::*;
use acmd::{acmd,acmd_func};
use smash::hash40;
use crate::FIGHTER_MANAGER;
use skyline::nn::ro::LookupSymbol;

static mut DOYLE_FRAMES : [f32; 8] = [2000.0; 8];
static mut DOYLE_FLAG : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;
static mut GAUGE_FRAMES : [i32; 8] = [90; 8];
static mut GAUGE_FLAG : [bool; 8] = [false; 8];
static mut REBIRTH_FRAME : [i32; 8] = [1; 8];
static mut REBEL_GAUGE : [f32; 8] = [100.0; 8];
static mut SUSPEND_DOYLE : [bool; 8] = [false; 8];
static mut SUSPENSION_TRIGGER : [bool; 8] = [false; 8];
static mut CREATE : [bool; 8] = [false; 8];
static mut WAIT : [i32; 8] = [90; 8];
static mut ZERO_LIFE : [bool; 8] = [false; 8];
static mut SET_ZERO : [bool; 8] = [false; 8];
static mut ZERO : [bool; 8] = [false; 8];

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_lw_attack",
    animcmd = "game_speciallwattack")]
pub fn joker_lw_attack(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            LookupSymbol(
                &mut FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
            );
            let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
            let mut dmg = 0.0;
            let mut effect = smash::hash40("collision_attr_purple");
            if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false {
                dmg = 4.0;
                effect = smash::hash40("collision_attr_slip");
            }
            else {
                dmg = 2.4;
                effect = smash::hash40("collision_attr_purple");
            }
        }
        frame(Frame=1)
        if(is_excute) {
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=8)
        if(is_excute) {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=50, KBG=100, FKB=82, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=-3.0, X2=0.0, Y2=10.0, Z2=3.0, Hitlag=1.25, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=2)
        if(is_excute) {
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute) {
            WHOLE_HIT(HIT_STATUS_NORMAL)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_lw_attack",
    animcmd = "game_specialairlwattack")]
pub fn joker_air_lw_attack(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            LookupSymbol(
                &mut FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
            );
            let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
            let mut dmg = 0.0;
            let mut effect = smash::hash40("collision_attr_purple");
            if FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false {
                dmg = 6.0;
                effect = smash::hash40("collision_attr_purple");
            }
            else {
                dmg = 2.4;
                effect = smash::hash40("collision_attr_purple");
            }
        }
        frame(Frame=1)
        if(is_excute) {
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=8)
        if(is_excute) {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=50, KBG=100, FKB=82, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=-3.0, X2=0.0, Y2=10.0, Z2=3.0, Hitlag=1.25, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=2)
        if(is_excute) {
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute) {
            WHOLE_HIT(HIT_STATUS_NORMAL)
        }
    });
}

pub fn joker(fighter: &mut L2CFighterCommon) {
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
        if fighter_kind == *FIGHTER_KIND_JACK
        && FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32))) == false {
            WorkModule::set_flag(module_accessor,false,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
            if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_r") {
                CancelModule::enable_cancel(module_accessor);
            }
            if ArticleModule::is_exist(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE) == false && FighterManager::is_result_mode(fighter_manager) == false
            && FighterManager::is_result_mode(fighter_manager) == false && smash::app::smashball::is_training_mode() == false && REBEL_GAUGE[ENTRY_ID] > 0.0 {
                CREATE[ENTRY_ID] = true;
            }
            if CREATE[ENTRY_ID] {
                WAIT[ENTRY_ID] -= 1;
                if WAIT[ENTRY_ID] == 0 {
                    CREATE[ENTRY_ID] = false;
                    WAIT[ENTRY_ID] = 90;
                    SET_ZERO[ENTRY_ID] = true;
                    ArticleModule::generate_article(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE,false,0);
                }
            }
            if ControlModule::check_button_on_trriger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) && DOYLE_FLAG[ENTRY_ID] == false && REBEL_GAUGE[ENTRY_ID] > 0.0 {
                WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE);
            }
            if WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) && DOYLE_FLAG[ENTRY_ID] == false {
                if AttackModule::is_infliction_status(module_accessor,*COLLISION_KIND_MASK_HIT) {
                    WorkModule::set_flag(module_accessor,true,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE);
                    DOYLE_FRAMES[ENTRY_ID] -= 1.0;
                    REBEL_GAUGE[ENTRY_ID] -= 0.05;
                    smash::app::FighterSpecializer_Jack::add_rebel_gauge(module_accessor,smash::app::FighterEntryID(ENTRY_ID as i32),-0.1);
                }
            }
            let motion = MotionModule::motion_kind(module_accessor);
            if MotionModule::frame(module_accessor) as i32 == FighterMotionModuleImpl::get_cancel_frame(module_accessor,smash::phx::Hash40::new_raw(motion),false)
            && ZERO_LIFE[ENTRY_ID] && DOYLE_FLAG[ENTRY_ID] == false && REBEL_GAUGE[ENTRY_ID] > 0.0 {
                WorkModule::set_flag(module_accessor,false,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE);
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) && DOYLE_FRAMES[ENTRY_ID] > 0.0 && GAUGE_FLAG[ENTRY_ID] == false {
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
                if DOYLE_FRAMES[ENTRY_ID] % 1.0 == 0.0 {
                    REBEL_GAUGE[ENTRY_ID] -= 0.05;
                }
                if DOYLE_FRAMES[ENTRY_ID] <= 0.0 {
                    DOYLE_FLAG[ENTRY_ID] = false;
                    SUSPEND_DOYLE[ENTRY_ID] = true;
                    REBEL_GAUGE[ENTRY_ID] = 0.0;
                }
            }
            if REBEL_GAUGE[ENTRY_ID] <= 0.0 && ZERO[ENTRY_ID] == false {
                ArticleModule::remove_exist(module_accessor,*FIGHTER_JACK_GENERATE_ARTICLE_DOYLE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                ZERO[ENTRY_ID] = true;
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
                DOYLE_FRAMES[ENTRY_ID] = 2000.0;
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
                DOYLE_FRAMES[ENTRY_ID] = 2000.0;
                DOYLE_FLAG[ENTRY_ID] = false;
                REBIRTH_FRAME[ENTRY_ID] = 1;
            }
        }
    }
}

pub fn arsene(weapon: &mut L2CFighterBase) {
    unsafe {
        let weapon_module_accessor = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let weapon_kind = smash::app::utility::get_kind(weapon_module_accessor);
        ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if weapon_kind == *WEAPON_KIND_JACK_DOYLE {
            if DOYLE_FLAG[ENTRY_ID] {
                let current_doyle_life = WorkModule::get_float(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                if (DOYLE_FRAMES[ENTRY_ID] * 0.9) != current_doyle_life {
                    WorkModule::add_float(weapon_module_accessor,(DOYLE_FRAMES[ENTRY_ID] * 0.9) - current_doyle_life,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                }
                WorkModule::add_float(weapon_module_accessor,-0.9,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
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
            if WorkModule::get_float(weapon_module_accessor,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE) == 0.0 {
                ZERO_LIFE[ENTRY_ID] = true;
            }
            else {
                ZERO_LIFE[ENTRY_ID] = false;
            }
            if SET_ZERO[ENTRY_ID] {
                WorkModule::set_float(weapon_module_accessor,1.0,*WEAPON_JACK_DOYLE_INSTANCE_WORK_ID_FLOAT_LIFE);
                SET_ZERO[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_hooks!(joker_air_lw_attack,joker_lw_attack);
    acmd::add_custom_hooks!(joker);
    acmd::add_custom_weapon_hooks!(arsene);
}
