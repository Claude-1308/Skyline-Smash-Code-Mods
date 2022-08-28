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
use crate::moveset::modules::*;

#[acmd_script(agent = "jack", script = "game_attackairn", category = ACMD_GAME)]
pub unsafe fn attack_air_n(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = JackModule::decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=7.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=2.8, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=3, Part=1, Bone=hash40("throw"), Damage=4.0, Angle=46, KBG=139, FKB=0, BKB=50, Size=2.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=sound, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=48)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "jack", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn attack_air_b(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let (effect,sound) = JackModule::decide_box_property(WorkModule::get_int(module_accessor,FIGHTER_JACK_INSTANCE_WORK_ID_INT_PERSONA_KIND));
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=0.0, Y=3.3, Z=-6.0, X2=0.0, Y2=10.7, Z2=-8.4, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=0.0, Y=4.7, Z=-11.1, X2=0.0, Y2=11.1, Z2=-14.2, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=3.0, X=0.0, Y=4.0, Z=-3.0, X2=0.0, Y2=8.5, Z2=-5.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=114, FKB=0, BKB=58, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=114, FKB=0, BKB=58, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=5.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
            }
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=-1.0, Y=2.5, Z=1.0, X2=-3.0, Y2=-0.5, Z2=8.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=0.0, Y=-3.0, Z=2.5, X2=-4.5, Y2=-6.0, Z2=10.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=3.5, X=0.0, Y=4.5, Z=-6.5, X2=0.0, Y2=7.5, Z2=-5.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        if(WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=114, FKB=0, BKB=58, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=114, FKB=0, BKB=58, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=5.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=effect, SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=sound, Type=ATTACK_REGION_PUNCH)
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}