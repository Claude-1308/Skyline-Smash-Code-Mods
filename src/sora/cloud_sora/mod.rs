use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use acmd::{acmd,acmd_func};
use smash::hash40;
use crate::FIGHTER_MANAGER;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use smash::app::BattleObjectModuleAccessor;
mod replacement_files;
use replacement_files::ARC_FILES;

static mut FIRE : [bool; 8] = [false; 8];
static mut ICE : [bool; 8] = [false; 8];
static mut ELEC : [bool; 8] = [false; 8];
static mut POISON : [bool; 8] = [false; 8];
static mut SWORD_TAKEN : [bool; 8] = [false; 8];
static mut LIMIT_CLEAR : [i32; 8] = [0; 8];
static mut SORA_BREAK : [bool; 8] = [true; 8];
static mut SPECIAL_LW_COUNTER : [bool; 8] = [false; 8];
static mut SPECIAL_LW_COUNTER_CHECK : [bool; 8] = [false; 8];
static mut SPECIAL_LW_COUNTER_HIT : [bool; 8] = [false; 8];
static mut SPECIAL_LW_HIT : [bool; 8] = [false; 8];
static mut SPECIAL_LW_COUNTER_TIME : [f32; 8] = [0.0; 8];
static mut SPECIAL_LW_HIT_TIME : [f32; 8] = [0.0; 8];
static mut SPECIAL_N : [bool; 8] = [false; 8];
static mut SPECIAL_N_COOL_DOWN : [i32; 8] = [0; 8];
static mut SPECIAL_HI : [bool; 8] = [false; 8];
static mut DAIR : [bool; 8] = [false; 8];
static mut DAIR_COOL_DOWN : [i32; 8] = [0; 8];
static mut DOWN_SMASH : [bool; 8] = [false; 8];
static mut DOWN_SMASH_COOL_DOWN : [i32; 8] = [0; 8];
static mut UP_SMASH : [bool; 8] = [false; 8];
static mut UP_SMASH_HOLD : [bool; 8] = [false; 8];
static mut UP_SMASH_COOL_DOWN : [i32; 8] = [0; 8];
static mut SIDE_SMASH : [bool; 8] = [false; 8];
static mut SIDE_SMASH_COOL_DOWN : [i32; 8] = [0; 8];
static mut SIDE_SMASH_HOLD : [bool; 8] = [false; 8];
static mut VICTORY_COLOR_INDEX: u32 = 0;

pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

pub unsafe fn cloud_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn color_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 1
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 3
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_11",
    animcmd = "game_attack11")] //done
pub fn attack_11(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=4)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=35, Size=2.0, X=0.0, Y=10.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=25, FKB=0, BKB=35, Size=2.0, X=0.0, Y=10.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=180, KBG=15, FKB=0, BKB=20, Size=2.3, X=0.0, Y=10.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=361, KBG=15, FKB=0, BKB=20, Size=2.3, X=0.0, Y=10.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=7)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            }
            frame(Frame=16)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
            }
            frame(Frame=30)
            if(is_excute){
                CancelModule::enable_cancel()
            }
            frame(Frame=44)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(48.0,false)
            }
            frame(Frame=53)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=1.8, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=1.8, X=0.0, Y=9.0, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=25, Size=2.0, X=0.0, Y=9.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=25, Size=2.0, X=0.0, Y=9.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                AttackModule::set_add_reaction_frame(0,2.0,false)
                AttackModule::set_add_reaction_frame(1,2.0,false)
                AttackModule::set_add_reaction_frame(2,2.0,false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=58)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            }
            frame(Frame=62)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_12",
    animcmd = "game_attack12")] //done
pub fn attack_12(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=35, Size=3.0, X=0.0, Y=9.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=35, Size=3.0, X=0.0, Y=9.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=25, Size=3.5, X=0.0, Y=9.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=8)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            }
            frame(Frame=34)
            if(is_excute){
                CancelModule::enable_cancel()
            }
            frame(Frame=47)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(51.0,false)
            }
            frame(Frame=56)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=40, Size=3.0, X=0.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=40, Size=3.2, X=0.0, Y=10.0, Z=6.5, X2=0.0, Y2=9.2, Z2=6.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=3.4, X=0.0, Y=11.0, Z=10.0, X2=0.0, Y2=9.5, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_add_reaction_frame(0,5.0,false)
                AttackModule::set_add_reaction_frame(1,5.0,false)
                AttackModule::set_add_reaction_frame(2,5.0,false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=63)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_13",
    animcmd = "effect_attack13")]
pub fn effect_13(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=3)
            if(is_excute){
                EFFECT_OFF_KIND(hash40("sys_attack_arc_b"),true,true)
            }
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(5.000002)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=6)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
            frame(Frame=8)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(61.0,false)
            }
            frame(Frame=65)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=71)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
            frame(Frame=72)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_13",
    animcmd = "game_attack13")] //done
pub fn attack_13(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 3.5;
            let mut kbg = 50;
            let mut bkb = 75;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 5.0;
                        kbg = 25;
                        bkb = 40;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 2.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 4.5;
                        kbg = 40;
                        bkb = 55;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=30, KBG=kbg, FKB=0, BKB=bkb, Size=4.5, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=9.0, Z2=8.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=45)
            if(is_excute){
                CancelModule::enable_cancel()
            }
            frame(Frame=58)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(61.0,false)
            }
            frame(Frame=66)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=12.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=4.0, X=0.0, Y=5.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=45, KBG=100, FKB=0, BKB=60, Size=3.3, X=0.0, Y=1.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            }
            wait(Frames=4)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_dash",
    animcmd = "effect_attackdash")]
pub fn effect_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=7)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_dash_smoke"),hash40("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false)
            }
            frame(Frame=9)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(9.000002)
            }
            frame(Frame=13)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=19)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=7)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_dash_smoke"),hash40("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,false)
            }
            frame(Frame=9)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(9.000002)
            }
            frame(Frame=13)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=19)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_dash",
    animcmd = "game_attackdash")] //done
pub fn attack_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 11.0;
            let mut dmg2 = 8.0;
            let mut kbg1 = 70;
            let mut kbg2 = 50;
            let mut bkb = 100;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 13.0;
                        dmg2 = 10.0;
                        kbg1 = 45;
                        kbg2 = 25;
                        bkb = 70;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 9.5;
                        dmg2 = 6.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 12.0;
                        dmg2 = 9.0;
                        kbg1 = 55;
                        kbg2 = 38;
                        bkb = 85;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=9)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=60, KBG=kbg1, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=60, KBG=kbg1, FKB=0, BKB=bkb, Size=4.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=60, KBG=kbg1, FKB=0, BKB=bkb, Size=3.0, X=0.0, Y=8.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=65, KBG=kbg2, FKB=0, BKB=bkb, Size=4.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=65, KBG=kbg2, FKB=0, BKB=bkb, Size=3.3, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg2, Angle=65, KBG=kbg2, FKB=0, BKB=bkb, Size=2.8, X=0.0, Y=8.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=9)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=60, KBG=70, FKB=0, BKB=100, Size=5.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=60, KBG=70, FKB=0, BKB=100, Size=4.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.0, Angle=60, KBG=70, FKB=0, BKB=100, Size=3.0, X=0.0, Y=8.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=65, KBG=50, FKB=0, BKB=100, Size=4.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=65, KBG=50, FKB=0, BKB=100, Size=3.3, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=65, KBG=50, FKB=0, BKB=100, Size=2.8, X=0.0, Y=8.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=5)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_s3_s",
    animcmd = "effect_attacks3")]
pub fn effect_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(8.000002)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=10)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
            frame(Frame=11)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(70.0,false)
            }
            frame(Frame=82)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=83)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
            frame(Frame=86)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_s3_s",
    animcmd = "game_attacks3")] //done
pub fn attack_s3(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 11.0;
            let mut kbg = 100;
            let mut bkb = 20;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 13.0;
                        kbg = 70;
                        bkb = 10;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 9.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 12.0;
                        kbg = 85;
                        bkb = 15;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=9)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=6.0, X=0.0, Y=9.0, Z=14.5, X2=0.0, Y2=9.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=36)
            if(is_excute){
                CancelModule::enable_cancel()
            }
            frame(Frame=68)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(70.0,false)
            }
            FT_MOTION_RATE(FSM=0.77)
            frame(Frame=83)
            FT_MOTION_RATE(FSM=0.8)
            frame(Frame=84)
            if(is_excute){
                if(WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK)) {
                    MotionModule::set_rate(1.25)
                }
                else {
                    MotionModule::set_rate(1.0)
                }
            }
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=25, Size=4.0, X=0.0, Y=10.0, Z=10.0, X2=0.0, Y2=8.0, Z2=25.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_hi3",
    animcmd = "effect_attackhi3")]
