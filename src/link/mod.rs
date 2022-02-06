use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use crate::FIGHTER_MANAGER;
use crate::ITEM_MANAGER;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
//use crate::draw_meter;

static mut CHANGE : [bool; 8] = [false; 8];
static mut NO_SWORD : [bool; 8] = [false; 8];
static mut SWORD : [bool; 8] = [true; 8];
static mut NO_SHIELD : [bool; 8] = [false; 8];
static mut SHIELD : [bool; 8] = [true; 8];
static mut MODEL_DATA_POS : [u64; 8] = [0; 8]; //model data storage
static mut MESH_OFFSET : usize = 0x35D22B0; //13.0.0
//hook
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;

//globals
const FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER : i32 = 495;
const FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_FROZEN : i32 = 496;
const FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_UNFREEZE : i32 = 497;
const FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_RESET : i32 = 498;
//ints
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_NORMAL : i32 = 0x100000c5;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY : i32 = 0x100000c6;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION : i32 = 0x100000c7;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_LIFETIME : i32 = 0x100000c8;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_OBJECT_ID : i32 = 0x100000c9;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT : i32 = 0x100000ca;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE : i32 = 0x100000cb;
const ARROW_TYPE_FIRE : i32 = 1;
const ARROW_TYPE_ICE : i32 = 2;
const ARROW_TYPE_SHOCK : i32 = 3;
const ARROW_TYPE_BOMB : i32 = 4;
const ARROW_TYPE_NONE : i32 = 0;
const ARROW_TYPE_ANCIENT : i32 = 5;
const ARROW_TYPE_LIGHT : i32 = 6;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_ICE_ARROW_COUNT : i32 = 0x100000cc;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_SHOCK_ARROW_COUNT : i32 = 0x100000cd;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_ARROW_COUNT : i32 = 0x100000ce;
const FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_COOLDOWN : i32 = 0x100000cf;
const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE : i32 = 0x1000000e;
//flags
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_EFFECT : i32 = 0x200000e9;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS : i32 = 0x200000ea;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_INITIALIZE_STASIS : i32 = 0x200000eb;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_INITIATED : i32 = 0x200000ec;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS : i32 = 0x200000ed;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATE : i32 = 0x200000ee;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATED : i32 = 0x200000ef;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB : i32 = 0x200000f0;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT : i32 = 0x200000f1;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_USED : i32 = 0x200000f2;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_UNLOCKED : i32 = 0x200000f3;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FIRE_ARROW_USED : i32 = 0x200000f4;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ICE_ARROW_USED : i32 = 0x200000f5;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SHOCK_ARROW_USED : i32 = 0x200000f6;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_ARROW_USED : i32 = 0x200000f7;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_HALT : i32 = 0x200000f8;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER : i32 = 0x200000f9;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_ZOOM : i32 = 0x200000fa;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_URBOSA_FURY : i32 = 0x200000fb;
const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CRYONIS_WAIT : i32 = 0x200000fc;

pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize { //get entry_id
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

#[acmd_script(agent = "link", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn attack_s4_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        FT_MOTION_RATE(FSM=1.5)
        frame(Frame=14)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=7.0, Angle=69, KBG=12, FKB=0, BKB=45, Size=4.3, X=2.7, Y=-2.0, Z=-2.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=14.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=3.5, X=8.5, Y=-2.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=60, KBG=12, FKB=0, BKB=45, Size=3.2, X=0.0, Y=8.5, Z=6.0, X2=0.0, Y2=8.5, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
        frame(Frame=35)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

//sound setup
#[acmd_script(agent = "link", scripts = ["sound_specialnstart", "sound_specialairnstart"], category = ACMD_SOUND)]
unsafe fn special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter,Hash40::new("se_link_final01"));
        }
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {//light start
            PLAY_STATUS(fighter,Hash40::new("se_link_special_n_start"));
        }
    }
    else {
        sv_animcmd::frame(lua_state,13.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter,Hash40::new("se_link_special_n01"));
        }
    }
}

#[acmd_script(agent = "link", scripts = ["sound_specialnend", "sound_specialairnend"], category = ACMD_SOUND)]
unsafe fn special_n_end_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_link_special_n01"));
        STOP_SE(fighter,Hash40::new("se_link_final01"));
        STOP_SE(fighter,Hash40::new("se_link_special_n_start"));
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT
    || WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
        sv_animcmd::frame(lua_state,2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter,Hash40::new("se_link_final02"));
        }
    }
    else {
        sv_animcmd::frame(lua_state,2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter,Hash40::new("se_link_special_n03"));
        }
    }
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn arrow_fly_pre(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let arrow_type = WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    WorkModule::set_int(module_accessor,arrow_type,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    original!(weapon)
}

//arrow box setup
#[acmd_script(agent = "link_bowarrow", script = "game_fly", category = ACMD_GAME)]
unsafe fn arrow(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_FIRE {
        if is_excute(weapon) {
            WorkModule::add_int(owner_module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT);
            if WorkModule::is_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.0,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_LL,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(module_accessor,1.25);
            }
            else {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FIRE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(module_accessor,1.15);
            }
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
    else if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ICE {
        if is_excute(weapon) {
            WorkModule::add_int(owner_module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ICE_ARROW_COUNT);
            if WorkModule::is_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,100,0,35,1.0,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_LL,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(module_accessor,1.25);
            }
            else {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.5,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_ice"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_FREEZE,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(module_accessor,1.15);
            }
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
    else if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_SHOCK {
        if is_excute(weapon) {
            WorkModule::add_int(owner_module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SHOCK_ARROW_COUNT);
            if WorkModule::is_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT) {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,100,21,0,1.75,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_LL,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_OBJECT);
                AttackModule::set_add_reaction_frame_revised(module_accessor,0,15.0,false);
                AttackModule::set_power_mul(module_accessor,1.5);
            }
            else {
                ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,1.75,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(module_accessor,1.3);
            }
            AttackModule::enable_safe_pos(module_accessor);
            QUAKE(weapon,*CAMERA_QUAKE_KIND_S);
        }
    }
    else if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB {
        if is_excute(weapon) {
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,2.0,0.0,0.0,0.0,None,None,None,0.5,1.0,*ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
            AttackModule::set_power_mul(module_accessor,1.5);
            QUAKE(weapon,*CAMERA_QUAKE_KIND_L);
        }
    }
    else if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
        if is_excute(weapon) {
            WorkModule::on_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_USED);
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,10,5.0,0.0,0.0,4.0,None,None,None,0.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
            AttackModule::set_power_mul(module_accessor,1.75);
            EFFECT_OFF_KIND(weapon,Hash40::new("link_final_arrow_flare"),false,true);
            EFFECT_OFF_KIND(weapon,Hash40::new("link_final_arrow_hold"),false,true);
            QUAKE(weapon,*CAMERA_QUAKE_KIND_L);
        }
        sv_animcmd::frame(lua_state,2.0);
        if is_excute(weapon) {
            EFFECT_FOLLOW(weapon,Hash40::new("link_final_arrow_flare"),Hash40::new("top"),0,0.6,2,0,180,0,1,false);
            LAST_EFFECT_SET_RATE(weapon,2.0);
        }
    }
    else if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
        if is_excute(weapon) {
            ATTACK(weapon,0,0,Hash40::new("top"),5.0,361,71,0,15,2.0,0.0,0.0,0.0,None,None,None,0.5,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,true,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(module_accessor);
        }
    }
    else {
        original!(weapon);
    }
}

