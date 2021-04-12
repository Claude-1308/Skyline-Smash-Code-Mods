use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon};
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::*;
use smash_script::macros::*;
use smash_script::fighter_frame;
use crate::FIGHTER_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{getRegionAddress, Region};

//elemental arrow
static mut FIRE_ARROW : [bool; 8] = [false; 8];
static mut ICE_ARROW : [bool; 8] = [false; 8];
static mut SHOCK_ARROW : [bool; 8] = [false; 8];
static mut BOMB_ARROW : [bool; 8] = [false; 8];
//bullet time
static mut GLOBAL_BULLET_TIME : bool = false; //when active other links cannot use bullet time
static mut BULLET_TIME : [bool; 8] = [false; 8]; //bullet time flag
static mut STAMINA : [f32; 8] = [210.0; 8]; //timer till u can have slowdown
static mut STAMINA_EXHAUSTED : [bool; 8] = [false; 8]; //flag for exhaustion to not let usage before recharge
static mut BULLET_TIME_CALLER_ID : usize = 0; //user id for separation from other links
//mipha grace
static mut MIPHA_GRACE : [bool; 8] = [true; 8]; //one time flag for last stock heal
static mut MIPHA_INVINCIBILITY : [i32; 8] = [180; 8]; //timer for stall a bit before healing kicks in
static mut HEAL : [bool; 8] = [false; 8]; //flag for heal one time
//parry
static mut SET_FRAME : [bool; 8] = [false; 8]; //flag for parry motion setup
//stasis
static mut FRAME : [f32; 8] = [0.0; 8]; //frame at which enemy is locked
static mut MOTION : [u64; 8] = [0; 8]; //motion kind of enemy locked
static mut POS : [f32; 8] = [0.0; 8]; //posture of enemy locked
static mut STASIS_TIMER : [i32; 8] = [0; 8];
static mut SET_FREEZE : [bool; 8] = [false; 8]; //setup flag for stasis
static mut FREEZE : [bool; 8] = [false; 8]; //flag for operation during stasis
static mut STASIS_COOLDOWN : [bool; 8] = [false; 8]; //cooldown flag to not let link do it again
static mut STASIS_COOLDOWN_TIMER : [i32; 8] = [0; 8];
static mut GLOBAL_STASIS : bool = false; //flag setting to not let more than 1 link do it
static mut SET_UNFREEZE : bool = false; //setup flag for after stasis
static mut SOUND : [bool; 8] = [false; 8]; //flag for using stasis hit effect
static mut PRE_DAMAGE : [f32; 8] = [0.0; 8]; //pre stasis damage
static mut POST_DAMAGE : [f32; 8] = [0.0; 8]; //during stasis damage (yes name is misleading)
static mut START : [bool; 8] = [false; 8]; //flag for stasis start sound
static mut RESET_UNFREEZE_TIMER : i32 = 0; //stall for resetting stasis setup
static mut STASIS_CALLER_ID : usize = 8; //user id to differentiate from other links
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20; //hook offset
//common
static mut ENTRY_ID : usize = 0;