pub fn effect_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(5.000001)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=7)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
            frame(Frame=10)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
            frame(Frame=11)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(5.000001)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=7)
            if(is_excute){
                FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
            frame(Frame=10)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
            frame(Frame=11)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")] //done
pub fn attack_hi3(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 8.0;
            let mut kbg = 100;
            let mut bkb = 46;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 10.0;
                        kbg = 70;
                        bkb = 30;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 6.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 9.0;
                        kbg = 85;
                        bkb = 40;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,3,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=95, KBG=kbg, FKB=0, BKB=bkb, Size=5.6, X=0.0, Y=12.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=95, KBG=kbg, FKB=0, BKB=bkb, Size=5.0, X=0.0, Y=8.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=95, KBG=kbg, FKB=0, BKB=bkb, Size=6.0, X=-1.0, Y=3.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=dmg, Angle=95, KBG=kbg, FKB=0, BKB=bkb, Size=4.5, X=0.0, Y=21.0, Z=0.0, X2=0.0, Y2=16.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            }
            wait(Frames=3)
            if(is_excute){
                AttackModule::clear(ID=3,false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=6)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=87, KBG=85, FKB=0, BKB=55, Size=3.9, X=0.0, Y=12.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=87, KBG=85, FKB=0, BKB=55, Size=4.2, X=0.0, Y=8.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=87, KBG=85, FKB=0, BKB=55, Size=4.5, X=-1.0, Y=3.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=87, KBG=85, FKB=0, BKB=55, Size=3.7, X=0.0, Y=21.0, Z=0.0, X2=0.0, Y2=16.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
                AttackModule::set_add_reaction_frame(0,5.0,false)
                AttackModule::set_add_reaction_frame(1,5.0,false)
                AttackModule::set_add_reaction_frame(2,5.0,false)
                AttackModule::set_add_reaction_frame(3,5.0,false)
            }
            wait(Frames=3)
            if(is_excute){
                AttackModule::clear(ID=3,false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_s4_s",
    animcmd = "effect_attacks4")]
pub fn effect_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),65534,10,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=19)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(19.000004)
                EFFECT(73574333809 as u64,hash40("top"),0,0,0,0,0,0,1,true)
                EffectModule::set_disable_render_offset_last()
            }
            frame(Frame=20)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=21)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
            frame(Frame=24)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(24.000004)
            }
            frame(Frame=27)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
            frame(Frame=28)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(28.000004)
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=31)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            if(is_excute){
                MotionModule::set_frame(79.0,false)
            }
            frame(Frame=81)
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),65534,10,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=90)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=91)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=108)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_s4_hold",
    animcmd = "game_attacks4charge")]
pub fn attack_s4_hold(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor) == false) {
            if(SIDE_SMASH_HOLD[entry_id(module_accessor)] == false){
                MotionModule::set_frame(61.0,false)
                rust {
                    SIDE_SMASH_HOLD[entry_id(module_accessor)] = true;
                }
            }
            if(MotionModule::frame(module_accessor) >= 122.0) {
                MotionModule::set_frame(122.0,false)
            }
        }
        else {
            if(MotionModule::frame(module_accessor) >= 59.0) {
                MotionModule::set_frame(59.0,false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4")] //done
pub fn attack_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 3.0;
            let mut dmg2 = 13.0;
            let mut kbg1 = 100;
            let mut kbg2 = 115;
            let mut bkb = 42;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype1 = *COLLISION_SOUND_ATTR_CLOUD_SMASH_01;
            let mut sfxtype2 = *COLLISION_SOUND_ATTR_CLOUD_SMASH_02;
            let mut sfxtype3 = *COLLISION_SOUND_ATTR_CLOUD_SMASH_03;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 5.0;
                        dmg2 = 15.0;
                        kbg1 = 70;
                        kbg2 = 75;
                        bkb = 30;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype1 = *COLLISION_SOUND_ATTR_FIRE;
                        sfxtype2 = *COLLISION_SOUND_ATTR_FIRE;
                        sfxtype3 = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 1.0;
                        dmg2 = 10.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype1 = *COLLISION_SOUND_ATTR_FREEZE;
                        sfxtype2 = *COLLISION_SOUND_ATTR_FREEZE;
                        sfxtype3 = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 4.0;
                        dmg2 = 14.0;
                        kbg1 = 80;
                        kbg2 = 100;
                        bkb = 35;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype1 = *COLLISION_SOUND_ATTR_ELEC;
                        sfxtype2 = *COLLISION_SOUND_ATTR_ELEC;
                        sfxtype3 = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype1 = *COLLISION_SOUND_ATTR_FIRE;
                        sfxtype2 = *COLLISION_SOUND_ATTR_FIRE;
                        sfxtype3 = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,3,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=19)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=60, KBG=30, FKB=25, BKB=30, Size=4.5, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype1, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=170, KBG=kbg1, FKB=10, BKB=0, Size=4.5, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype1, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=0, KBG=kbg1, FKB=10, BKB=0, Size=4.5, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype1, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=180, KBG=kbg1, FKB=10, BKB=0, Size=4.5, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype1, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=24)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=15, KBG=kbg1, FKB=80, BKB=0, Size=2.5, X=0.0, Y=9.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype2, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=15, KBG=kbg1, FKB=50, BKB=0, Size=5.2, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype2, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=170, KBG=kbg1, FKB=10, BKB=0, Size=5.2, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype2, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
                if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=28)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg2, Angle=361, KBG=kbg2, FKB=0, BKB=bkb, Size=5.5, X=0.0, Y=9.0, Z=10.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype3, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmg2, Angle=361, KBG=kbg2, FKB=0, BKB=bkb, Size=5.5, X=0.0, Y=9.0, Z=16.0, X2=0.0, Y2=9.0, Z2=6.5, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype3, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=76)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            frame(Frame=86)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=91)
            FT_MOTION_RATE(FSM=0.5)
            frame(Frame=93)
            FT_MOTION_RATE(FSM=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=70, FKB=10, BKB=10, Size=5.0, X=0.0, Y=8.5, Z=13.0, X2=0.0, Y2=8.5, Z2=14.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_SMASH_02, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=70, FKB=10, BKB=0, Size=5.0, X=0.0, Y=8.5, Z=6.0, X2=0.0, Y2=8.5, Z2=7.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_SMASH_02, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
                AttackModule::set_add_reaction_frame(0,3.0,false)
            }
            frame(Frame=96)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=99)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=70, FKB=10, BKB=10, Size=5.0, X=0.0, Y=8.5, Z=14.5, X2=0.0, Y2=8.5, Z2=15.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_SMASH_03, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=70, FKB=10, BKB=0, Size=5.0, X=0.0, Y=8.5, Z=7.0, X2=0.0, Y2=8.5, Z2=8.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_SMASH_03, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
                AttackModule::set_add_reaction_frame(0,3.0,false)
            }
            frame(Frame=102)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=105)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=40, KBG=85, FKB=0, BKB=40, Size=7.0, X=0.0, Y=8.5, Z=16.0, X2=0.0, Y2=8.5, Z2=17.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=40, KBG=85, FKB=0, BKB=40, Size=7.0, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=9.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            }
            frame(Frame=108)
            if(is_excute){
                AttackModule::clear_all()
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_hi4",
    animcmd = "effect_attackhi4")]
pub fn effect_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            if(is_excute){
                MotionModule::set_frame(74.0,false)
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),0,11,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=87)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(13.000002)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=89)
            if(is_excute){
                LANDING_EFFECT(hash40("sys_v_smoke_a"),hash40("top"),4,0,0,0,0,0,1,0,0,0,0,0,0,false)
            }
            frame(Frame=93)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
            frame(Frame=98)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),0,11,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=19)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=27)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_hi4_hold",
    animcmd = "game_attackhi4charge")]
pub fn attack_hi4_hold(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            if(UP_SMASH_HOLD[entry_id(module_accessor)] == false){
                MotionModule::set_frame(61.0,false)
                rust {
                    UP_SMASH_HOLD[entry_id(module_accessor)] = true;
                }
            }
            if(MotionModule::frame(module_accessor) >= 122.0) {
                MotionModule::set_frame(122.0,false)
            }
        }
        else {
            if(MotionModule::frame(module_accessor) >= 59.0) {
                MotionModule::set_frame(59.0,false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")] //done //invert
pub fn attack_hi4(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 13.0;
            let mut dmg2 = 12.0;
            let mut dmg3 = 8.0;
            let mut kbg1 = 105;
            let mut kbg2 = 96;
            let mut kbg3 = 100;
            let mut bkb1 = 40;
            let mut bkb2 = 32;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 14.0;
                        dmg2 = 13.0;
                        dmg3 = 9.0;
                        kbg1 = 70;
                        kbg2 = 63;
                        kbg3 = 65;
                        bkb1 = 25;
                        bkb2 = 20;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 10.0;
                        dmg2 = 9.0;
                        dmg3 = 5.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 13.5;
                        dmg2 = 12.5;
                        dmg3 = 8.5;
                        kbg1 = 93;
                        kbg2 = 90;
                        kbg3 = 87;
                        bkb1 = 34;
                        bkb2 = 28;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            FT_MOTION_RATE(FSM=0.83)
            frame(Frame=81)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            FT_MOTION_RATE(FSM=0.738)
            frame(Frame=88)
            FT_MOTION_RATE(FSM=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=83, KBG=kbg2, FKB=0, BKB=bkb1, Size=3.3, X=0.0, Y=2.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=dmg2, Angle=83, KBG=kbg2, FKB=0, BKB=bkb1, Size=3.5, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=8.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=83, KBG=kbg1, FKB=0, BKB=bkb1, Size=4.0, X=0.0, Y=9.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=83, KBG=kbg1, FKB=0, BKB=bkb1, Size=4.0, X=0.0, Y=13.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=91)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg3, Angle=60, KBG=kbg3, FKB=0, BKB=bkb2, Size=3.0, X=0.0, Y=2.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg3, Angle=60, KBG=kbg3, FKB=0, BKB=bkb2, Size=3.0, X=0.0, Y=7.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg3, Angle=60, KBG=kbg3, FKB=0, BKB=bkb2, Size=3.0, X=0.0, Y=11.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                AttackModule::clear(ID=3,false)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
        else {
            if(WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK)) {
                MotionModule::set_rate(1.25)
            }
            else {
                MotionModule::set_rate(1.1)
            }
            frame(Frame=10)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=20)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=21.0, Angle=85, KBG=75, FKB=0, BKB=20, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=21.0, Angle=78, KBG=75, FKB=0, BKB=20, Size=4.2, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=21.0, Angle=75, KBG=85, FKB=0, BKB=20, Size=3.9, X=2.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=6)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=70)
            if(is_excute){
                StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_WAIT,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_lw4",
    animcmd = "effect_attacklw4")]