//bomb arrow ground blast
#[acmd_script(agent = "link_bowarrow", script = "game_stick", category = ACMD_GAME)]
unsafe fn bomb_arrow_blast(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB {
        if is_excute(weapon) {
            ATTACK(weapon,0,0,Hash40::new("top"),7.0,45,80,0,20,3.0,0.0,0.0,0.0,None,None,None,1.2,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_LL,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_BOMB);
            ATTACK(weapon,1,0,Hash40::new("top"),4.0,45,60,0,15,6.0,0.0,0.0,0.0,None,None,None,1.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_BOMB,*ATTACK_REGION_BOMB);
            QUAKE(weapon,*CAMERA_QUAKE_KIND_L);
            EFFECT(weapon,Hash40::new("sys_bomb_a"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
            PLAY_SE(weapon,Hash40::new("se_common_bomb_m"));
        }
    }
}

//arrow rng boxes
#[acmd_script(agent = "link_bowarrow", script = "game_hitstick", category = ACMD_GAME)]
unsafe fn arrow_blast(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
        if is_excute(weapon) {
            EFFECT_OFF_KIND(weapon,Hash40::new("link_arrow_trace"),false,true);
            EFFECT_OFF_KIND(weapon,Hash40::new("link_arrow"),false,true);
            EFFECT_OFF_KIND(weapon,Hash40::new("link_final_arrow_flare"),false,true);
        }
        if PostureModule::lr(module_accessor) > 0.0 {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("link_final_hit"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        else {
            if is_excute(weapon) {
                EFFECT(weapon, Hash40::new("link_final_hit"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        if is_excute(weapon) {
            PLAY_SE(weapon,Hash40::new("se_link_special_n_fire")); //ancient blast
        }
    }
    if WorkModule::get_int(module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
        if is_excute(weapon) {
            EFFECT_OFF_KIND(weapon,Hash40::new("sys_sscope_hold"),false,true);
            PLAY_SE(weapon,Hash40::new("se_link_special_n_bomb_start")); //light hit
            EFFECT(weapon, Hash40::new("sys_counter_flash"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[skyline::hook(replace = ArticleModule::change_motion)]
unsafe fn article_change_motion_replace(module_accessor: &mut BattleObjectModuleAccessor, article: i32, mut motion: Hash40, is_inherit_frame: bool, frame: f32) -> u64 {
    if article == *FIGHTER_LINK_GENERATE_ARTICLE_BOW {
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
            if motion.hash == hash40("n_start") {
                motion.hash = hash40("n_ancient_start");
            }
            if motion.hash == hash40("n") {
                motion.hash = hash40("n_ancient");
            }
            if motion.hash == hash40("n_end") {
                motion.hash = hash40("n_ancient_end");
            }
            if motion.hash == hash40("n_air_start") {
                motion.hash = hash40("n_air_ancient_start");
            }
            if motion.hash == hash40("n_air") {
                motion.hash = hash40("n_air_ancient");
            }
            if motion.hash == hash40("n_air_end") {
                motion.hash = hash40("n_air_ancient_end");
            }
        }
        else if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
            if motion.hash == hash40("n_start") {
                motion.hash = hash40("n_light_start");
            }
            if motion.hash == hash40("n") {
                motion.hash = hash40("n_light");
            }
            if motion.hash == hash40("n_end") {
                motion.hash = hash40("n_light_end");
            }
            if motion.hash == hash40("n_air_start") {
                motion.hash = hash40("n_air_light_start");
            }
            if motion.hash == hash40("n_air") {
                motion.hash = hash40("n_air_light");
            }
            if motion.hash == hash40("n_air_end") {
                motion.hash = hash40("n_air_light_end");
            }
        }
    }
    original!()(module_accessor,article,motion,is_inherit_frame,frame)
}

//mesh and effect setup
#[smashline::weapon_frame(agent = WEAPON_KIND_LINK_BOWARROW)]
fn link_arrow(weapon: &mut L2CFighterBase) {
    unsafe {
        let lua_state = weapon.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if StatusModule::status_kind(module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_HAVED
        || StatusModule::status_kind(module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_FLY {
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_FIRE {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air"),true,-1.0);
                    }
                }
                EFFECT_FOLLOW(weapon,Hash40::new("sys_damage_fire"),Hash40::new("top"),0,0,0,0,0,0,0.25,false);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_FIRE {
                EFFECT_OFF_KIND(weapon,Hash40::new("sys_damage_fire"),false,true);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ICE {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air"),true,-1.0);
                    }
                }
                EFFECT_FOLLOW(weapon,Hash40::new("sys_freezer"),Hash40::new("top"),0,0,0,0,0,0,0.12,false);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_ICE {
                EFFECT_OFF_KIND(weapon,Hash40::new("sys_freezer"),false,true);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_SHOCK {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air"),true,-1.0);
                    }
                }
                EFFECT_FOLLOW(weapon,Hash40::new("sys_thunder"),Hash40::new("top"),0,0,0,0,0,0,0.3,false);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_SHOCK {
                EFFECT_OFF_KIND(weapon,Hash40::new("sys_thunder"),false,true);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air"),true,-1.0);
                    }
                }
                EFFECT_FOLLOW(weapon,Hash40::new("sys_flame"),Hash40::new("top"),0,0,3,0,0,0,0.12,false);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_ancient_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_ancient"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_ancient_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_ancient"),true,-1.0);
                    }
                }
                if StatusModule::status_kind(module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_HAVED
                && MotionModule::frame(module_accessor) == 0.0 {  
                    EFFECT_FOLLOW(weapon,Hash40::new("link_final_arrow_hold"),Hash40::new("top"),0,-0.5,3,0,0,0,1,false);
                }
                else if StatusModule::status_kind(module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_HAVED {
                    if WorkModule::is_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_EFFECT) {
                        EFFECT_FOLLOW(weapon,Hash40::new("link_final_arrow_flare"),Hash40::new("top"),0,0.6,2,0,180,0,1,false);
                        WorkModule::off_flag(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_EFFECT);
                    }
                }
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_ANCIENT {
                EFFECT_OFF_KIND(weapon,Hash40::new("link_final_arrow_hold"),false,true);
                EFFECT_OFF_KIND(weapon,Hash40::new("link_final_arrow_flare"),false,true);
            }
            if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_GROUND {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_light_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_light"),true,-1.0);
                    }
                }
                else {
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_light_start"),true,-1.0);
                    }
                    if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                        ArticleModule::change_motion(owner_module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOW,Hash40::new("n_air_light"),true,-1.0);
                    }
                }
            }
        }
        if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_FIRE {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ICE {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_SHOCK {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),false);
        }
        else {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("fire"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ice"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("elec"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bomb"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("ancient"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("light_arrow"),false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("arrowa"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("winga"),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("wingb"),true);
        }
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT) == false {
        let rnd = sv_math::rand(hash40("link"),100);
        if rnd <= 30 {
            println!("yes");
            WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT);
        }
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_FIRE
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FIRE_ARROW_USED) {
        WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ICE
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ICE_ARROW_USED) {
        WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_SHOCK
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SHOCK_ARROW_USED) {
        WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_ARROW_USED) {
        WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    }
    if smashball::is_training_mode() == false {
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ANCIENT
        && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_USED) {
            WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
        }
    }
    original!(fighter)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_RND_EFFECT);
    if smashball::is_training_mode() == false {
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_FIRE {
            WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT) <= 0 {
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FIRE_ARROW_USED);
            }
        }
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_ICE {
            WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ICE_ARROW_COUNT);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ICE_ARROW_COUNT) <= 0 {
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ICE_ARROW_USED);
            }
        }
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_SHOCK {
            WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SHOCK_ARROW_COUNT);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SHOCK_ARROW_COUNT) <= 0 {
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SHOCK_ARROW_USED);
            }
        }
        if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_BOMB {
            WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_ARROW_COUNT);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_ARROW_COUNT) <= 0 {
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_ARROW_USED);
            }
        }
    }
    original!(fighter)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn fly_init(weapon: &mut L2CFighterBase) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    original!(weapon);
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::get_int(owner_module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
        let mut l2c_agent = L2CAgent::new(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(15.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_speed(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_accel(lua_state);
        KineticModule::enable_energy(module_accessor,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    }
    return L2CValue::I32(0)
}

#[skyline::hook(offset=0x35d2f50)] //mesh control function hook
pub unsafe fn set_mesh_visibility_lvl_2_replace(model_data: u64,mesh: u64,visibility: u8) -> u64 {
    let mut player_number = 7;
    if sv_information::is_ready_go() {
        for i in 0..8 {
            if MODEL_DATA_POS[i] == model_data {
                player_number = i;
                break;
            }
        }
        if mesh == hash40("link_ken") {
            if visibility == 0 {
                if CHANGE[player_number] {
                    NO_SHIELD[player_number] = true;
                    SWORD[player_number] = false;
                    NO_SWORD[player_number] = true;
                    SHIELD[player_number] = false;
                    CHANGE[player_number] = false;
                }
                else {
                    NO_SWORD[player_number] = true;
                    SWORD[player_number] = false;
                }
            }
            if visibility == 1 {
                if CHANGE[player_number] == false {
                    SWORD[player_number] = true;
                    SHIELD[player_number] = true;
                    CHANGE[player_number] = true;
                }
            }
        }
        if mesh == hash40("link_tate") {
            if visibility == 0 {
                if CHANGE[player_number] {
                    NO_SHIELD[player_number] = true;
                    SWORD[player_number] = true;
                    NO_SWORD[player_number] = false;
                    SHIELD[player_number] = false;
                    CHANGE[player_number] = false;
                }
            }
            if visibility == 1 {
                if CHANGE[player_number] == false {
                    SHIELD[player_number] = true;
                    CHANGE[player_number] = true;
                }
            }
        }
    }
    original!()(model_data,mesh,visibility)
}

//arrow toggle and bow mesh setup
#[smashline::fighter_frame(agent = FIGHTER_KIND_LINK)]
fn link(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        //draw_meter(module_accessor);
        if WorkModule::is_flag(module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_WAIT_SHIELD) {
            smash_script::shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("havel"), 5, 2.5, 0, 0, LUA_VOID, LUA_VOID, LUA_VOID, 0.5, 1, 999, false, 0.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        else {
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        let boma = module_accessor as *mut smash::app::BattleObjectModuleAccessor as u64;
        MODEL_DATA_POS[entry_id(module_accessor)] = *((*((boma + 0x78) as *const u64) + 0x10) as *const u64);
        if NO_SWORD[entry_id(module_accessor)] {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("link_sword_glow"),false);
            EffectModule::kill_kind(module_accessor,Hash40::new("link_bomb_aura"),true,false);
            EffectModule::kill_kind(module_accessor,Hash40::new("link_sword_flare"),true,false);
            NO_SWORD[entry_id(module_accessor)] = false;
        }
        if NO_SHIELD[entry_id(module_accessor)] {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("link_shield_glow"),false);
            EffectModule::kill_kind(module_accessor,Hash40::new("link_sword_flare"),true,false);
            EffectModule::kill_kind(module_accessor,Hash40::new("link_bomb_aura"),true,false);
            NO_SHIELD[entry_id(module_accessor)] = false;
        }
        if SWORD[entry_id(module_accessor)] {
            EFFECT_FOLLOW(fighter,Hash40::new("link_sword_flare"),Hash40::new("sword1"),0,0,0,0,0,0,1,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("sword1"),2.5,0,0,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("sword1"),5,0,0,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("sword1"),7.5,0,0,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("sword1"),10,0,0,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("sword1"),12.5,0,0,0,0,0,0.25,false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("link_sword_glow"),true);
            SWORD[entry_id(module_accessor)] = false;
        }
        if SHIELD[entry_id(module_accessor)] {
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("havel"),1,1,1,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("havel"),1,-1,1,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("havel"),1,1,-1,0,0,0,0.25,false);
            EFFECT_FOLLOW(fighter,Hash40::new("link_bomb_aura"),Hash40::new("havel"),1,-1,-1,0,0,0,0.25,false);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("link_shield_glow"),true);
            SHIELD[entry_id(module_accessor)] = false;
        }
        //start resets
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY
        || (smashball::is_training_mode() && WorkModule::is_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_RESET) == false) {
            if ArticleModule::is_exist(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY) == false {
                ArticleModule::generate_article(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,false,-1);
            }
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_LIGHT {
                WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
            }
            WorkModule::set_int(module_accessor,10,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT);
            WorkModule::set_int(module_accessor,10,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ICE_ARROW_COUNT);
            WorkModule::set_int(module_accessor,5,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SHOCK_ARROW_COUNT);
            WorkModule::set_int(module_accessor,3,FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_ARROW_COUNT);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_USED);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_UNLOCKED);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FIRE_ARROW_USED);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ICE_ARROW_USED);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SHOCK_ARROW_USED);
            WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_ARROW_USED);
            SWORD[entry_id(module_accessor)] = true;
            SHIELD[entry_id(module_accessor)] = true;
            NO_SHIELD[entry_id(module_accessor)] = true;
            NO_SWORD[entry_id(module_accessor)] = true;
            WorkModule::on_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_RESET);
        }
        //elemental arrows
        //setup for arrows based on taunt button input during charge
        if WorkModule::is_flag(module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE)
        || WorkModule::is_flag(module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_FIRE
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_FIRE_ARROW_USED) == false {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_FIRE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    /*match WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT) {
                        1 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_1"),true,-1.0),
                        2 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_2"),true,-1.0),
                        3 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_3"),true,-1.0),
                        4 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_4"),true,-1.0),
                        5 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_5"),true,-1.0),
                        6 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_6"),true,-1.0),
                        7 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_7"),true,-1.0),
                        8 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_8"),true,-1.0),
                        9 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_9"),true,-1.0),
                        10 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_10"),true,-1.0),
                        _ => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_0"),true,-1.0),
                    };*/
                }
                else {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_LW) {
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_ICE
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ICE_ARROW_USED) == false {
                    /*match WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT) {
                        1 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_1"),true,-1.0),
                        2 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_2"),true,-1.0),
                        3 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_3"),true,-1.0),
                        4 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_4"),true,-1.0),
                        5 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_5"),true,-1.0),
                        6 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_6"),true,-1.0),
                        7 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_7"),true,-1.0),
                        8 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_8"),true,-1.0),
                        9 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_9"),true,-1.0),
                        10 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_10"),true,-1.0),
                        _ => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_0"),true,-1.0),
                    };*/
                    WorkModule::set_int(module_accessor,ARROW_TYPE_ICE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
                else {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_SHOCK
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SHOCK_ARROW_USED) == false {
                    /*match WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT) {
                        1 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_1"),true,-1.0),
                        2 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_2"),true,-1.0),
                        3 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_3"),true,-1.0),
                        4 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_4"),true,-1.0),
                        5 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_5"),true,-1.0),
                        _ => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_0"),true,-1.0),
                    };*/
                    WorkModule::set_int(module_accessor,ARROW_TYPE_SHOCK,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
                else {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_BOMB
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_ARROW_USED) == false {
                    /*match WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_FIRE_ARROW_COUNT) {
                        1 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_1"),true,-1.0),
                        2 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_2"),true,-1.0),
                        3 => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_3"),true,-1.0),
                        _ => ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("number_0"),true,-1.0),
                    };*/
                    WorkModule::set_int(module_accessor,ARROW_TYPE_BOMB,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
                else {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_GUARD) {
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) != ARROW_TYPE_ANCIENT
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_USED) == false
                && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_UNLOCKED) {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_ANCIENT,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_EFFECT);
                }
                else {
                    WorkModule::set_int(module_accessor,ARROW_TYPE_NONE,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE) == ARROW_TYPE_NONE
            && DamageModule::damage(module_accessor,0) >= 100.0 {
                WorkModule::set_int(module_accessor,ARROW_TYPE_LIGHT,FIGHTER_LINK_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_UNLOCKED);
            }
        }
        if ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ANCIENT_ARROW_EFFECT);
        }
        //let pos = Vector3f{x: PostureModule::pos_x(module_accessor) - 10.0, y: PostureModule::pos_y(module_accessor) + 20.0, z: 0.0};
        //ArticleModule::set_pos(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,pos);
        //rune choice
        if StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_N {
            /*if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB) {
                ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("remote bomb"),true,-1.0);
            }
            if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS) {
                ArticleModule::change_motion(module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_NAVY,Hash40::new("stasis"),true,-1.0);
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_R) {
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS);
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS);
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB);
            }*/
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI)
            && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_HALT) == false {
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS);
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS);
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB);
            }
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_S_L) {
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS);
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS);
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB);
            }
        }
        //stasis
        if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS) {
            if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_INITIATED) {
                WorkModule::add_int(module_accessor,-1,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
                if WorkModule::get_int(module_accessor,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER) % 30 == 0 {
                    PLAY_SE(fighter,Hash40::new("se_link_stasis_timer"));
                }
                if WorkModule::get_int(module_accessor,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER) <= 0 {
                    WorkModule::set_int(module_accessor,120,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
                    PLAY_SE(fighter,Hash40::new("se_link_stasis_end"));
                    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_INITIALIZE_STASIS);
                    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_INITIATED);
                    if smashball::is_training_mode() == false {
                        WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_HALT);
                        WorkModule::set_int(module_accessor,900,FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_COOLDOWN);
                        WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BOMB);
                        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS);
                    }
                }
            }
        }
        if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_HALT) {
            WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_COOLDOWN);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_COOLDOWN) <= 0 {
                WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_HALT);
            }
        }
        //cryonis
        if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS) {
            if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATED) {
                let object_id = WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_OBJECT_ID) as u32;
                let cryonis_boma = sv_battle_object::module_accessor(object_id);
                if MotionModule::motion_kind(cryonis_boma) == hash40("born") && MotionModule::is_end(cryonis_boma) {
                    WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CRYONIS_WAIT);
                    MotionModule::change_motion(cryonis_boma,Hash40::new("wait"),0.0,1.0,false,0.0,false,false);
                }
                if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CRYONIS_WAIT) {
                    WorkModule::add_int(module_accessor,-1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_LIFETIME);
                }
                let conditions = [ITEM_KINETIC_ENERGY_ID_GROUND,ITEM_KINETIC_ENERGY_ID_ROT,ITEM_KINETIC_ENERGY_ID_WIND,ITEM_KINETIC_ENERGY_ID_OUTER,ITEM_KINETIC_ENERGY_ID_DAMAGE,ITEM_KINETIC_ENERGY_ID_JOSTLE,ITEM_KINETIC_ENERGY_ID_MOTION,ITEM_KINETIC_ENERGY_ID_CONTROL,ITEM_KINETIC_ENERGY_ID_CONTROL_ROT,ITEM_KINETIC_ENERGY_ID_MOTION_LINKED_MAIN,ITEM_KINETIC_ENERGY_ID_MOTION_LINKED_SUB1];
                for x in conditions.iter() {
                    KineticModule::unable_energy(cryonis_boma,**x);
                }
                KineticModule::clear_speed_all(cryonis_boma);
                if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_LIFETIME) <= 0
                || StopModule::is_damage(cryonis_boma) {
                    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CRYONIS_WAIT);
                    MotionModule::change_motion(cryonis_boma,Hash40::new("break"),0.0,1.0,false,0.0,false,false);
                }
                if MotionModule::motion_kind(cryonis_boma) == hash40("break") && MotionModule::is_end(cryonis_boma) {
                    WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATED);
                    let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager,object_id);
                }
            }
        }
    }
}