//elemental arrow
//hitbox setup based on user choice for arrow
#[script(agent = "link_bowarrow", script = "game_fly", category = ACMD_GAME)]
unsafe fn arrow(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if FIRE_ARROW[ENTRY_ID] == true {
        if is_excute(weapon) {
            EFFECT_FOLLOW(weapon,Hash40::new("sys_damage_fire"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.3,true);
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,75,0,15,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
    else if ICE_ARROW[ENTRY_ID] == true {
        if is_excute(weapon) {
            EFFECT_FOLLOW(weapon,Hash40::new("sys_freezer"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.3,true);
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,75,0,15,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
    else if SHOCK_ARROW[ENTRY_ID] == true {
        if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_hit_elec_s"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.25,true);
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,90,0,20,1.75,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_paralyze"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(module_accessor);
                AttackModule::set_power_mul(module_accessor,1.15);
                QUAKE(weapon,*CAMERA_QUAKE_KIND_S);
            }
        }
        else {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_hit_elec_s"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.25,true);
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,90,0,20,1.75,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(module_accessor);
            }
        }
    }
    else if BOMB_ARROW[ENTRY_ID] == true {
        if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_bomb_spark"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.25,true);
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,100,0,25,2.0,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_LL,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(module_accessor);
                AttackModule::set_power_mul(module_accessor,1.25);
                QUAKE(weapon,*CAMERA_QUAKE_KIND_L);
            }
        }
        else {
            if is_excute(weapon) {
                EFFECT_FOLLOW(weapon,Hash40::new("sys_bomb_spark"),Hash40::new("armr"),0.0,-0.5,6.5,0,0,0,0.25,true);
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,100,0,25,2.0,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(module_accessor);
                QUAKE(weapon,*CAMERA_QUAKE_KIND_S);
            }
        }
    }
    else {
        if is_excute(weapon) {
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.35,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_cutup"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_CUTUP,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
}

//parry
//hitbox for slipping enemy, usually wont work but when it does it will slip enemy, good rng type based move but i did not set the rng
#[script(agent = "link", script = "game_justshieldoff", category = ACMD_GAME)]
unsafe fn parry(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,6.5);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),1.0,361,80,70,0,10.0,0.0,10.0,5.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_slip"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state,10.5);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
}

//stasis box
//useful box to be setup stasis, collision from here is detected in the offset hook below
#[script(agent = "link", scripts = ["game_appealsr", "game_appealsl"], category = ACMD_GAME)]
unsafe fn stasis(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        ATTACK(fighter,0,0,Hash40::new("top"),0.0,361,80,70,0,30.0,0.0,15.0,15.0,Some(0.0),Some(30.0),Some(30.0),1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state,20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
}

#[fighter_frame(agent = FIGHTER_KIND_LINK)]
unsafe fn link(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    //elemental arrows
    //setup for arrows based on taunt button input during charge
    if WorkModule::is_flag(module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) == true {
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
            if FIRE_ARROW[ENTRY_ID] == false {
                FIRE_ARROW[ENTRY_ID] = true;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
            else {
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_LW) {
            if ICE_ARROW[ENTRY_ID] == false {
                ICE_ARROW[ENTRY_ID] = true;
                FIRE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
            else {
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
            if SHOCK_ARROW[ENTRY_ID] == false {
                SHOCK_ARROW[ENTRY_ID] = true;
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
            else {
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
        }
        if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
            if BOMB_ARROW[ENTRY_ID] == false {
                BOMB_ARROW[ENTRY_ID] = true;
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
            }
            else {
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
        } 
        if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == true { //effects to show arrow selected
            if FIRE_ARROW[ENTRY_ID] {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_damage_fire"),Hash40::new("arml"),6.0,2.0,0.0,0,0,0,0.3,true);
            }
            else if ICE_ARROW[ENTRY_ID] {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_freezer"),Hash40::new("arml"),6.0,1.0,0.0,0,0,0,0.15,true);
            }
            else if SHOCK_ARROW[ENTRY_ID] {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_damage_elec"),Hash40::new("arml"),6.0,1.0,0.0,0,0,0,0.3,true);
            }
            else if BOMB_ARROW[ENTRY_ID] {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_flame"),Hash40::new("arml"),6.0,1.0,0.0,0,0,0,0.15,true);
            }
        }
    }
    //bullet time
    //setup for starting bullet time
    if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == true && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
    && ControlModule::get_stick_x(module_accessor) == 0.0 && ControlModule::get_stick_y(module_accessor) == 0.0 && BULLET_TIME[ENTRY_ID] == false
    && STAMINA_EXHAUSTED[ENTRY_ID] == false && GLOBAL_BULLET_TIME == false {
        SLOW_OPPONENT(fighter,20.0,210.0);
        BULLET_TIME[ENTRY_ID] = true;
        GLOBAL_BULLET_TIME = true;
        BULLET_TIME_CALLER_ID = ENTRY_ID;
        //PLAY_SE(fighter,Hash40::new("se_link_bullet_time"));
    }
    if ControlModule::get_stick_y(module_accessor) == 0.0 && STAMINA[ENTRY_ID] > 0.0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && BULLET_TIME[ENTRY_ID] == true
    && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP) == false { //kinetic changes to support link's aerial motion
        let fall_speed  = Vector3f { x: 0.4, y: 0.4, z: 0.4 };
        KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    let cat1 = ControlModule::get_command_flag_cat(module_accessor,0);
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
    || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK) == true
    || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0
    || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0
    || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0
    || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP)
    || CaptureModule::is_capture(module_accessor) == true
    || STAMINA[ENTRY_ID] <= 0.0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && BULLET_TIME_CALLER_ID == ENTRY_ID { //any of the follwoing will cancel/end bullet time
        SLOW_OPPONENT(fighter,0.0,0.0);
        let fall_speed  = Vector3f { x: 1.0, y: 1.0, z: 1.0 };
        KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        BULLET_TIME[ENTRY_ID] = false;
        GLOBAL_BULLET_TIME = false;
        EffectModule::kill_kind(module_accessor,Hash40::new("sys_timer"),true,false);
    }
    if BULLET_TIME[ENTRY_ID] == true { //time setup
        STAMINA[ENTRY_ID] -= 1.0;
        FT_SET_FINAL_FEAR_FACE(fighter,1);
        if STAMINA[ENTRY_ID] % 21.0 == 0.0 {
            EFFECT_FOLLOW(fighter,Hash40::new("sys_dragoon_bg_lightning"),Hash40::new("top"),0.0,10.0,0.0,0,0,0,0.35,true);
        }
        if STAMINA[ENTRY_ID] <= 60.0 {
            if STAMINA[ENTRY_ID] % 4.0 == 0.0 {
                EFFECT_FOLLOW(fighter,Hash40::new("sys_timer"),Hash40::new("top"),0.0,10.0,0.0,0,0,0,0.35,true);
            }
        }
    }
    else if STAMINA[ENTRY_ID] <= 0.0 {
        STAMINA_EXHAUSTED[ENTRY_ID] = true;
    }
    if STAMINA_EXHAUSTED[ENTRY_ID] == false { //bullet time recharge based on exhaustion flag
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            if STAMINA[ENTRY_ID] < 210.0 {
                STAMINA[ENTRY_ID] += 1.0;
                if STAMINA[ENTRY_ID] % 10.0 == 0.0 {
                    EFFECT_FOLLOW(fighter,Hash40::new("sys_recovery"),Hash40::new("top"),0.0,0.0,0.0,0,0,0,0.65,true);
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
                    EFFECT_FOLLOW(fighter,Hash40::new("sys_recovery"),Hash40::new("top"),0.0,0.0,0.0,0,0,0,0.65,true);
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
    //mipha grace
    if WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DEATH_PREDICTION) { //called during red screen
        if MIPHA_GRACE[ENTRY_ID] {
            MIPHA_GRACE[ENTRY_ID] = false;
            KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
            lua_bind::FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),true);
        }
    }
    if MIPHA_INVINCIBILITY[ENTRY_ID] > 0 && MIPHA_GRACE[ENTRY_ID] == false {
        MIPHA_INVINCIBILITY[ENTRY_ID] -= 1;
        DamageModule::set_damage_lock(module_accessor,true);
        DamageModule::set_reaction_mul(module_accessor,0.0);
        if MIPHA_INVINCIBILITY[ENTRY_ID] == 120 {
            lua_bind::FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),false);
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
            EFFECT_FOLLOW(fighter,Hash40::new("sys_fairybottle_navy"),Hash40::new("top"),0.0,5.0,0.0,10,40,0,0.6,true);
        }
    }
    if HEAL[ENTRY_ID] { //heal function
        let curr_damage = DamageModule::damage(module_accessor,0);
        let x = curr_damage/50.0;
        let heal_mul = (0.25 + 0.2*x) * -1.0;
        DamageModule::add_damage(module_accessor,curr_damage*heal_mul,0);
        HEAL[ENTRY_ID] = false;
        PLAY_SE(fighter,Hash40::new("se_link_mipha_grace"));
    }
    STOP_SE(fighter,Hash40::new("se_common_justshield")); //usual parry sound stub
    //parry shield
    if MotionModule::motion_kind(module_accessor) == smash::hash40("just_shield_off") { //new parry setup
        if SET_FRAME[ENTRY_ID] == false {
            MotionModule::set_frame(module_accessor,5.5,false);
            SET_FRAME[ENTRY_ID] = true;
            HitModule::set_hit_stop_mul(module_accessor,0.0,smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8 },0.0);
            HitModule::set_hit_stop_mul(module_accessor,0.0,smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_OPPONENT as u8 },0.0);
            PLAY_SE(fighter,Hash40::new("se_link_parry"));
            EFFECT_FOLLOW(fighter,Hash40::new("sys_smash_flash"),Hash40::new("haver"),0.0,10.0,5.0,0,0,0,1.0,true);
        }
        if MotionModule::frame(module_accessor) >= 5.5 && MotionModule::frame(module_accessor) <= 10.5 {
            MotionModule::set_rate(module_accessor,0.6);
            SlowModule::set_whole(module_accessor,3,0);
        }
        else {
            MotionModule::set_rate(module_accessor,6.0);
            SlowModule::clear_whole(module_accessor);
        }
    }
    else if SET_FRAME[ENTRY_ID] == true {
        SET_FRAME[ENTRY_ID] = false;
        SlowModule::clear_whole(module_accessor);
    }
    //stasis
    //stasis sound and timer setup
    if STASIS_CALLER_ID == ENTRY_ID {
        if START[ENTRY_ID] == true {
            PLAY_SE(fighter,Hash40::new("se_link_stasis_start"));
            START[ENTRY_ID] = false;
        }
        if AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT) == true {
            PLAY_SE(fighter,Hash40::new("se_link_stasis_hit_end"));
        }
        STASIS_TIMER[ENTRY_ID] += 1;
        if STASIS_TIMER[ENTRY_ID] <= 120 {
            if STASIS_TIMER[ENTRY_ID] % 60 == 0 {
                PLAY_SE(fighter,Hash40::new("se_link_stasis_timer"));
            }
        }
        else if STASIS_TIMER[ENTRY_ID] <= 180 {
            if STASIS_TIMER[ENTRY_ID] % 30 == 0 {
                PLAY_SE(fighter,Hash40::new("se_link_stasis_timer"));
            }
        }
        else if STASIS_TIMER[ENTRY_ID] <= 240 {
            if STASIS_TIMER[ENTRY_ID] % 20 == 0 {
                PLAY_SE(fighter,Hash40::new("se_link_stasis_timer"));
            }
        }
        else if STASIS_TIMER[ENTRY_ID] <= 300 {
            if STASIS_TIMER[ENTRY_ID] % 15 == 0 {
                PLAY_SE(fighter,Hash40::new("se_link_stasis_timer"));
            }
        }
        if STASIS_TIMER[ENTRY_ID] == 300 {
            STASIS_TIMER[ENTRY_ID] = 0;
            GLOBAL_STASIS = false;
            SET_UNFREEZE = true;
            SOUND[ENTRY_ID] = false;
            STASIS_COOLDOWN[ENTRY_ID] = true;
            PLAY_SE(fighter,Hash40::new("se_link_stasis_end"));
        }
    }
    if GLOBAL_STASIS == false {
        STASIS_TIMER[ENTRY_ID] = 0;
    }
    //stasis cooldown timer
    if STASIS_COOLDOWN[ENTRY_ID] == true {
        STASIS_COOLDOWN_TIMER[ENTRY_ID] += 1;
        if STASIS_COOLDOWN_TIMER[ENTRY_ID] == 1800 {
            STASIS_COOLDOWN_TIMER[ENTRY_ID] = 0;
            STASIS_COOLDOWN[ENTRY_ID] = false;
        }
    }
    //common
    //variable reset
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY
    || StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_REBIRTH {
        FIRE_ARROW[ENTRY_ID] = false;
        ICE_ARROW[ENTRY_ID] = false;
        SHOCK_ARROW[ENTRY_ID] = false;
        BOMB_ARROW[ENTRY_ID] = false;
        STAMINA[ENTRY_ID] = 210.0;
        STAMINA_EXHAUSTED[ENTRY_ID] = false;
        BULLET_TIME[ENTRY_ID] = false;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            MIPHA_GRACE[ENTRY_ID] = true;
            MIPHA_INVINCIBILITY[ENTRY_ID] = 180;
            HEAL[ENTRY_ID] = false;
        }
    }
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
        SLOW_OPPONENT(fighter,30.0,100.0);
        STAMINA[ENTRY_ID] = 210.0;
        STAMINA_EXHAUSTED[ENTRY_ID] = false;
        BULLET_TIME[ENTRY_ID] = false;
    }
}