pub fn effect_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),0,11,0,0,0,0,1,0,0,0,0,0,0,true)
            }   
            frame(Frame=20)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(20.000004)
                LAST_EFFECT_SET_RATE(20.000004)
            }
            frame(Frame=21)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=23)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
        else {
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),0,11,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=15)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(15)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 9.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -9.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 18.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -18.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.2, true)
            }
            frame(Frame=22)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
            frame(Frame=30)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(30)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 180, /*Size?*/ 1.2, true)
                EFFECT_FOLLOW(/*Effect*/ hash40("sys_counter_flash"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -20.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 180, /*Size?*/ 1.2, true)
            }
            frame(Frame=37)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")] //done //invert
pub fn attack_lw4(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 11.0;
            let mut kbg = 92;
            let mut bkb = 35;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 13.0;
                        kbg = 65;
                        bkb = 27;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 9.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 12.0;
                        kbg = 86;
                        bkb = 30;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,3,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=3)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=170, KBG=97, FKB=90, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=8.0, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=155, KBG=97, FKB=85, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=13.5, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=160, KBG=97, FKB=90, BKB=0, Size=3.5, X=0.0, Y=4.0, Z=8.0, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=150, KBG=97, FKB=85, BKB=0, Size=3.5, X=0.0, Y=4.0, Z=13.5, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=21)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=35, KBG=kbg, FKB=0, BKB=bkb, Size=4.4, X=0.0, Y=4.8, Z=-14.0, X2=0.0, Y2=4.8, Z2=-1.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
            }
            wait(Frames=3)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=15)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=105, KBG=40, FKB=70, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=9.0, X2=0.0, Y2=7.0, Z2=4.5, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.5, Angle=105, KBG=40, FKB=70, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=18.0, X2=0.0, Y2=7.0, Z2=9.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.5, Angle=15, KBG=40, FKB=70, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=-9.0, X2=0.0, Y2=7.0, Z2=4.5, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.5, Angle=15, KBG=40, FKB=70, BKB=0, Size=3.0, X=0.0, Y=7.0, Z=-18.0, X2=0.0, Y2=7.0, Z2=9.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,5.0,false)
                AttackModule::set_add_reaction_frame(1,5.0,false)
                AttackModule::set_add_reaction_frame(2,5.0,false)
                AttackModule::set_add_reaction_frame(3,5.0,false)
            }
            wait(Frames=7)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=30)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.5, Angle=130, KBG=80, FKB=0, BKB=45, Size=5.0, X=0.0, Y=10.0, Z=18.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.5, Angle=50, KBG=80, FKB=0, BKB=45, Size=5.0, X=0.0, Y=10.0, Z=-18.0, X2=0.0, Y2=10.0, Z2=-9.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                rust {
                    DOWN_SMASH[entry_id(module_accessor)] = true;
                }
            }
            wait(Frames=7)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_n",
    animcmd = "effect_attackairn")]
pub fn effect_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(5.000001)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=18)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
            }
            frame(Frame=19)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=8)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
            frame(Frame=14)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=21)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")] //done
pub fn attack_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 8.0;
            let mut kbg = 100;
            let mut bkb = 25;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 10.0;
                        kbg = 70;
                        bkb = 18;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 6.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 9.0;
                        kbg = 85;
                        bkb = 21;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=2)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=5)
            FT_MOTION_RATE(FSM=0.75)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=40, KBG=kbg, FKB=0, BKB=bkb, Size=3.8, X=3.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=40, KBG=kbg, FKB=0, BKB=bkb, Size=3.8, X=3.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg, Angle=40, KBG=kbg, FKB=0, BKB=bkb, Size=3.8, X=3.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=19)
            if(is_excute){
                AttackModule::clear_all()
            }
            FT_MOTION_RATE(FSM=1)
            frame(Frame=34)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
        else {
            if(WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK)) {
                MotionModule::set_rate(1.1)
            }
            else {
                MotionModule::set_rate(0.8)
            }
            frame(Frame=2)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=6)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=80, KBG=40, FKB=0, BKB=45, Size=5.0, X=0.0, Y=-1.5, Z=5.0, X2=0.0, Y2=-3.7, Z2=3.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=80, KBG=40, FKB=0, BKB=45, Size=5.0, X=0.0, Y=-1.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=80, KBG=40, FKB=0, BKB=45, Size=5.0, X=0.0, Y=-1.5, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,2.0,false)
                AttackModule::set_add_reaction_frame(1,2.0,false)
                AttackModule::set_add_reaction_frame(2,2.0,false)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=50, Size=6.0, X=0.0, Y=-2.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=50, Size=6.0, X=2.5, Y=-2.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=7.0, Angle=50, KBG=95, FKB=0, BKB=50, Size=6.0, X=2.5, Y=-2.5, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=7)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=47)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_f",
    animcmd = "effect_attackairf")]
pub fn effect_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                EFFECT(hash40("sys_smash_flash"),hash40("haver"),0,0,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=16)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(16.000004)
            }
            frame(Frame=17)
            if(is_excute){
                EFFECT_FOLLOW(71718119863 as u64,hash40("top"),0,0,0,0,0,0,1,true)
            }
            frame(Frame=25)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=9)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=23)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")] //done
pub fn attack_air_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 14.0;
            let mut dmg2 = 13.0;
            let mut dmg3 = 11.0;
            let mut kbg1 = 90;
            let mut kbg2 = 100;
            let mut kbg3 = 83;
            let mut bkb1 = 20;
            let mut bkb2 = 30;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 15.0;
                        dmg2 = 14.0;
                        dmg3 = 12.0;
                        kbg1 = 60;
                        kbg2 = 70;
                        kbg3 = 50;
                        bkb1 = 10;
                        bkb2 = 15;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 11.0;
                        dmg2 = 10.0;
                        dmg3 = 8.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 14.5;
                        dmg2 = 13.5;
                        dmg3 = 11.5;
                        kbg1 = 75;
                        kbg2 = 85;
                        kbg3 = 65;
                        bkb1 = 15;
                        bkb2 = 20;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,3,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=3)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=18)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=270, KBG=kbg1, FKB=0, BKB=bkb1, Size=3.3, X=0.0, Y=7.5, Z=14.5, X2=0.0, Y2=6.5, Z2=14.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=60, KBG=kbg2, FKB=0, BKB=bkb1, Size=3.3, X=0.0, Y=7.5, Z=14.5, X2=0.0, Y2=6.5, Z2=14.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg2, Angle=45, KBG=kbg3, FKB=0, BKB=bkb2, Size=7.8, X=0.0, Y=9.5, Z=14.0, X2=0.0, Y2=14.5, Z2=14.6, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=45, KBG=kbg3, FKB=0, BKB=bkb2, Size=4.0, X=0.0, Y=12.0, Z=-2.0, X2=0.0, Y2=4.0, Z2=-2.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear(ID=0,false)
                AttackModule::clear(ID=1,false)
                AttackModule::clear(ID=2,false)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=dmg3, Angle=45, KBG=kbg3, FKB=0, BKB=bkb2, Size=3.0, X=0.0, Y=12.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=26)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=43)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=77)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
        }
        else {
            if(WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK)) {
                MotionModule::set_rate(1.25)
            }
            else {
                MotionModule::set_rate(1.1)
            }
            frame(Frame=3)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=10)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=270, KBG=100, FKB=0, BKB=35, Size=3.9, X=0.0, Y=15.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=270, KBG=100, FKB=0, BKB=35, Size=4.2, X=0.0, Y=9.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=270, KBG=100, FKB=0, BKB=35, Size=4.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=15)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=80, KBG=110, FKB=0, BKB=35, Size=3.9, X=0.0, Y=15.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=80, KBG=110, FKB=0, BKB=35, Size=4.2, X=0.0, Y=9.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=80, KBG=110, FKB=0, BKB=35, Size=4.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=23)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=50)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_b",
    animcmd = "effect_attackairb")]