#[smashline::fighter_frame_callback]
fn global_checks(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        //stasis
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        if WorkModule::is_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_FROZEN) {
            lua_bind::FighterManager::set_position_lock(fighter_manager,smash::app::FighterEntryID(entry_id(module_accessor) as i32),true);
            WHOLE_HIT(fighter,*HIT_STATUS_XLU);
            MotionModule::set_rate(module_accessor,0.0);
            AttackModule::clear_all(module_accessor);
            WorkModule::add_int(module_accessor,-1,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
            if WorkModule::get_int(module_accessor,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER) % 60 == 0 {
                COL_NORMAL(fighter);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER) % 30 == 0 {
                FLASH(fighter,1,1,0,1);
            }
        }
        if WorkModule::get_int(module_accessor,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER) <= 0 {
            WorkModule::on_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_UNFREEZE);
        }
        if WorkModule::is_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_UNFREEZE) {
            lua_bind::FighterManager::set_position_lock(fighter_manager,FighterEntryID(entry_id(module_accessor) as i32),false);
            MotionModule::set_rate(module_accessor,1.0);
            COL_NORMAL(fighter);
            WHOLE_HIT(fighter,*HIT_STATUS_NORMAL);
            WorkModule::off_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_FROZEN);
            WorkModule::off_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_UNFREEZE);
            WorkModule::set_int(module_accessor,120,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
        }
        //variable resets
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::set_int(module_accessor,120,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
        }
        if smashball::is_training_mode() {
            if WorkModule::is_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_RESET) == false {
                WorkModule::set_int(module_accessor,120,FIGHTER_INSTANCE_WORK_ID_INT_STASIS_TIMER);
                WorkModule::on_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_RESET);
            }
        }
    }
}