//parry shield
//reflect on parry function
extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE"]
    fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool;
}

#[skyline::hook(replace=is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        return true;
    }
    else {
        return false;
    }
}

unsafe fn global_checks(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //stop stasis and bullet time in fs
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
        GLOBAL_BULLET_TIME = false;
        GLOBAL_STASIS = false;
    }
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    //stasis setup on enemy 
    if SET_FREEZE[ENTRY_ID] == true && FREEZE[ENTRY_ID] == false {
        lua_bind::FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),true);
        FREEZE[ENTRY_ID] = true;
        MOTION[ENTRY_ID] = MotionModule::motion_kind(module_accessor);
        FRAME[ENTRY_ID] = MotionModule::frame(module_accessor);
        SET_FREEZE[ENTRY_ID] = false;
        POS[ENTRY_ID] = PostureModule::lr(module_accessor);
        PRE_DAMAGE[ENTRY_ID] = DamageModule::damage(module_accessor,0);
    }
    if FREEZE[ENTRY_ID] == true {
        MotionModule::change_motion(module_accessor,Hash40::new_raw(MOTION[ENTRY_ID]),FRAME[ENTRY_ID],0.0,false,0.0,false,false);
        PostureModule::set_lr(module_accessor,POS[ENTRY_ID]);
        if STASIS_TIMER[STASIS_CALLER_ID] % 30 == 0 {
            COL_NORMAL(fighter);
        }
        else if STASIS_TIMER[STASIS_CALLER_ID] % 15 == 0 {
            FLASH(fighter,1.0,1.0,0.0,1.0);
        }
        POST_DAMAGE[ENTRY_ID] = DamageModule::damage(module_accessor,0);
        if POST_DAMAGE[ENTRY_ID] - PRE_DAMAGE[ENTRY_ID] >= 20.0 {
            DamageModule::set_damage_lock(module_accessor,true);
        }
    }
    if SET_UNFREEZE == true {
        if FREEZE[ENTRY_ID] == true {
            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_DAMAGE,true);
            lua_bind::FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(ENTRY_ID as i32),false);
            FREEZE[ENTRY_ID] = false;
            SET_FREEZE[ENTRY_ID] = false;
            DamageModule::set_damage_lock(module_accessor,false);
            POST_DAMAGE[ENTRY_ID] = 0.0;
            PRE_DAMAGE[ENTRY_ID] = 0.0;
        }
        COL_NORMAL(fighter);
        RESET_UNFREEZE_TIMER += 1;
        if RESET_UNFREEZE_TIMER == 10 {
            SET_UNFREEZE = false;
            RESET_UNFREEZE_TIMER = 0;
            STASIS_CALLER_ID = 8;
        }
    }
    //variable resets
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
        GLOBAL_BULLET_TIME = false;
        GLOBAL_STASIS = false;
        SOUND[ENTRY_ID] = false;
        SET_FRAME[ENTRY_ID] = false;
        FREEZE[ENTRY_ID] = false;
        SET_UNFREEZE = false;
        RESET_UNFREEZE_TIMER = 0;
        STASIS_CALLER_ID = 8;
    }
}