pub fn effect_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(10.000002)
                AFTER_IMAGE4_ON_arg29(72431755945 as u64, 69865419539 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=14)
            if(is_excute){
                AFTER_IMAGE_OFF(3)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
        else {
            frame(Frame=10)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(10.000002)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=14)
            if(is_excute){
                AFTER_IMAGE_OFF(3)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")] //done
pub fn attack_air_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg = 13.0;
            let mut kbg = 95;
            let mut bkb = 20;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg = 14.5;
                        kbg = 65;
                        bkb = 10;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg = 10.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg = 14.0;
                        kbg = 78;
                        bkb = 15;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=5)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=7.0, X=0.0, Y=8.5, Z=-15.0, X2=0.0, Y2=8.5, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=7.0, X=0.0, Y=16.0, Z=-15.0, X2=0.0, Y2=16.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=37)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=95, FKB=0, BKB=20, Size=7.0, X=0.0, Y=8.5, Z=-15.0, X2=0.0, Y2=8.5, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=95, FKB=0, BKB=20, Size=7.0, X=0.0, Y=16.0, Z=-15.0, X2=0.0, Y2=16.0, Z2=-7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=37)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_hi",
    animcmd = "effect_attackairhi")]
pub fn effect_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                AFTER_IMAGE4_ON_arg29(69579998085 as u64, 71624955430 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=7)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(7.000001)
            }
            frame(Frame=8)
            if(is_excute){
                AFTER_IMAGE_OFF(3)
            }
            frame(Frame=24)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=3)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=12)
            if(is_excute){
                AFTER_IMAGE_OFF(3)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")] //done
pub fn attack_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 11.0;
            let mut dmg2 = 8.0;
            let mut kbg = 80;
            let mut bkb = 42;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 12.0;
                        dmg2 = 10.0;
                        kbg = 50;
                        bkb = 30;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 9.0;
                        dmg2 = 6.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 11.5;
                        dmg2 = 9.0;
                        kbg = 63;
                        bkb = 37;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,2,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=2)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.2, X=0.0, Y=24.0, Z=0.0, X2=0.0, Y2=17.5, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.2, X=0.0, Y=25.0, Z=4.5, X2=0.0, Y2=17.5, Z2=4.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.2, X=0.0, Y=26.0, Z=9.0, X2=0.0, Y2=17.5, Z2=9.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.0, X=0.0, Y=4.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.0, X=0.0, Y=9.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=4.0, X=0.0, Y=14.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=3.0, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=3.0, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=80, KBG=kbg, FKB=0, BKB=bkb, Size=3.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=14)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=32)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
        else {
            frame(Frame=3)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=6.0, Angle=80, KBG=90, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=80, KBG=90, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=80, KBG=90, FKB=0, BKB=40, Size=4.2, X=0.0, Y=1.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=80, KBG=90, FKB=0, BKB=40, Size=3.9, X=0.0, Y=3.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,5.0,false)
                AttackModule::set_add_reaction_frame(1,5.0,false)
                AttackModule::set_add_reaction_frame(2,5.0,false)
                AttackModule::set_add_reaction_frame(3,5.0,false)
            }
            frame(Frame=18)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=38)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_lw",
    animcmd = "effect_attackairlw")]
pub fn effect_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=2)
            if(is_excute){
                EFFECT(67837198872 as u64,hash40("top"),0,0,0,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=10)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(10.000002)
            }
            frame(Frame=36)
            if(is_excute){
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)
            }
            frame(Frame=42)
            if(is_excute){
                AFTER_IMAGE_OFF(4)
                EFFECT_OFF_KIND(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")] //done 
pub fn attack_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            let mut dmg1 = 15.0;
            let mut dmg2 = 13.0;
            let mut dmg3 = 8.0;
            let mut kbg1 = 100;
            let mut kbg2 = 105;
            let mut kbg3 = 55;
            let mut bkb1 = 10;
            let mut bkb2 = 30;
            let mut bkb3 = 80;
            let mut effect = smash::hash40("collision_attr_cutup");
            let mut sfxtype = *COLLISION_SOUND_ATTR_CLOUD_HIT;
            if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                if color_id(module_accessor) {
                    if FIRE[entry_id(module_accessor)] {
                        dmg1 = 16.0;
                        dmg2 = 14.0;
                        dmg3 = 10.0;
                        kbg1 = 70;
                        kbg2 = 73;
                        kbg3 = 30;
                        bkb1 = 4;
                        bkb2 = 20;
                        bkb3 = 50;
                        effect = smash::hash40("collision_attr_fire");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                    }
                    else if ICE[entry_id(module_accessor)] {
                        dmg1 = 12.5;
                        dmg2 = 11.0;
                        dmg3 = 9.0;
                        effect = smash::hash40("collision_attr_ice");
                        sfxtype = *COLLISION_SOUND_ATTR_FREEZE;
                    }
                    else if ELEC[entry_id(module_accessor)] {
                        dmg1 = 15.5;
                        dmg2 = 13.5;
                        dmg3 = 9.0;
                        kbg1 = 85;
                        kbg2 = 88;
                        kbg3 = 40;
                        bkb1 = 5;
                        bkb2 = 23;
                        bkb3 = 60;
                        effect = smash::hash40("collision_attr_elec");
                        sfxtype = *COLLISION_SOUND_ATTR_ELEC;
                    }
                    else if POISON[entry_id(module_accessor)] {
                        effect = hash40("collision_attr_curse_poison");
                        sfxtype = *COLLISION_SOUND_ATTR_FIRE;
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 100);
                        if rng % 2 == 0 {
                            AttackModule::set_poison_param(module_accessor,0,361,45,1.0,false);
                            AttackModule::set_poison_param(module_accessor,1,361,45,1.0,false);
                        }
                    }
                }
            }
        }
        if(cloud_id(module_accessor)) {
            frame(Frame=5)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=dmg1, Angle=270, KBG=kbg1, FKB=0, BKB=bkb1, Size=3.6, X=0.0, Y=11.0, Z=0.0, X2=0.0, Y2=12.5, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg2, Angle=60, KBG=kbg2, FKB=0, BKB=bkb2, Size=4.5, X=0.0, Y=7.0, Z=0.0, X2=0.0, Y2=13.0, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=3)
            if(is_excute){
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=dmg3, Angle=65, KBG=kbg3, FKB=0, BKB=bkb3, Size=3.5, X=0.0, Y=9.0, Z=0.0, X2=0.0, Y2=11.0, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sfxtype, Type=ATTACK_REGION_SWORD)
                AttackModule::clear(ID=0,false)
            }
            wait(Frames=26)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=46)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
        else {
            frame(Frame=2)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                rust {
                    DAIR[entry_id(module_accessor)] = true;
                }
            }
            frame(Frame=5)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=1.5, Angle=367, KBG=100, FKB=20, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=1.5, Angle=367, KBG=100, FKB=20, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=1.5, Angle=367, KBG=100, FKB=20, BKB=20, Size=4.0, X=8.0, Y=6.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=1.5, Angle=367, KBG=100, FKB=20, BKB=20, Size=4.2, X=0.0, Y=-6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)  
            }
            frame(Frame=25)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=26)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=40, KBG=0, FKB=70, BKB=70, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=40, KBG=0, FKB=70, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=2.5, Angle=40, KBG=0, FKB=70, BKB=70, Size=4.0, X=8.0, Y=6.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=40, KBG=0, FKB=70, BKB=70, Size=4.2, X=0.0, Y=-6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=16)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=60)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "catch",
    animcmd = "game_catch")] //done
pub fn catch(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=9)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.6, Z=4.0, X2=0.0, Y2=6.6, Z2=8.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.6, Z=2.35, X2=0.0, Y2=6.6, Z2=10.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
        else {
            frame(Frame=5)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=6)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=7.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=8.0, Z=2.35, X2=0.0, Y2=8.0, Z2=9.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "catch_dash",
    animcmd = "game_catchdash")] //done
pub fn catch_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=11)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=12)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=2.6, X=0.0, Y=6.6, Z=4.0, X2=0.0, Y2=6.6, Z2=10.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.3, X=0.0, Y=6.6, Z=2.7, X2=0.0, Y2=6.6, Z2=11.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
        else {
            frame(Frame=8)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=9)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=2.6, X=0.0, Y=7.0, Z=4.0, X2=0.0, Y2=7.0, Z2=9.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.3, X=0.0, Y=7.0, Z=2.7, X2=0.0, Y2=7.0, Z2=10.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "catch_turn",
    animcmd = "game_catchturn")] //done
pub fn catch_turn(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=12)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=13)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.6, Z=-4.0, X2=0.0, Y2=6.6, Z2=-14.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.6, Z=-2.35, X2=0.0, Y2=6.6, Z2=-16.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
        else {
            frame(Frame=9)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=10)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=7.0, Z=-4.0, X2=0.0, Y2=7.0, Z2=-13.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=7.0, Z=-2.35, X2=0.0, Y2=7.0, Z2=-14.85, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            game_CaptureCutCommon()
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "catch_attack",
    animcmd = "game_catchattack")] //done
pub fn catch_attack(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
                AttackModule::set_catch_only_all(true, false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=1)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=4.8, X=0.0, Y=7.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                AttackModule::set_catch_only_all(true, false)
            }
            wait(Frames=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "throw_hi",
    animcmd = "game_throwhi")] //done