#[acmd_script(agent = "link", scripts = ["game_speciallwstasis", "game_specialairlwstasis"], category = ACMD_GAME)]
unsafe fn special_lw_stasis(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_INITIALIZE_STASIS);
        ATTACK(fighter,0,0,Hash40::new("top"),0.0,361,0,0,0,20.0,0.0,10.0,20.0,None,None,None,0.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_FIGHTER,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(lua_state,14.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_INITIALIZE_STASIS);
        AttackModule::clear_all(module_accessor);
    }
}

#[acmd_script(agent = "link", scripts = ["effect_speciallwstasis", "effect_specialairlwstais"], category = ACMD_EFFECT)]
unsafe fn effect_special_lw_stasis(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
}

#[acmd_script(agent = "link", scripts = ["sound_speciallwstasis", "sound_specialairlwstasis"], category = ACMD_SOUND)]
unsafe fn sound_special_lw_stasis(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_rune_filter"));
    }
    sv_animcmd::frame(lua_state,9.0);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_link_rune_filter"));
    }
}

#[acmd_script(agent = "link", scripts = ["game_speciallwcryonis", "game_specialairlwcryonis"], category = ACMD_GAME)]
unsafe fn special_lw_cryonis(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATE);
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS)
    || WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS) {
        StatusModule::init_settings(module_accessor,SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,0,0,0,0);
        FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64 | *FIGHTER_LOG_MASK_FLAG_SHOOT as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,0);
        L2CValue::I32(0)
    }
    else {
        original!(fighter)
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STASIS) {
        WorkModule::set_int(module_accessor,StatusModule::situation_kind(module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(module_accessor,Hash40::new("special_lw_stasis"),0.0,1.0,false,0.0,false,false);
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw_stasis"),0.0,1.0,false,0.0,false,false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(stasis_main_loop as *const () as _))
    }
    else if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS) {
        WorkModule::set_int(module_accessor,StatusModule::situation_kind(module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(module_accessor,Hash40::new("special_lw_cryonis"),0.0,1.0,false,0.0,false,false);
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw_cryonis"),0.0,1.0,false,0.0,false,false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(cryonis_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

pub unsafe extern "C" fn stasis_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION) != StatusModule::situation_kind(module_accessor) {
        WorkModule::set_int(module_accessor,StatusModule::situation_kind(module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION);
        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("special_lw_stasis"),-1.0,1.0,0.0);
    }
    if AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT) {
        PLAY_SE(fighter,Hash40::new("se_link_stasis_start"));
    }
    if MotionModule::is_end(module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn cryonis_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION) != StatusModule::situation_kind(module_accessor) {
        WorkModule::set_int(module_accessor,StatusModule::situation_kind(module_accessor),FIGHTER_LINK_INSTANCE_WORK_ID_INT_STASIS_PREV_SITUATION);
        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,Hash40::new("special_lw_cryonis"),-1.0,1.0,0.0);
    }
    let link_pos = Vector3f{x: PostureModule::pos_x(module_accessor) + (10.0 * PostureModule::lr(module_accessor)), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATE)
    && GroundModule::get_distance_to_floor(module_accessor,&link_pos,link_pos.y,true) > 0.0 {
        ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_CARRIERBOX),0,0,false,false);
        let object_id = ItemModule::get_have_item_id(module_accessor,0) as u32;
        WorkModule::set_int(module_accessor,object_id as i32,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_OBJECT_ID);
        ItemModule::drop_item(module_accessor,0.0,0.0,0);
        let cryonis_boma = sv_battle_object::module_accessor(object_id);
        let mut cryonis_pos = Vector3f{x: PostureModule::pos_x(cryonis_boma), y: PostureModule::pos_y(cryonis_boma), z: PostureModule::pos_z(cryonis_boma)};
        let distance = GroundModule::get_distance_to_floor(cryonis_boma,&cryonis_pos,cryonis_pos.y,true);
        cryonis_pos.y -= distance;
        PostureModule::set_pos(cryonis_boma,&cryonis_pos);
        WorkModule::set_int(module_accessor,300,FIGHTER_LINK_INSTANCE_WORK_ID_INT_CRYONIS_LIFETIME);
        MotionModule::change_motion(cryonis_boma,Hash40::new("born"),0.0,1.0,false,0.0,false,false);
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATE);
        WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATED);
    }
    else {
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATE);
    }
    KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if MotionModule::is_end(module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

#[acmd_script(agent = "link", script = "effect_specialhiholdurbosafury", category = ACMD_EFFECT)]
unsafe fn special_hi_hold_urbosa_fury_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            EFFECT(hash40("sys_thunder"),hash40("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,true)
            EFFECT(hash40("sys_thunder"),hash40("top"),0,0,20,0,0,0,1,0,0,0,0,0,0,true)
            EFFECT(hash40("sys_thunder"),hash40("top"),0,0,-20,0,0,0,1,0,0,0,0,0,0,true)
            EFFECT(hash40("sys_thunder"),hash40("top"),0,0,40,0,0,0,1,0,0,0,0,0,0,true)
            EFFECT(hash40("sys_thunder"),hash40("top"),0,0,-40,0,0,0,1,0,0,0,0,0,0,true)
        }
    });
}

