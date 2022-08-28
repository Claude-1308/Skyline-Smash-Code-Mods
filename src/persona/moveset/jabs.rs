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

#[acmd_script(agent = "jack", script = "game_attack11", category = ACMD_GAME)]
pub unsafe fn attack_11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = JackModule::decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=32, Size=2.0, X=0.0, Y=7.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=32, Size=2.0, X=0.0, Y=7.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=20, FKB=0, BKB=32, Size=2.0, X=0.0, Y=7.5, Z=12.5, X2=0.0, Y2=7.5, Z2=15.2, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=25, Size=2.0, X=0.0, Y=7.5, Z=12.5, X2=0.0, Y2=7.5, Z2=15.2, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=6)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_script(agent = "jack_doyle", script = "effect_attack11", category = ACMD_EFFECT)]
unsafe fn doyle_effect_11(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_MESSIAH => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("jack_doyle_magic_flash"), hash40("handl"), 2, 0, 0, 0, 0, 0, 1, true)
                }
                frame(Frame=5)
                if(is_excute){
                    EFFECT_FLIP(hash40("jack_doyle_crow"), hash40("jack_doyle_crow"), hash40("top"), -3, 18, 2, -9, 13, -230, 0.8, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
                    LAST_EFFECT_SET_RATE(1.5)
                }
            });
        },
        PERSONA_KIND_HACHIMAKI => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_elec"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_AME => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_ice"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.3, true)
                }
            });
        },
        PERSONA_KIND_SETANTA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_aura"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_ORPHEOUS => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_magic"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.3, true)
                }
            });
        },
        PERSONA_KIND_YOSHITSUNE => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_normal"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.2, true)
                }
            });
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack", script = "game_attack12", category = ACMD_GAME)]
pub unsafe fn attack_12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = JackModule::decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=30, FKB=0, BKB=25, Size=2.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=30, FKB=0, BKB=25, Size=2.0, X=0.0, Y=8.0, Z=13.0, X2=0.0, Y2=8.0, Z2=14.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=30, FKB=0, BKB=30, Size=2.5, X=0.0, Y=8.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=30, FKB=0, BKB=30, Size=2.5, X=0.0, Y=8.0, Z=18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_script(agent = "jack_doyle", script = "effect_attack12", category = ACMD_EFFECT)]
unsafe fn doyle_effect_12(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_MESSIAH => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("jack_doyle_magic_flash"), hash40("handr"), 2, 0, 0, 0, 0, 0, 1, true)
                }
                frame(Frame=5)
                if(is_excute){
                    EFFECT_FLIP(hash40("jack_doyle_crow"), hash40("jack_doyle_crow"), hash40("top"), -3, 18, 8, 24, -7, 40, 0.8, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
                    LAST_EFFECT_SET_RATE(1.5)
                }
            });
        },
        PERSONA_KIND_HACHIMAKI => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_elec"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_AME => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_ice"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.3, true)
                }
            });
        },
        PERSONA_KIND_SETANTA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_aura"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_ORPHEOUS => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_magic"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.3, true)
                }
            });
        },
        PERSONA_KIND_YOSHITSUNE => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_normal"), hash40("handr"), 2, 0, 0, 0, 0, 0, 0.2, true)
                }
            });
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack", script = "game_attack13", category = ACMD_GAME)]
pub unsafe fn attack_13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = JackModule::decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=13.5, X2=0.0, Y2=8.0, Z2=14.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=2.0, Z2=0.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.95, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=13.5, X2=0.0, Y2=8.0, Z2=14.5, Hitlag=1.95, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=2.0, Z2=0.0, Hitlag=1.95, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=3.1, Angle=361, KBG=135, FKB=0, BKB=65, Size=2.5, X=0.0, Y=8.0, Z=11.0, X2=0.0, Y2=8.0, Z2=14.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=sound, Type=ATTACK_REGION_ENERGY)
            }
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.95, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=5, Part=1, Bone=hash40("top"), Damage=3.1, Angle=361, KBG=135, FKB=0, BKB=65, Size=2.5, X=0.0, Y=8.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sound, Type=ATTACK_REGION_ENERGY)
            }
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "jack_doyle", script = "effect_attack13", category = ACMD_EFFECT)]
pub unsafe fn doyle_effect_13(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_MESSIAH => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("jack_doyle_magic_flash"), hash40("handl"), 2, 0, 0, 0, 0, 0, 1, true)
                }
                frame(Frame=2)
                if(is_excute){
                    EFFECT_FLIP(hash40("jack_doyle_crow_line"), hash40("jack_doyle_crow_line"), hash40("top"), 1, 21, 3, 26, -20, -4, 1, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
                }
            });
        },
        PERSONA_KIND_HACHIMAKI => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_elec"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_AME => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_ice"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.35, true)
                }
            });
        },
        PERSONA_KIND_SETANTA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_aura"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_ORPHEOUS => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.8, true)
                }
            });
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_magic"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.3, true)
                }
            });
        },
        PERSONA_KIND_YOSHITSUNE => {
            acmd!(lua_state, {
                if(is_excute){
                    EFFECT_FOLLOW(hash40("sys_hit_normal"), hash40("handl"), 2, 0, 0, 0, 0, 0, 0.2, true)
                }
            });
        },
        _ => {},
    }
}