pub fn throw_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            LookupSymbol(
                &mut FIGHTER_CUTIN_MANAGER_ADDR,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                    .as_bytes()
                    .as_ptr(),
            );
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        }
        if(cloud_id(module_accessor)) {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=80, KBG=97, FKB=0, BKB=78, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=50, KBG=150, FKB=0, BKB=80, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_catch_only_all(true, false)
            }
            frame(Frame=10)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=13)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.5, Angle=50, KBG=150, FKB=0, BKB=80, Size=4.0, X=0.0, Y=21.5, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_catch_only_all(true, false)
                CHECK_FINISH_CAMERA(9, 18)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.3);
                }
            }
            frame(Frame=15)
            if(is_excute){
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                AttackModule::clear_all()
                CancelModule::enable_cancel()
            }
        }
        else {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.0, Angle=90, KBG=230, FKB=0, BKB=24, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=26)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=20, KBG=70, FKB=0, BKB=30, Size=3.8, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=20, KBG=70, FKB=0, BKB=30, Size=3.8, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_catch_only_all(true, false)
                CHECK_FINISH_CAMERA(6, 28)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.8);
                    let vec3f = smash::phx::Vector3f {x: 0.0,y: 12.0,z: 0.0};
                    FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager,vec3f);
                }
            }
            frame(Frame=28)
            if(is_excute){
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "throw_f",
    animcmd = "game_throwf")] //done
pub fn throw_f(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            LookupSymbol(
                &mut FIGHTER_CUTIN_MANAGER_ADDR,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                    .as_bytes()
                    .as_ptr(),
            );
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        }
        if(cloud_id(module_accessor)) {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=49, KBG=125, FKB=0, BKB=48, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=7)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=4.0, Angle=45, KBG=120, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_catch_only_all(true, false)
                CHECK_FINISH_CAMERA(15, 4)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.3);
                }
            }
            frame(Frame=9)
            if(is_excute){
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                AttackModule::clear_all()
            }
        }
        else {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=49, KBG=110, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=11)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=30, KBG=120, FKB=0, BKB=80, Size=6.5, X=0.0, Y=8.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_catch_only_all(true, false)
                CHECK_FINISH_CAMERA(0, 15)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.2);
                    let vec3f = smash::phx::Vector3f {x: 10.0,y: 1.0,z: 0.0};
                    FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager,vec3f);
                }
            }
            frame(Frame=13)
            if(is_excute){
                AttackModule::clear_all()
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "throw_b",
    animcmd = "game_throwb")] //done
pub fn throw_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            LookupSymbol(
                &mut FIGHTER_CUTIN_MANAGER_ADDR,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                    .as_bytes()
                    .as_ptr(),
            );
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        }
        if(cloud_id(module_accessor)) {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=38, KBG=110, FKB=0, BKB=48, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=13)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=3.0, Angle=45, KBG=90, FKB=0, BKB=48, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_catch_only_all(true, false)
                WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT)
                CHECK_FINISH_CAMERA(13, 8)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.3);
                }
            }
            frame(Frame=15)
            if(is_excute){
                AttackModule::clear_all()
                REVERSE_LR()
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
        }
        else {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=131, KBG=80, FKB=0, BKB=80, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=13)
            if(is_excute){
                CHECK_FINISH_CAMERA(-7, 0)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.2);
                    let vec3f = smash::phx::Vector3f {x: -2.0,y: 0.0,z: 0.0};
                    FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager,vec3f);
                }
            }
            frame(Frame=9)
            if(is_excute){
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "throw_lw",
    animcmd = "game_throwlw")] //done
pub fn throw_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            LookupSymbol(
                &mut FIGHTER_CUTIN_MANAGER_ADDR,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                    .as_bytes()
                    .as_ptr(),
            );
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        }
        if(cloud_id(module_accessor)) {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=122, KBG=110, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=17)
            if(is_excute){
                CHECK_FINISH_CAMERA(0, 0)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.3);
                }
            }
            frame(Frame=18)
            if(is_excute){
                AttackModule::clear_all()
                REVERSE_LR()
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
        }
        else {
            if(is_excute){
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.5, Angle=85, KBG=105, FKB=0, BKB=85, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=23)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=85, KBG=50, FKB=0, BKB=20, Size=5.0, X=0.0, Y=4.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_catch_only_all(true, false)
            }
            frame(Frame=24)
            if(is_excute){
                CHECK_FINISH_CAMERA(4, 0)
                rust {
                    FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager,1.0);
                    let vec3f = smash::phx::Vector3f {x: 0.0,y: -2.0,z: 0.0};
                    FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager,vec3f);
                }
            }
            frame(Frame=25)
            if(is_excute){
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
                WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_n",
    animcmd = "effect_specialn")]
pub fn effect_special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=12)
            if(is_excute){
                EFFECT(92518585121 as u64,hash40("top"),12.000006,0,9,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=17)
            if(is_excute){
                LANDING_EFFECT(0x0d0679b24d as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=23)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=20)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=85)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_n",
    animcmd = "game_specialn")] //done
pub fn special_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
            frame(Frame=16)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=5.5, X2=0.0, Y2=6.5, Z2=4.5, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
                ArticleModule::generate_article(FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,0)
            }
            frame(Frame=22)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
            FT_MOTION_RATE(FSM=0.8)
        }
        else {
            MotionModule::set_rate(1.2)
            CaptureModule::set_ignore_catching(true)
            frame(Frame=23)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=4.1, X=0.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,25.0,false)
                AttackModule::set_add_reaction_frame(1,25.0,false)
                AttackModule::set_add_reaction_frame(2,25.0,false)
                rust{
                    SPECIAL_N[entry_id(module_accessor)] = true;
                }
            }
            frame(Frame=53)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=55)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=4.1, X=0.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=85)
            if(is_excute){
                AttackModule::clear_all()
                CaptureModule::set_ignore_catching(false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_n",
    animcmd = "effect_specialairn")]
pub fn effect_special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=23)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
        else {
            frame(Frame=20)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=85)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_n",
    animcmd = "game_specialairn")] //done
pub fn special_air_n(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
            frame(Frame=16)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=5.5, X2=0.0, Y2=6.5, Z2=4.5, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
                ArticleModule::generate_article(FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,0)
            }
            frame(Frame=22)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
        }
        else {
            MotionModule::set_rate(1.2)
            CaptureModule::set_ignore_catching(true)
            rust{
                let fall_speed  = smash::phx::Vector3f { x: 0.6, y: 0.6, z: 0.6 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            frame(Frame=23)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,25.0,false)
                AttackModule::set_add_reaction_frame(1,25.0,false)
                AttackModule::set_add_reaction_frame(2,25.0,false)
                rust{
                    SPECIAL_N[entry_id(module_accessor)] = true;
                }
            }
            frame(Frame=53)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=55)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=10.0, Angle=120, KBG=100, FKB=0, BKB=40, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=85)
            if(is_excute){
                AttackModule::clear_all()
                CaptureModule::set_ignore_catching(false)
                rust {
                    let fall_speed  = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
                    KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_n_lb",
    animcmd = "effect_specialn_lb")]
pub fn effect_special_n_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=12)
            if(is_excute){
                EFFECT(92518585121 as u64,hash40("top"),12.000006,0,9,0,0,0,1,0,0,0,0,0,0,true)
            }
            frame(Frame=17)
            if(is_excute){
                LANDING_EFFECT(78748329283 as u64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
            }
            frame(Frame=23)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
        else {
            frame(Frame=20)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=85)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_n_lb",
    animcmd = "game_specialn_lb")] //done
pub fn special_n_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
            frame(Frame=16)
            if(is_excute){
                ArticleModule::generate_article(FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,0)
            }
            frame(Frame=18)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6)
            }
            frame(Frame=22)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
        }
        else {
            MotionModule::set_rate(1.3)
            CaptureModule::set_ignore_catching(true)
            frame(Frame=23)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,25.0,false)
                AttackModule::set_add_reaction_frame(1,25.0,false)
                AttackModule::set_add_reaction_frame(2,25.0,false)
                rust{
                    SPECIAL_N[entry_id(module_accessor)] = true;
                }
            }
            frame(Frame=53)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=55)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=110, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=110, FKB=0, BKB=55, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=110, FKB=0, BKB=55, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=85)
            if(is_excute){
                AttackModule::clear_all()
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
                CaptureModule::set_ignore_catching(false)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_n_lb",
    animcmd = "effect_specialairn_lb")]
pub fn effect_special_air_n_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=8)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=23)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
        else {
            frame(Frame=20)
            if(is_excute){
                EFFECT_FOLLOW_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),hash40("haver"),0,0,0,0,0,0,1,true)
                LAST_EFFECT_SET_COLOR(3.0, 1.5, 0.0)
                LAST_EFFECT_SET_ALPHA(8.000002)
            }
            frame(Frame=85)
            if(is_excute){
                EFFECT_OFF_KIND_WORK(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB),false,true)
            }
        }
    });
}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_n_lb",
    animcmd = "game_specialairn_lb")] //done
pub fn special_air_n_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=10)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
            frame(Frame=16)
            if(is_excute){
                ArticleModule::generate_article(FIGHTER_CLOUD_GENERATE_ARTICLE_WAVE,false,0)
            }
            frame(Frame=18)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6)
            }
            frame(Frame=22)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL)
            }
        }
        else {
            MotionModule::set_rate(1.3)
            CaptureModule::set_ignore_catching(true)
            rust{
                let fall_speed  = smash::phx::Vector3f { x: 0.6, y: 0.6, z: 0.6 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            frame(Frame=23)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=75, KBG=70, FKB=20, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_slip"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(0,25.0,false)
                AttackModule::set_add_reaction_frame(1,25.0,false)
                AttackModule::set_add_reaction_frame(2,25.0,false)
                rust{
                    SPECIAL_N[entry_id(module_accessor)] = true;
                }
            }
            frame(Frame=53)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=55)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=120, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=120, FKB=0, BKB=55, Size=4.7, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=150, KBG=120, FKB=0, BKB=55, Size=4.4, X=0.0, Y=0.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=85)
            if(is_excute){
                AttackModule::clear_all()
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
                CaptureModule::set_ignore_catching(false)
                rust {
                    let fall_speed  = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
                    KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_hi",
    animcmd = "game_specialhi")] //done