#[acmd_script(agent = "link", script = "sound_specialhihold", category = ACMD_SOUND)]
unsafe fn special_hi_hold_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_scene_slow")); //hold se
    }
}

#[acmd_script(agent = "link", script = "sound_specialairhipre", category = ACMD_SOUND)]
unsafe fn special_air_hi_pre_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h02")); //revali vc
    }
    sv_animcmd::frame(lua_state,15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h03")); //wind
    }
}

#[acmd_script(agent = "link", script = "sound_specialairhi", category = ACMD_SOUND)]
unsafe fn special_air_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h04")); //revali vc2
    }
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_h05")); //wind2
    }
}

#[acmd_script(agent = "link", script = "sound_specialhiholdurbosafury", category = ACMD_SOUND)]
unsafe fn special_hi_hold_urbosa_fury_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        STOP_SE(fighter,Hash40::new("se_link_scene_slow"));
        PLAY_SE(fighter,Hash40::new("se_link_stasis_hit")); //urbosa fury lightning range
    }
}

#[acmd_script(agent = "link", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=14.0, Angle=45, KBG=88, FKB=0, BKB=60, Size=4.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=14.0, Angle=45, KBG=88, FKB=0, BKB=60, Size=3.5, X=2.5, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=11.2, Angle=45, KBG=88, FKB=0, BKB=60, Size=2.8, X=8.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=12.0, Angle=43, KBG=88, FKB=0, BKB=52, Size=4.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=12.0, Angle=43, KBG=88, FKB=0, BKB=52, Size=3.5, X=2.5, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=9.6, Angle=43, KBG=88, FKB=0, BKB=52, Size=2.8, X=8.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=43, KBG=84, FKB=0, BKB=48, Size=4.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=43, KBG=84, FKB=0, BKB=48, Size=3.5, X=2.5, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=7.2, Angle=43, KBG=84, FKB=0, BKB=48, Size=2.8, X=8.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=16)
        if(is_excute){
            MotionModule::set_frame(40.0,false)
        }
        frame(Frame=40)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=48)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X)
        }
    });
}

