use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterBase,L2CFighterCommon};
use acmd::{acmd, acmd_func};
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::BattleObjectModuleAccessor;

static mut GLOBAL_BULLET_TIME : bool = false;
static mut BULLET_TIME : [bool; 8] = [false; 8];
static mut STAMINA : [i32; 8] = [210; 8];
static mut STAMINA_EXHAUSTED : [bool; 8] = [false; 8];
static mut FIRE_ARROW : [bool; 8] = [false; 8];
static mut ICE_ARROW : [bool; 8] = [false; 8];
static mut SHOCK_ARROW : [bool; 8] = [false; 8];
static mut BOMB_ARROW : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE"]
    fn is_valid_just_shield_reflector(module_accessor: &mut BattleObjectModuleAccessor) -> bool;
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

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_LINK_ANCIENT_BOWARROW, 
    animation = "fly",
    animcmd = "game_fly")]
    pub fn ancient_arrow1(fighter: &mut L2CFighterBase) {
        acmd!({
            if(is_excute) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=66, FKB=0, BKB=10, Size=5.0, X=0.0, Y=0.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_NONE)
            }
        });
    }
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_LINK_ANCIENT_BOWARROW, 
    animation = "stick",
    animcmd = "game_stick")]
    pub fn ancient_arrow2(fighter: &mut L2CFighterBase) {
        acmd!({
            if(is_excute){
                CAM_ZOOM_OUT_FINAL()
            }
            if(is_excute){
                CAM_ZOOM_IN_FINAL_arg13(3, 0, 2, -1, 0, 1, -10, 10, true, WEAPON_LINK_ANCIENTBOWARROW_INSTANCE_WORK_ID_INT_HIT_OBJECT_ID, 0, -10, 0)
            }
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=35.0, Angle=60, KBG=46, FKB=0, BKB=102, Size=25.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_NONE)
                AttackModule::set_force_reaction(0, true, false)
            }
            frame(Frame=2)
            if(is_excute){
                AttackModule::clear_all()
                CAM_ZOOM_OUT_FINAL()
            }
            frame(Frame=15)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x199c462b5d)
            }     
        });
    }

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_LINK_ANCIENT_BOWARROW, 
    animation = "stick_wall",
    animcmd = "game_stickwall")]
    pub fn ancient_arrow3(fighter: &mut L2CFighterBase) {
        acmd!({
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=35, FKB=0, BKB=122, Size=25.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_NONE)
                AttackModule::set_force_reaction(0, true, false)
            }
            frame(Frame=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=10)
            if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x199c462b5d)
            }
        });
    }

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
                if FIRE_ARROW[ENTRY_ID] == true {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_fire"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=75, FKB=0, BKB=15, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
                else if ICE_ARROW[ENTRY_ID] == true {
                    acmd!({
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_freezer"), /*Bone*/ hash40("armr"), /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 6.5, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.10, true)
                        if(is_excute) {
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=80, FKB=0, BKB=15, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_ice"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FREEZE, Type=ATTACK_REGION_NONE)
                            AttackModule::enable_safe_pos()
                        }
                    });
                }
                else if SHOCK_ARROW[ENTRY_ID] == true {
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
                                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
                                AttackModule::set_lerp(0, 2, 0)
                                QUAKE(CAMERA_QUAKE_KIND_S)
                                AttackModule::enable_safe_pos()
                            }
                        });
                    }
                }
                else if BOMB_ARROW[ENTRY_ID] == true {
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
                                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=100, FKB=0, BKB=25, Size=1.35, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_BOMB)
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

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
            GLOBAL_BULLET_TIME = false;
        }
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
                if FIRE_ARROW[ENTRY_ID] == true {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_fire"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 2.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                    });
                }
                if ICE_ARROW[ENTRY_ID] == true {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_freezer"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.15, true)
                    });
                }
                if SHOCK_ARROW[ENTRY_ID] == true {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_damage_elec"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.3, true)
                    });
                }
                if BOMB_ARROW[ENTRY_ID] == true {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_flame"), /*Bone*/ hash40("arml"), /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.15, true)
                    });
                }
            }
            let mut caller_id : usize = 0;
            if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_SPECIAL)
            && ControlModule::get_stick_x(module_accessor) == 0.0 && ControlModule::get_stick_y(module_accessor) == 0.0
            && BULLET_TIME[ENTRY_ID] == false && STAMINA_EXHAUSTED[ENTRY_ID] == false && GLOBAL_BULLET_TIME == false {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                    acmd!(lua_state, {
                        sv_animcmd::SLOW_OPPONENT(30, 210)
                    });
                    BULLET_TIME[ENTRY_ID] = true;
                    GLOBAL_BULLET_TIME = true;
                    caller_id = ENTRY_ID;
                }
            }
            if ControlModule::get_stick_y(module_accessor) == 0.0 && STAMINA[ENTRY_ID] > 0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && BULLET_TIME[ENTRY_ID] == true
            && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP) == false {
                let fall_speed  = smash::phx::Vector3f { x: 0.4, y: 0.4, z: 0.4 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_ATTACK)
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_hi")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_blast")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_s1")
            || ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_JUMP)
            || STAMINA[ENTRY_ID] == 0 && STAMINA_EXHAUSTED[ENTRY_ID] == false && caller_id == ENTRY_ID {
                acmd!(lua_state, {
                    sv_animcmd::SLOW_OPPONENT(0, 0)
                });
                let fall_speed  = smash::phx::Vector3f { x: 1.0, y: 1.0, z: 1.0 };
                KineticModule::mul_speed(module_accessor, &fall_speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                BULLET_TIME[ENTRY_ID] = false;
                GLOBAL_BULLET_TIME = false;
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
            }
            if BULLET_TIME[ENTRY_ID] == true {
                EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_timer"),true,false);
                STAMINA[ENTRY_ID] -= 1;
                acmd!(lua_state, {
                    FT_SET_FINAL_FEAR_FACE(1)
                });
                if STAMINA[ENTRY_ID] % 21 == 0 {
                    acmd!(lua_state, {
                        sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_dragoon_bg_lightning"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.35, true)
                    });
                }
                if STAMINA[ENTRY_ID] <= 60 {
                    if STAMINA[ENTRY_ID] % 4 == 0 {
                        acmd!(lua_state, {
                            sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_timer"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.35, true)
                        });
                    }
                }
            }
            if BULLET_TIME[ENTRY_ID] == false && STAMINA[ENTRY_ID] == 0 {
                STAMINA_EXHAUSTED[ENTRY_ID] = true;
            }
            if STAMINA_EXHAUSTED[ENTRY_ID] == false {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    if STAMINA[ENTRY_ID] < 210 {
                        STAMINA[ENTRY_ID] += 2;
                        if STAMINA[ENTRY_ID] % 5 == 0 {
                            acmd!(lua_state, {
                                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_recovery"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
                            });
                        }
                    }
                }
                else {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
                if STAMINA[ENTRY_ID] == 210 {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
            }
            else {
                if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    if STAMINA[ENTRY_ID] < 210 {
                        STAMINA[ENTRY_ID] += 1;
                        if STAMINA[ENTRY_ID] % 10 == 0 {
                            acmd!(lua_state, {
                                sv_animcmd::EFFECT_FOLLOW(/*Effect*/ hash40("sys_recovery"), /*Bone*/ hash40("top"), /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*XRot*/ 0, /*YRot*/ 0, /*ZRot*/ 0, /*Size?*/ 0.65, true)
                            });
                        }
                    }
                }
                else {
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
                if STAMINA[ENTRY_ID] == 210 {
                    STAMINA_EXHAUSTED[ENTRY_ID] = false;
                    EffectModule::kill_kind(module_accessor,smash::phx::Hash40::new("sys_recovery"),true,false);
                }
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
                STAMINA[ENTRY_ID] = 210;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
                STAMINA[ENTRY_ID] = 210;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
                FIRE_ARROW[ENTRY_ID] = false;
                ICE_ARROW[ENTRY_ID] = false;
                SHOCK_ARROW[ENTRY_ID] = false;
                BOMB_ARROW[ENTRY_ID] = false;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
                acmd!(lua_state, {
                    sv_animcmd::SLOW_OPPONENT(30, 100)
                });
                STAMINA[ENTRY_ID] = 210;
                STAMINA_EXHAUSTED[ENTRY_ID] = false;
            }
        }
    }
}

pub fn install() {
    acmd::add_hooks!(arrow,ancient_arrow1,ancient_arrow2,ancient_arrow3);
    acmd::add_custom_hooks!(once_per_fighter_frame);
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
}