pub fn special_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
                AttackModule::clear_all()
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=94, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=96, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=40, FKB=0, BKB=98, Size=3.5, X=0.0, Y=14.5, Z=-10.0, X2=0.0, Y2=6.0, Z2=-10.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=12)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=85, KBG=50, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=96, KBG=50, FKB=0, BKB=50, Size=3.5, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=40, FKB=0, BKB=70, Size=3.5, X=0.0, Y=14.5, Z=-12.0, X2=0.0, Y2=6.0, Z2=-12.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }   
            frame(Frame=16)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            frame(Frame=20)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=23)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
            frame(Frame=25)
            FT_MOTION_RATE(FSM=0.8)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=27)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
        }
        else {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
                AttackModule::clear_all()
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=94, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=96, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=90, KBG=40, FKB=0, BKB=98, Size=3.5, X=0.0, Y=14.5, Z=-10.0, X2=0.0, Y2=6.0, Z2=-10.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=16)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            frame(Frame=20)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=23)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
            frame(Frame=25)
            FT_MOTION_RATE(FSM=0.8)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=27)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                if(ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)) {
                    StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_FALL,true)
                    rust {
                        SPECIAL_HI[entry_id(module_accessor)] = true;
                    }
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")] //done
pub fn special_air_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
                AttackModule::clear_all()
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=94, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=96, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=40, FKB=0, BKB=98, Size=3.5, X=0.0, Y=14.5, Z=-10.0, X2=0.0, Y2=6.0, Z2=-10.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=12)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=85, KBG=50, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=96, KBG=50, FKB=0, BKB=50, Size=3.5, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=40, FKB=0, BKB=70, Size=3.5, X=0.0, Y=14.5, Z=-12.0, X2=0.0, Y2=6.0, Z2=-12.0, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }   
            frame(Frame=16)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            frame(Frame=20)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=23)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
            frame(Frame=25)
            FT_MOTION_RATE(FSM=0.8)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=27)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
        }
        else {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
                AttackModule::clear_all()
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=94, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=96, KBG=100, FKB=121, BKB=0, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("haver"), Damage=8.0, Angle=90, KBG=40, FKB=0, BKB=98, Size=3.5, X=0.0, Y=14.5, Z=-10.0, X2=0.0, Y2=6.0, Z2=-10.0, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=16)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            frame(Frame=20)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=23)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
            frame(Frame=25)
            FT_MOTION_RATE(FSM=0.8)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=27)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                if(ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)) {
                    StatusModule::change_status_request_from_script(FIGHTER_STATUS_KIND_FALL,true)
                    rust {
                        SPECIAL_HI[entry_id(module_accessor)] = true;
                    }
                }
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_hi2",
    animcmd = "game_specialhi2")] //done
pub fn special_hi2(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE)
            }
            frame(Frame=12)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=262, KBG=100, FKB=110, BKB=0, Size=2.0, X=0.0, Y=2.0, Z=10.4, X2=0.0, Y2=2.0, Z2=-1.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=45, KBG=80, FKB=0, BKB=50, Size=4.0, X=0.0, Y=4.6, Z=9.4, X2=0.0, Y2=4.2, Z2=-1.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        else {
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_NONE)
            }
            frame(Frame=12)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=270, KBG=100, FKB=110, BKB=0, Size=2.0, X=0.0, Y=2.0, Z=10.4, X2=0.0, Y2=2.0, Z2=-1.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=270, KBG=80, FKB=0, BKB=50, Size=4.0, X=0.0, Y=4.6, Z=9.4, X2=0.0, Y2=4.2, Z2=-1.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_hi_lb",
    animcmd = "game_specialhi_lb")] //done
pub fn special_hi_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=11)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
            }
            FT_MOTION_RATE(FSM=2)
            frame(Frame=14)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            FT_MOTION_RATE(FSM=1)
            frame(Frame=25)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=29)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
            }
            frame(Frame=36)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=40)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
        }
        else {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=11)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
            }
            FT_MOTION_RATE(FSM=2)
            frame(Frame=14)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            FT_MOTION_RATE(FSM=1)
            frame(Frame=25)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=29)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
            }
            frame(Frame=36)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=40)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_hi_lb",
    animcmd = "game_specialairhi_lb")] //done
pub fn special_air_hi_lb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=11)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
            }
            FT_MOTION_RATE(FSM=2)
            frame(Frame=14)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            FT_MOTION_RATE(FSM=1)
            frame(Frame=25)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=29)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
            }
            frame(Frame=36)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=40)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
        }
        else {
            frame(Frame=6)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
            }
            frame(Frame=7)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=40, KBG=100, FKB=80, BKB=0, Size=2.0, X=0.0, Y=7.0, Z=3.0, X2=0.0, Y2=7.0, Z2=3.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=78, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=5.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=102, KBG=100, FKB=80, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
            }
            frame(Frame=8)
            if(is_excute){
                SA_SET(State=SITUATION_KIND_AIR)
            }
            frame(Frame=9)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=10)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=1, Bone=hash40("haver"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=6.0, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("haver"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=5.0, X=0.0, Y=14.5, Z=0.0, X2=0.0, Y2=6.0, Z2=0.0, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=10.0, Angle=88, KBG=142, FKB=0, BKB=50, Size=3.5, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=6.5, Hitlag=3.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CLOUD_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=11)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
            }
            FT_MOTION_RATE(FSM=2)
            frame(Frame=14)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=15)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE)
            }
            FT_MOTION_RATE(FSM=1)
            frame(Frame=25)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL)
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL)
            }
            frame(Frame=29)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=35)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
            }
            frame(Frame=36)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT)
            }
            frame(Frame=40)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_lw",
    animcmd = "game_speciallw")] //done
pub fn special_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=1)
            FT_MOTION_RATE(FSM=0.8125)
            frame(Frame=16)
            FT_MOTION_RATE(FSM=1)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=431, FKB=0, BKB=60, Size=10.0, X=0.0, Y=10.0, Z=13.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=17)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=431, FKB=0, BKB=60, Size=8.0, X=0.0, Y=20.0, Z=9.0, X2=0.0, Y2=11.0, Z2=10.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=18)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=406, FKB=0, BKB=60, Size=7.0, X=0.0, Y=22.0, Z=-2.0, X2=0.0, Y2=22.0, Z2=-8.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=19)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=406, FKB=0, BKB=60, Size=8.0, X=0.0, Y=16.0, Z=-11.5, X2=0.0, Y2=10.0, Z2=-11.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=20)
            if(is_excute){
                AttackModule::set_target_category(ID=0, Hitbits=COLLISION_CATEGORY_MASK_NO_IF)
                AttackModule::set_size(ID=0, Size=0.1)
            }
            frame(Frame=27)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=100, FKB=80, BKB=0, Size=13.0, X=0.0, Y=15.0, Z=16.0, X2=0.0, Y2=15.0, Z2=-16.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=32)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=1)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            MotionModule::set_rate(1.5)
            frame(Frame=17)
            MotionModule::set_rate(0.8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(frame=20)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=23)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(frame=26)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=29)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=32)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
            }
            frame(Frame=34)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=80, KBG=70, FKB=0, BKB=100, Size=30.0, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=20.0, Z2=0.0, Hitlag=4.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=40)
            if(is_excute){
                AttackModule::clear_all()
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_lw",
    animcmd = "game_specialairlw")] //done
pub fn special_air_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(cloud_id(module_accessor)) {
            frame(Frame=16)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=371, FKB=0, BKB=60, Size=10.0, X=0.0, Y=10.0, Z=13.0, X2=0.0, Y2=10.0, Z2=9.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=17)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=371, FKB=0, BKB=60, Size=8.0, X=0.0, Y=20.0, Z=9.0, X2=0.0, Y2=11.0, Z2=10.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=18)
            if(is_excute){
                AttackModule::set_target_category(ID=0, Hitbits=COLLISION_CATEGORY_MASK_NO_IF)
                AttackModule::set_size(ID=0, Size=0.1)
            }
            frame(Frame=19)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=80, KBG=349, FKB=0, BKB=60, Size=8.0, X=0.0, Y=16.0, Z=-11.5, X2=0.0, Y2=10.0, Z2=-11.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=20)
            if(is_excute){
                AttackModule::set_target_category(ID=0, Hitbits=COLLISION_CATEGORY_MASK_NO_IF)
                AttackModule::set_size(ID=0, Size=0.1)
            }
            frame(Frame=27)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=86, FKB=80, BKB=0, Size=13.0, X=0.0, Y=15.0, Z=16.0, X2=0.0, Y2=15.0, Z2=-16.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=32)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        else {
            frame(Frame=1)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
            MotionModule::set_rate(1.5)
            frame(Frame=17)
            MotionModule::set_rate(0.8)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(frame=20)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=23)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(frame=26)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=29)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=5.0, X2=0.0, Y2=15.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=70, FKB=100, BKB=0, Size=15.0, X=0.0, Y=15.0, Z=-5.0, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=32)
            if(is_excute){
                AttackModule::clear_all()
                WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
            }
            frame(Frame=34)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=80, KBG=70, FKB=0, BKB=100, Size=30.0, X=0.0, Y=20.0, Z=0.0, X2=0.0, Y2=20.0, Z2=0.0, Hitlag=4.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=40)
            if(is_excute){
                AttackModule::clear_all()
                sv_battle_object::notify_event_msc_cmd(0x25813802b6u64)
                WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_LW_FLAG_LB_SCENE)
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
            }
        }
    });
}

