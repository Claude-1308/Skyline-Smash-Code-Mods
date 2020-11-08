use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterBase,L2CFighterCommon};
use acmd::{acmd, acmd_func};
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::BattleObjectModuleAccessor;

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
    
pub fn cancellable_taunts(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_LINK {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_r") {
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }
}

pub fn install() {
    acmd::add_hooks!(
        ancient_arrow1,
        ancient_arrow2,
        ancient_arrow3
    );
    acmd::add_custom_hooks!(cancellable_taunts);
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
}
