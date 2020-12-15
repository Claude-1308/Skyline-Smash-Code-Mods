use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterBase,L2CFighterCommon};
use acmd::{acmd, acmd_func};
use smash::hash40;
use smash::app::lua_bind::*;

static mut FIRE_ARROW : [bool; 8] = [false; 8];
static mut ICE_ARROW : [bool; 8] = [false; 8];
static mut SHOCK_ARROW : [bool; 8] = [false; 8];
static mut BOMB_ARROW : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_LINK_BOWARROW, 
    animation = "fly",
    animcmd = "game_fly")]
pub fn arrow(fighter: &mut L2CFighterBase) {
    acmd!({
        rust {
            let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            if FIRE_ARROW[ENTRY_ID] {
                acmd!({
                    sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_fire"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                    if(is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=75, FKB=0, BKB=15, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                        AttackModule::enable_safe_pos()
                    }
                });
            }
            else if ICE_ARROW[ENTRY_ID] {
                acmd!({
                    sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_freezer"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.10, true)
                    if(is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=80, FKB=0, BKB=15, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_ice"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FREEZE, Type=ATTACK_REGION_NONE)
                        AttackModule::enable_safe_pos()
                    }
                });
            }
            else if SHOCK_ARROW[ENTRY_ID] {
                if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_hit_elec_s"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.25, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                            AttackModule::set_lerp(0, 2, 0)
                            AREA_WIND_2ND_RAD(0,2,0.02,1000,1,0,0,60)
                            QUAKE(CAMERA_QUAKE_KIND_S)
                        }
                        wait(Frames=5)
                        if(is_excute) {
                            AreaModule::erase_wind(0)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
                else {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_hit_elec_s"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.25, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
            }
            else if BOMB_ARROW[ENTRY_ID] {
                if WorkModule::is_flag(owner_module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX) {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_bomb_spark"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.25, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=100, FKB=0, BKB=25, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_BOMB)
                            AREA_WIND_2ND_RAD(0,2,0.02,1000,1,0,0,60)
                            QUAKE(CAMERA_QUAKE_KIND_L)
                        }
                        wait(Frames=5)
                        if(is_excute) {
                            AreaModule::erase_wind(0)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
                else {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_bomb_spark"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.25, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=25, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_BOMB)
                            QUAKE(CAMERA_QUAKE_KIND_L)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
            }
            else {
                acmd!({
                    if(is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=71, FKB=0, BKB=10, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                        AttackModule::enable_safe_pos()
                    }
                });
            }
        }    
    });
}

pub fn link_elemental_arrows(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if fighter_kind == *FIGHTER_KIND_LINK {
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
            if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) && WorkModule::is_flag(module_accessor,*FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
                if FIRE_ARROW[ENTRY_ID] {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_fire"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 2.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                    });
                }
                else if ICE_ARROW[ENTRY_ID] {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_freezer"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.15, true)
                    });
                }
                else if SHOCK_ARROW[ENTRY_ID] {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_elec"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                    });
                }
                else if BOMB_ARROW[ENTRY_ID] {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_flame"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.15, true)
                    });
                }
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_hooks!(arrow);
    acmd::add_custom_hooks!(link_elemental_arrows);
}
