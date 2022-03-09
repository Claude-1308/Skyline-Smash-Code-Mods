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

#[acmd_script(agent = "jack", script = "game_attacks3hi", category = ACMD_GAME)]
pub unsafe fn attack_s3_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=180, KBG=100, FKB=10, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=35, KBG=100, FKB=25, BKB=0, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=45, KBG=128, FKB=0, BKB=45, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                AttackModule::set_size(ID=2, Size=0.0)
                ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=45, KBG=121, FKB=0, BKB=60, Size=4.0, X=0.0, Y=3.5, Z=0.0, X2=0.0, Y2=4.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=1, Bone=hash40("knife"), Damage=5.0, Angle=45, KBG=121, FKB=0, BKB=60, Size=4.0, X=0.0, Y=3.5, Z=0.0, X2=0.0, Y2=4.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_BOMB)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            }
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear(ID=2,false)
        }
        frame(Frame=20)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "jack", script = "game_attacks3", category = ACMD_GAME)]
pub unsafe fn attack_s3_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=93, KBG=100, FKB=20, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=40, KBG=100, FKB=20, BKB=0, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=35, KBG=110, FKB=0, BKB=45, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                AttackModule::set_size(ID=2, Size=0.0)
                ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=35, KBG=106, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.5, Z=0.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=0, Part=1, Bone=hash40("knife"), Damage=5.0, Angle=35, KBG=106, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.5, Z=0.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_BOMB)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            }
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear(ID=2,false)
        }
        frame(Frame=20)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "jack", script = "game_attacks3lw", category = ACMD_GAME)]
pub unsafe fn attack_s3_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=93, KBG=100, FKB=20, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=3.0, Angle=40, KBG=100, FKB=20, BKB=0, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=35, KBG=110, FKB=0, BKB=45, Size=2.5, X=0.0, Y=-3.0, Z=0.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                AttackModule::set_size(ID=2, Size=0.0)
                ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=5.0, Angle=25, KBG=106, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.5, Z=0.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=0, Part=1, Bone=hash40("knife"), Damage=5.0, Angle=25, KBG=106, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.5, Z=0.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_BOMB)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            }
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear(ID=2,false)
        }
        frame(Frame=20)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "jack_doyle", scripts = ["effect_attacks3hi", "effect_attacks3s", "effect_attacks3lw"], category = ACMD_EFFECT)]
pub unsafe fn effect_s3(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    match WorkModule::get_int(owner_module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND) {
        PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL | PERSONA_KIND_OKUNINUSHI | PERSONA_KIND_MESSIAH => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("jack_doyle_magic_flash"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("jack_doyle_magic_flash"), hash40("handl"), 3, 2, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true)
                    EFFECT_FLIP(hash40("jack_doyle_crow"), hash40("jack_doyle_crow"), hash40("top"), -1, 20, 4, -55, 9, -257, 0.6, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
                }
            });
        },
        PERSONA_KIND_HACHIMAKI => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_damage_elec"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_damage_elec"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_AME => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_hit_ice"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_hit_ice"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_SETANTA => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_damage_aura"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_damage_aura"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_ORPHEOUS => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_damage_fire"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_damage_fire"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_hit_magic"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_hit_magic"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        PERSONA_KIND_YOSHITSUNE => {
            acmd!(lua_state, {
                frame(Frame=3)
                if(is_excute){
                    EFFECT(hash40("sys_hit_normal"), hash40("handl"), 2, 2, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, true)
                }
                frame(Frame=11)
                if(is_excute){
                    EFFECT(hash40("sys_hit_normal"), hash40("handl"), 3, 2, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
                }
            });
        },
        _ => {},
    }
}

#[acmd_script(agent = "jack", script = "game_attackhi3", category = ACMD_GAME)]
pub unsafe fn attack_hi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            MotionModule::set_rate(1.5)
        }
        frame(Frame=10)
        if(is_excute){
            MotionModule::set_rate(1.0)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=120, KBG=100, FKB=150, BKB=0, Size=2.5, X=0.0, Y=4.0, Z=4.0, X2=0.0, Y2=8.0, Z2=7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=105, KBG=100, FKB=150, BKB=0, Size=2.5, X=-1.0, Y=0.0, Z=0.0, X2=1.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=100, KBG=100, FKB=60, BKB=0, Size=2.5, X=-1.0, Y=0.0, Z=0.0, X2=1.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=110, KBG=100, FKB=165, BKB=0, Size=2.0, X=0.0, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=115, KBG=100, FKB=60, BKB=0, Size=2.0, X=0.0, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=123, KBG=100, FKB=165, BKB=0, Size=2.0, X=0.0, Y=3.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=115, KBG=100, FKB=60, BKB=0, Size=2.0, X=0.0, Y=3.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear(ID=0,false)
            AttackModule::clear(ID=1,false)
            AttackModule::clear(ID=2,false)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=130, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=23.0, Z=1.0, X2=0.0, Y2=20.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=4.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=23.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=4.0, Angle=133, KBG=100, FKB=0, BKB=0, Size=3.0, X=0.0, Y=13.0, Z=7.5, X2=0.0, Y2=18.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=6,false)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=1.0, Angle=367, KBG=20, FKB=0, BKB=20, Size=5.0, X=0.0, Y=1.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=4,false)
            AttackModule::clear(ID=5,false)
        }
        frame(Frame=14)
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=0.7, Angle=367, KBG=20, FKB=0, BKB=50, Size=6.0, X=0.0, Y=25.0, Z=-1.0, X2=0.0, Y2=25.0, Z2=1.0, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_SWORD)
            }
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=1.0, Angle=90, KBG=270, FKB=0, BKB=35, Size=5.0, X=0.0, Y=1.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=0,false)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=3.9, Angle=90, KBG=180, FKB=0, BKB=50, Size=5.0, X=0.0, Y=1.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sound, Type=ATTACK_REGION_SWORD)
            }
        }
        frame(Frame=27)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}