#[acmd_script(agent = "link", script = "effect_specialhi", category = ACMD_EFFECT)]
unsafe fn effect_special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            AFTER_IMAGE4_ON_arg29(hash40("tex_link_sword1"), hash40("tex_link_sword2"), 4, hash40("sword1"), 1, 0, 0, hash40("sword1"), 14.6, 0.2, -0.2, true, hash40("null"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.6, 0.2)
        }
        frame(Frame=2)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_whirlwind_l"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_ALPHA(0.6)
            LAST_EFFECT_SET_RATE(1.3)
        }
        frame(Frame=9)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_whirlwind_l"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
            LAST_EFFECT_SET_ALPHA(0.6)
            LAST_EFFECT_SET_RATE(1.3)
        }
        frame(Frame=20)
        if(is_excute){
            FLASH_FRM(5, 0, 0, 0, 0)
            AFTER_IMAGE_OFF(6)
            LANDING_EFFECT(hash40("sys_whirlwind_l"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=23)
        if(is_excute){
            COL_NORMAL()
        }
    });
}

#[acmd_script(agent = "link", script = "game_specialairhi", category = ACMD_GAME)]
unsafe fn special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    acmd!(lua_state, {
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
    });
}

#[acmd_script(agent = "link", script = "effect_specialairhi", category = ACMD_EFFECT)]
unsafe fn effect_special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        EFFECT(fighter,Hash40::new("sys_windwave"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
    }
}