// stasis
//hook of collision offset required for fighter boma used by stasis setup for enemies affected as well as initiating stasis
#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_id);
    ENTRY_ID = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if MotionModule::motion_kind(attacker_boma) == smash::hash40("appeal_s_l")
    || MotionModule::motion_kind(attacker_boma) == smash::hash40("appeal_s_r") {
        if GLOBAL_STASIS == false && STASIS_COOLDOWN[attacker] == false && attacker_fighter_kind == *FIGHTER_KIND_LINK {
            STASIS_CALLER_ID = attacker;
            GLOBAL_STASIS = true;
            SOUND[attacker] = true;
            START[attacker] = true;
            if smash::app::utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                if SET_FREEZE[ENTRY_ID] == false {
                    SET_FREEZE[ENTRY_ID] = true;
                }
            }
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

//hook for disabling any movement if enemy is in stasis
#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if GLOBAL_STASIS == true {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S {
            return false;
        }
        else if FREEZE[ENTRY_ID] == true {
            return false;
        }
        else {
            return ret;
        } 
    }
    else {
        return ret;
    }
}

//hook to stub any sounds coming from attacks by link, also means yes link wont be making any sound on other players but thats fine
#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    if SOUND[ENTRY_ID] == true && GLOBAL_STASIS == true {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        for i in 0..36 {
            let mut x = hitbox_params[i];
            if i == 34 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*COLLISION_SOUND_ATTR_NONE as u64));
            } else {
                l2c_agent.push_lua_stack(&mut x);
            }
        }
    }
    original!()(lua_state);
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn install() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    smash_script::replace_scripts!(
        arrow,
        parry,
        stasis
    );
    smash_script::replace_fighter_frames!(link);
    smash_script::add_fighter_frame_callbacks!(global_checks);
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    skyline::install_hook!(attack_replace);
}