pub fn randomized_limit_sword(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if cloud_id(module_accessor) {
                if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) && color_id(module_accessor) {
                    if SWORD_TAKEN[entry_id(module_accessor)] == false {
                        let rng = smash::app::sv_math::rand(hash40("cloud"), 20);
                        if rng == 1 || rng == 5 || rng == 8 || rng == 16 || rng == 3 {
                            FIRE[entry_id(module_accessor)] = true;
                        }
                        else if rng == 0 || rng == 6 || rng == 12 || rng == 18 {
                            ICE[entry_id(module_accessor)] = true;
                        }
                        else if rng == 7 || rng == 9 || rng == 2 {
                            ELEC[entry_id(module_accessor)] = true;
                        }
                        else if rng == 2 || rng == 4 || rng == 13 {
                            POISON[entry_id(module_accessor)] = true;
                        }
                        SWORD_TAKEN[entry_id(module_accessor)] = true;
                    }
                    if LIMIT_CLEAR[entry_id(module_accessor)] <= 1750 {
                        LIMIT_CLEAR[entry_id(module_accessor)] += 1;
                        if LIMIT_CLEAR[entry_id(module_accessor)] >= 1500 {
                            if LIMIT_CLEAR[entry_id(module_accessor)] % 30 == 0 {
                                acmd!(lua_state, {
                                    EFFECT_FOLLOW(/*Effect*/ hash40("sys_timer"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.8, true)
                                    PLAY_SE(hash40("se_common_landing_water_s"))
                                });
                            }
                        }
                    }
                }
                else {
                    FIRE[entry_id(module_accessor)] = false;
                    ICE[entry_id(module_accessor)] = false;
                    ELEC[entry_id(module_accessor)] = false;
                    POISON[entry_id(module_accessor)] = false;
                    SWORD_TAKEN[entry_id(module_accessor)] = false;
                    LIMIT_CLEAR[entry_id(module_accessor)] = 0;
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
                }
                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    if UP_SMASH[entry_id(module_accessor)] == false {
                        MotionModule::set_frame(module_accessor,74.0,false);
                        UP_SMASH[entry_id(module_accessor)] = true;
                    }
                }
                if UP_SMASH[entry_id(module_accessor)] {
                    UP_SMASH_COOL_DOWN[entry_id(module_accessor)] += 1;
                    if UP_SMASH_COOL_DOWN[entry_id(module_accessor)] == 60 {
                        UP_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                        UP_SMASH[entry_id(module_accessor)] = false;
                        UP_SMASH_HOLD[entry_id(module_accessor)] = false;
                    }
                }
                if StopModule::is_damage(module_accessor) {
                    UP_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                    UP_SMASH[entry_id(module_accessor)] = false;
                    UP_SMASH_HOLD[entry_id(module_accessor)] = false;
                    DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                    DOWN_SMASH[entry_id(module_accessor)] = false;
                }
            }
        }
    }
}

pub fn special_lw_counter(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if cloud_id(module_accessor) == false {
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("cloud_limitgauge"),true,false);
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("cloud_limitgauge_cursor"),true,false);
                if StatusModule::status_kind(module_accessor) == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE {
                    if SPECIAL_LW_COUNTER[entry_id(module_accessor)] == false {
                        SPECIAL_LW_COUNTER_CHECK[entry_id(module_accessor)] = true;
                    }
                }
                if SPECIAL_LW_COUNTER_CHECK[entry_id(module_accessor)] {
                    SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] += 1.0;
                    if StopModule::is_damage(module_accessor) || CaptureModule::is_capture(module_accessor)
                    || StatusModule::status_kind(module_accessor) != *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE {
                        if SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] >= 37.0 {
                            SPECIAL_LW_HIT[entry_id(module_accessor)] = true;
                        }
                        SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] = 65.0;
                    }
                    if SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] >= 7.0 && SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] <= 37.0 {
                        acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0) });
                        DamageModule::set_damage_mul(module_accessor,0.0001);
                        if StopModule::is_damage(module_accessor) {
                            if ItemModule::is_attach_item(module_accessor,smash::app::ItemKind(*ITEM_KIND_RIPSTICKFLOWER)) {
                                ItemModule::eject_attach(module_accessor,smash::app::ItemKind(*ITEM_KIND_RIPSTICKFLOWER),true,false);
                            }
                            else if ItemModule::is_attach_item(module_accessor,smash::app::ItemKind(*ITEM_KIND_WALKMUSHFLOWER)) {
                                ItemModule::eject_attach(module_accessor,smash::app::ItemKind(*ITEM_KIND_WALKMUSHFLOWER),true,false);
                            }
                            SPECIAL_LW_COUNTER_HIT[entry_id(module_accessor)] = true;
                        }
                    }
                    else {
                        DamageModule::set_damage_mul(module_accessor,1.0);
                        acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0) });
                    }
                    if SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] >= 65.0 {
                        if SPECIAL_LW_COUNTER_HIT[entry_id(module_accessor)] == true
                        || SPECIAL_LW_HIT[entry_id(module_accessor)] == false {
                            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_WAIT,true);
                                MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("wait"),0.0,0.0,0.0);
                            }
                            else if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_FALL,true);
                                MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("fall"),0.0,0.0,0.0);
                            }
                        }
                        SPECIAL_LW_COUNTER_TIME[entry_id(module_accessor)] = 0.0;
                        SPECIAL_LW_COUNTER_CHECK[entry_id(module_accessor)] = false;
                    }
                }
                if SPECIAL_LW_COUNTER_HIT[entry_id(module_accessor)] {
                    SPECIAL_LW_HIT_TIME[entry_id(module_accessor)] += 1.0;
                    DamageModule::set_damage_lock(module_accessor,true);
                    acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0) });
                    if SPECIAL_LW_HIT_TIME[entry_id(module_accessor)] == 60.0 {
                        SPECIAL_LW_COUNTER_HIT[entry_id(module_accessor)] = false;
                        SPECIAL_LW_HIT_TIME[entry_id(module_accessor)] = 0.0;
                        DamageModule::set_damage_lock(module_accessor,false);
                        SPECIAL_LW_COUNTER[entry_id(module_accessor)] = false;
                        acmd!(lua_state, { sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0) });
                        WorkModule::add_float(module_accessor,15.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
                    }
                }
                if WorkModule::get_float(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE) <= 99.9 {
                    WorkModule::add_float(module_accessor,0.01,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
                }
                else {
                    WorkModule::add_float(module_accessor,0.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
                }
            }
        }
    }
}

pub fn moves_lag(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if cloud_id(module_accessor) == false {
                if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                    if SPECIAL_N[entry_id(module_accessor)] {
                        SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] == 55 {
                            SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] = 0;
                            SPECIAL_N[entry_id(module_accessor)] = false;
                        }
                    }
                    if DAIR[entry_id(module_accessor)] {
                        DAIR_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if DAIR_COOL_DOWN[entry_id(module_accessor)] == 61
                        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                            DAIR_COOL_DOWN[entry_id(module_accessor)] = 0;
                            DAIR[entry_id(module_accessor)] = false;
                        }
                    }
                    if DOWN_SMASH[entry_id(module_accessor)] {
                        DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] == 48 {
                            DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                            DOWN_SMASH[entry_id(module_accessor)] = false;
                        }
                    }
                    if SIDE_SMASH[entry_id(module_accessor)] {
                        SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] == 74 {
                            SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                            SIDE_SMASH[entry_id(module_accessor)] = false;
                            SIDE_SMASH_HOLD[entry_id(module_accessor)] = false;
                        }
                    }
                }
                else {
                    if SPECIAL_N[entry_id(module_accessor)] {
                        SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] == 60 {
                            SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] = 0;
                            SPECIAL_N[entry_id(module_accessor)] = false;
                        }
                    }
                    if DAIR[entry_id(module_accessor)] {
                        DAIR_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if DAIR_COOL_DOWN[entry_id(module_accessor)] == 61
                        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                            DAIR_COOL_DOWN[entry_id(module_accessor)] = 0;
                            DAIR[entry_id(module_accessor)] = false;
                        }
                    }
                    if DOWN_SMASH[entry_id(module_accessor)] {
                        DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] == 60 {
                            DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                            DOWN_SMASH[entry_id(module_accessor)] = false;
                        }
                    }
                    if SIDE_SMASH[entry_id(module_accessor)] {
                        SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] += 1;
                        if SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] == 74 {
                            SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                            SIDE_SMASH[entry_id(module_accessor)] = false;
                            SIDE_SMASH_HOLD[entry_id(module_accessor)] = false;
                        }
                    }
                }
                if StopModule::is_damage(module_accessor) {
                    SIDE_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                    SIDE_SMASH[entry_id(module_accessor)] = false;
                    DOWN_SMASH_COOL_DOWN[entry_id(module_accessor)] = 0;
                    DOWN_SMASH[entry_id(module_accessor)] = false;
                    DAIR_COOL_DOWN[entry_id(module_accessor)] = 0;
                    DAIR[entry_id(module_accessor)] = false;
                    SPECIAL_N_COOL_DOWN[entry_id(module_accessor)] = 0;
                    SPECIAL_N[entry_id(module_accessor)] = false;
                    SIDE_SMASH_HOLD[entry_id(module_accessor)] = false;
                }
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
                || StopModule::is_damage(module_accessor) {
                    SPECIAL_HI[entry_id(module_accessor)] = false;
                }
                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                    if SIDE_SMASH[entry_id(module_accessor)] == false {
                        MotionModule::set_frame(module_accessor,79.0,false);
                        SIDE_SMASH[entry_id(module_accessor)] = true;
                    }
                }
            }
        }
    }
}