#[acmd_script(agent = "link", script = "game_specialhiurbosafury", category = ACMD_GAME)]
unsafe fn special_hi_urbosa_fury(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_n_ice")); //lightning strike
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_thunder"),true,false);
        EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),0,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),20,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),-20,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),40,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
        EFFECT(fighter,Hash40::new("sys_thunder_flash"),Hash40::new("top"),-40,0,0,0,0,0,0.75,0,0,0,0,0,0,false);
    }
    sv_animcmd::frame(lua_state,3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_special_n_shock")); //urbosa fury vc
    }
    sv_animcmd::frame(lua_state,4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("vc_link_special_h01"));
        ATTACK(fighter,0,0,Hash40::new("top"),15.0,45,90,0,55,20.0,0.0,25.0,12.5,Some(0.0),Some(25.0),Some(25.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        ATTACK(fighter,1,0,Hash40::new("top"),15.0,45,90,0,55,20.0,0.0,10.0,12.5,Some(0.0),Some(10.0),Some(25.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        ATTACK(fighter,2,0,Hash40::new("top"),15.0,45,90,0,55,20.0,0.0,25.0,-12.5,Some(0.0),Some(25.0),Some(-25.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
        ATTACK(fighter,3,0,Hash40::new("top"),15.0,45,90,0,55,20.0,0.0,10.0,-12.5,Some(0.0),Some(10.0),Some(-25.0),2.0,1.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS,false,0.0,0.0,0.0,false,false,false,false,false,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_elec"),*ATTACK_SOUND_LEVEL_L,*COLLISION_SOUND_ATTR_ELEC,*ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(lua_state,6.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_bg_criticalhit"),true,false);
        SlowModule::clear_whole(module_accessor);
    }
    sv_animcmd::frame(lua_state,10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_ZOOM);
    }
    sv_animcmd::frame(lua_state,16.0);
    if is_excute(fighter) {
        MotionModule::set_frame(module_accessor,40.0,false);
    }
}

pub unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
        }
    }
    else if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        if MotionModule::is_end(module_accessor) {
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(),false.into());
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_int(module_accessor,0,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_NORMAL);
        WorkModule::set_int(module_accessor,10,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY);
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER);
        WorkModule::off_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_URBOSA_FURY);
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_start"),0.0,1.0,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
    }
    /*else {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi_pre"),0.0,1.0,false,0.0,false,false);
        if ControlModule::get_stick_x(module_accessor) > 0.3 {
            PostureModule::set_lr(module_accessor,1.0);
        }
        else if ControlModule::get_stick_x(module_accessor) < -0.3 {
            PostureModule::lr(module_accessor,-1.0);
        }
        let mut l2c_agent = L2CAgent::new(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_STOP));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.02));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_brake(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_STOP));
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*ENERGY_STOP_RESET_TYPE_AIR));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::reset_energy(lua_state);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
    }*/
    else {
        original!(fighter)
    }
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi_hold") {
            WorkModule::add_int(module_accessor,1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_NORMAL);
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_NORMAL) >= 45 {
                MotionModule::change_motion(module_accessor,Hash40::new("special_hi_hold_urbosa_fury"),10.0,1.0,false,0.0,false,false);
                WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_URBOSA_FURY);
            }
        }
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi_hold_urbosa_fury") {
            WorkModule::add_int(module_accessor,1,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY);
            if StopModule::is_damage(module_accessor) {
                EFFECT_OFF_KIND(fighter,Hash40::new("sys_thunder"),true,false);
            }
            if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY) >= 55 {
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(),false.into());
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

unsafe extern "C" fn special_air_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if GroundModule::can_entry_cliff(module_accessor) == 1 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
        return L2CValue::I32(1)
    }
    if MotionModule::is_end(module_accessor) {
        WorkModule::set_float(module_accessor,30.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return L2CValue::I32(1)
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_hold"),0.0,1.0,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_hold_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("special_air_hi"),0.0,1.0,false,0.0,false,false);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_air_hi_hold_main_loop as *const () as _))
    }
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_hi_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        let speed_x = 0.5 * ControlModule::get_stick_x(module_accessor);
        let mut l2c_agent = L2CAgent::new(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(2.0));
        sv_kinetic_energy::set_speed(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.1));
        sv_kinetic_energy::set_brake(lua_state);
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.5));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(50.0));
        sv_kinetic_energy::set_limit_speed(lua_state);
    }
    return L2CValue::I32(0)
}

pub unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::motion_kind(module_accessor) == hash40("special_hi")
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER) == false {
        for i in 0..3 {
            if AttackModule::is_attack(module_accessor,i,false) {
                let power = AttackModule::get_power(module_accessor,i,false,1.0,false) as f32 + (WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_NORMAL)/5) as f32;
                AttackModule::set_power(module_accessor,i,power,false);
                if i == 2 {
                    WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER);
                }
            }
        }
    }
    if MotionModule::motion_kind(module_accessor) == hash40("special_hi_urbosa_fury")
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER) == false {
        for i in 0..2 {
            if AttackModule::is_attack(module_accessor,i,false) {
                let power = AttackModule::get_power(module_accessor,i,false,1.0,false) as f32 + (WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY)/5) as f32;
                AttackModule::set_power(module_accessor,i,power,false);
                if i == 1 {
                    WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_POWER);
                }
            }
        }
    }
    if WorkModule::get_int(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FURY) >= 40 {
        if AttackModule::is_infliction(module_accessor,*COLLISION_KIND_MASK_HIT) == true
        && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_ZOOM) == false {
            EFFECT(fighter,Hash40::new("sys_bg_criticalhit"),Hash40::new("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false);
            SlowModule::set_whole(module_accessor,2,0);
            CAM_ZOOM_IN_arg5(
                fighter,
                /*frames to zoom in totally*/3.0,
                /*idk best kept 0*/0,
                /*the amt of zoom needed, higher the number lower the zoom*/1.8,
                /*y_rot*/0.0,
                /*x_rot*/0.0
            );
            PLAY_SE(fighter,Hash40::new("se_common_criticalhit"));
            WorkModule::on_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_SET_ZOOM);
        }
    }
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_URBOSA_FURY) {
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi_urbosa_fury"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(module_accessor,Hash40::new("special_hi"),0.0,1.0,false,0.0,false,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

#[status_script(agent = "link", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}

#[acmd_script(agent = "link", script = "game_justshieldoff", category = ACMD_GAME)]
unsafe fn just_shieldoff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        PLAY_SE(fighter,Hash40::new("se_link_parry")); //daruk vc
        ATTACK(fighter,0,0,Hash40::new("top"),1.0,361,50,0,5,10.0,0.0,10.0,10.0,None,None,None,0.0,0.0,*ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F,false,0.0,0.0,0.0,false,false,false,false,true,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_bind_extra"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(lua_state,5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE"]
    fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool;
}

#[skyline::hook(replace=is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_hook(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        return true;
    }
    else {
        return false;
    }
}

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    if attacker_kind == *FIGHTER_KIND_LINK && WorkModule::is_flag(attacker_boma,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_INITIALIZE_STASIS) {
        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            WorkModule::on_flag(defender_boma,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_FROZEN);
            WorkModule::on_flag(attacker_boma,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_STASIS_INITIATED);
        }
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    if WorkModule::is_flag(module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_STASIS_FROZEN) {
        return false;
    }
    if WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS_ACTIVATED)
    && WorkModule::is_flag(module_accessor,FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CRYONIS) {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
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

static MESH_SEARCH_CODE: &[u8] = &[
    0xb5, 0x02, 0x40, 0xf9, 0xf5, 0xfb, 0xff, 0xb4,
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
        LookupSymbol(
            &mut ITEM_MANAGER,
            "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
        //if let Some(offset) = find_subsequence(text, MESH_SEARCH_CODE) {
        //    MESH_OFFSET = offset;
        //}
    }
    install_acmd_scripts!(
        attack_s4_s,
        special_n_start_sound,
        special_n_end_sound,
        arrow,
        bomb_arrow_blast,
        arrow_blast,
        effect_special_lw_stasis,
        sound_special_lw_stasis,
        special_lw_stasis,
        special_lw_cryonis,
        special_hi,
        effect_special_hi,
        //effect_special_air_hi,
        special_hi_hold_sound,
        special_air_hi_pre_sound,
        //special_air_hi_sound,
        //special_air_hi,
        special_hi_hold_urbosa_fury_effect,
        special_hi_hold_urbosa_fury_sound,
        special_hi_urbosa_fury,
        just_shieldoff
    );
    install_agent_frames!(link,link_arrow);
    install_agent_frame_callbacks!(global_checks);
    install_status_scripts!(
        special_n_pre,
        special_n_end,
        special_lw_pre,
        special_lw_main,
        special_hi_main,
        special_hi_end,
        special_hi_hold_main,
        special_hi_hold_exec,
        special_hi_end_main,
        special_hi_end_init,
        fly_init,
        arrow_fly_pre,
    );
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    skyline::install_hook!(article_change_motion_replace);
    skyline::install_hook!(set_mesh_visibility_lvl_2_replace);
}