pub fn sora_break(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor); 
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if cloud_id(module_accessor) == false {
                if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                    SORA_BREAK[entry_id(module_accessor)] = true;
                    AttackModule::set_power_mul(module_accessor,1.2);
                    AttackModule::set_reaction_mul(module_accessor,0.8);
                    MotionModule::set_rate(module_accessor,1.25);
                    if LIMIT_CLEAR[entry_id(module_accessor)] <= 1750 {
                        LIMIT_CLEAR[entry_id(module_accessor)] += 1;
                        if LIMIT_CLEAR[entry_id(module_accessor)] >= 1500 {
                            if LIMIT_CLEAR[entry_id(module_accessor)] % 30 == 0 {
                                acmd!(lua_state, {
                                    EFFECT_FOLLOW(/*Effect*/ hash40("sys_timer"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.8, true)
                                    PLAY_SE(hash40("se_common_landing_water_s"))
                                });
                            }
                        }
                    }
                }
                else if SORA_BREAK[entry_id(module_accessor)] {
                    LIMIT_CLEAR[entry_id(module_accessor)] = 0;
                    AttackModule::set_power_mul(module_accessor,1.0);
                    AttackModule::set_reaction_mul(module_accessor,1.0);
                    MotionModule::set_rate(module_accessor,1.0);
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
                    SORA_BREAK[entry_id(module_accessor)] = false;
                }
            }
        }
    }
}

pub fn victory_theme(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        LookupSymbol(
            &mut FIGHTER_MANAGER,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
        );
        let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
        let victor = FighterManager::get_top_rank_player(fighter_manager,0) as usize;
        if entry_id(module_accessor) == victor {
            if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_CLOUD {
                if cloud_id(module_accessor) == false {
                    VICTORY_COLOR_INDEX = 0;
                }
                else {
                    let rng = smash::app::sv_math::rand(hash40("cloud"), 2);
                    if rng == 0 {
                        VICTORY_COLOR_INDEX = 1;
                    }
                    else {
                        VICTORY_COLOR_INDEX = 2;
                    }
                }
            }
        }
    }
}

#[skyline::hook(offset=0x35ae700)] // 10.1.0
pub fn music_function_replace(param_1: *mut u64,param_2: i64,nus3bank_hash: u64,nus3audio_hash: *const u64,mut nus3audio_index: u32) {
    unsafe {
        if let Some(_path) = ARC_FILES.0.get(&*nus3audio_hash) {
            nus3audio_index = VICTORY_COLOR_INDEX;
            VICTORY_COLOR_INDEX = 0;
        }
    }
    original!()(param_1,param_2,nus3bank_hash,nus3audio_hash,nus3audio_index);
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    if fighter_kind == *FIGHTER_KIND_CLOUD {
        if DOWN_SMASH[entry_id(module_accessor)] || SIDE_SMASH[entry_id(module_accessor)] || DAIR[entry_id(module_accessor)]
        || UP_SMASH[entry_id(module_accessor)] || SPECIAL_N[entry_id(module_accessor)] {
            if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_CUT && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D {
                return false;
            }
            else {
                return ret;
            }
        }
        if SPECIAL_LW_COUNTER[entry_id(module_accessor)] {
            if SPECIAL_LW_COUNTER_CHECK[entry_id(module_accessor)] {
                if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP
                && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_CUT && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH
                && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_WAIT
                && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R
                && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                    return false;
                }
                else {
                    return ret;
                }
            }
        }
        if SPECIAL_HI[entry_id(module_accessor)] {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
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
    else {
        return ret;
    }
}

#[skyline::hook(offset=0x4dedc0)]
pub unsafe fn get_param_float_replace(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_CLOUD {
        if param_hash == smash::hash40("limit_gauge_attack_add") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 0.7;
            }
        }
        if param_hash == smash::hash40("limit_gauge_damage_add") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                if WorkModule::get_float(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE) <= 0.0 {
                    return 0.0;
                }
                else {
                    return -0.35;
                }
            }
        }
        if param_hash == smash::hash40("ground_brake") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 0.094;
            }
        }
        if param_hash == smash::hash40("dash_speed") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 1.87;
            }
        }
        if param_hash == smash::hash40("run_accel_mul") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 0.07964;
            }
        }
        if param_hash == smash::hash40("run_speed_max") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 1.672;
            }
        }
        if param_hash == smash::hash40("jump_initial_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 18.425;
            }
        }
        if param_hash == smash::hash40("jump_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 33.5;
            }
        }
        if param_hash == smash::hash40("mini_jump_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 16.8;
            }
        }
        if param_hash == smash::hash40("jump_aerial_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 33.5;
            }
        }
        if param_hash == smash::hash40("aerial_accel_x_mul") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 0.04;
            }
        }
        if param_hash == smash::hash40("air_speed_y_stable") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 1.58;
            }
        }
        if param_hash == smash::hash40("dive_speed_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 2.528;
            }
        }
        if param_hash == smash::hash40("cliff_jump_y") {
            if cloud_id(module_accessor) {
                return ret;
            }
            else {
                return 36.75;
            }
        }
        if param_type == smash::hash40("param_special_hi") {
            if param_hash == smash::hash40("special_air_hi_start_x_mul") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    return 2.0;
                }
            }
            if param_hash == smash::hash40("special_hi_pass_mul") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                        return 1.8;
                    }
                    else {
                        return 1.2;
                    }
                }
            }
            if param_hash == smash::hash40("special_air_hi_pass_mul") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                        return 2.0;
                    }
                    else {
                        return 1.4;
                    }
                }
            }
            if param_hash == smash::hash40("special_hi_fall_x_mul") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
                        return 1.2;
                    }
                    else {
                        return 1.0;
                    }
                }
            }
        }
        if param_type == smash::hash40("param_special_lw") {
            if param_hash == smash::hash40("limit_gauge_add") {
                if cloud_id(module_accessor)  {
                    return ret;
                }
                else {
                    return 0.0;
                }
            }
            if param_hash == smash::hash40("critical_zoom_rate") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    return 5.0;
                }
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::hook(offset=0x4ded80)]
pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_CLOUD {
        if param_type == smash::hash40("param_special_lw") {
            if param_hash == smash::hash40("limit_gauge_frame") {
                if cloud_id(module_accessor) == false {
                    return 5;
                }
                else {
                    return ret;
                }
            }
            if param_hash == smash::hash40("limit_break_clear_frame") {
                return 1800;
            }
            if param_hash == smash::hash40("critical_frame") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    return 60;
                }
            }
            if param_hash == smash::hash40("critical_slow_rate") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    return 12;
                }
            }
            if param_hash == smash::hash40("critical_bg_only_frame") {
                if cloud_id(module_accessor) {
                    return ret;
                }
                else {
                    return 30;
                }
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

pub fn install() {
    acmd::add_hooks!(
        attack_11,
        attack_12,
        attack_13,
        effect_13,
        attack_dash,
        effect_dash,
        attack_s3,
        effect_s3,
        attack_hi3,
        effect_hi3,
        attack_s4_hold,
        attack_s4,
        effect_s4,
        attack_hi4_hold,
        attack_hi4,
        effect_hi4,
        attack_lw4,
        effect_lw4,
        attack_air_n,
        effect_air_n,
        attack_air_f,
        effect_air_f,
        attack_air_b,
        effect_air_b,
        attack_air_hi,
        effect_air_hi,
        attack_air_lw,
        effect_air_lw,
        catch,
        catch_dash,
        catch_turn,
        catch_attack,
        throw_hi,
        throw_f,
        throw_b,
        throw_lw,
        special_n,
        effect_special_n,
        special_air_n,
        effect_special_air_n,
        special_n_lb,
        effect_special_n_lb,
        special_air_n_lb,
        effect_special_air_n_lb,
        special_hi,
        special_air_hi,
        special_hi2,
        special_hi_lb,
        special_air_hi_lb,
        special_lw,
        special_air_lw
    );
    acmd::add_custom_hooks!(
        randomized_limit_sword,
        special_lw_counter,
        moves_lag,
        sora_break,
        victory_theme
    );
    skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(music_function_replace);
    lazy_static::initialize(&ARC_FILES);
}
