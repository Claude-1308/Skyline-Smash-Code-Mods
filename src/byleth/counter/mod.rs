use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use smash::lib::lua_const::*;
use acmd::{acmd,acmd_func};
use smash::hash40;
use smash::app::BattleObjectModuleAccessor;

static mut COUNTER_START : [bool; 8] = [false; 8];
static mut CHECK_HIT : [bool; 8] = [false; 8];
static mut COUNTER_HIT : [bool; 8] = [false; 8];
static mut COUNTER_TIMER : [f32; 8] = [65.0; 8];
static mut COUNTER_INVINCIBILITY : [bool; 8] = [false; 8];
static mut COUNTER_POWER : [f32; 8] = [0.0; 8];
static mut ATTACKER_ENTRY_ID : [i32; 8] = [0; 8];
static mut ATTACKED : [bool; 8] = [false; 8];
static mut MOTION_CHANGE : [bool; 8] = [false; 8];
static mut FIGHTERS : [u64; 8] = [0; 8];
static mut ENTRY_ID : usize = 0;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_MASTER_ARROW2, 
    animation = "search",
    animcmd = "game_search")]
    pub fn arrow(fighter: &mut L2CFighterBase) {
        acmd!({
            rust {
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                let dmgarrow : f32;    
                let mul : i32;
                let kbg : i32;
                let bkb : i32;
                let hitlag : f32;
                let size : f32;
                if COUNTER_POWER[ENTRY_ID] > 0.0 && COUNTER_POWER[ENTRY_ID] < 9.0 {
                    dmgarrow = 9.0;
                    mul = 0;
                    kbg = 90;
                    bkb = 50;
                    hitlag = 1.5;
                    size = 10.0;
                }
                else if COUNTER_POWER[ENTRY_ID] > 9.0 {
                    dmgarrow = COUNTER_POWER[ENTRY_ID] * 1.25;
                    mul = 0;
                    kbg = 90;
                    bkb = 50;
                    hitlag = 1.5;
                    size = 10.0;
                }
                else {
                    dmgarrow = 29.0;
                    mul = -1;
                    kbg = 56;
                    bkb = 53;
                    hitlag = 0.8;
                    size = 1.2;
                }
            }
            if(is_excute) {
                ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_ARROW2,false,0)
                ArticleModule::shoot_exist(FIGHTER_MASTER_GENERATE_ARTICLE_ARROW2,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false)
                SET_SPEED_EX(mul, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            }
            if(is_excute) {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=dmgarrow, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=size, X=2.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=1.0, Hitlag=hitlag, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=9, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=dmgarrow, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=size, X=-2.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=1.0, Hitlag=hitlag, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=9, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, Type=ATTACK_REGION_ENERGY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=dmgarrow, Angle=361, KBG=kbg, FKB=0, BKB=bkb, Size=size, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=1.0, Hitlag=hitlag, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=9, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, Type=ATTACK_REGION_ENERGY)
                AttackModule::disable_tip()
            }
        });
    }

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        FIGHTERS[ENTRY_ID] = (&mut *module_accessor as *mut smash::app::BattleObjectModuleAccessor) as u64;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL {
            for i in 0..8 {
                if COUNTER_START[i] {
                    COUNTER_TIMER[i] = -1.0;
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_MASTER {
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI)
            || ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
                if COUNTER_START[ENTRY_ID] == false {
                    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WAIT
                        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_RUN
                        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DASH {
                            COUNTER_START[ENTRY_ID] = true;
                            ArticleModule::generate_article(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,false,0);
                        }
                    }
                    else if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        if StatusModule::prev_status_kind(module_accessor,0) != *FIGHTER_STATUS_KIND_THROWN
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_AIR 
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY 
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR 
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U 
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_DAMAGE_FALL
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_FINAL
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SLEEP
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_ESCAPE_B
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_ESCAPE_F
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SWALLOWED
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_AIR_LASSO
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CATCHED_REFLET
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
                        && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_ATTACK_AIR
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_n_start")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_n")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_n_max")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_n_cancel")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_n_jump_cancel")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_lw_landing2")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_lw")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_lw_hit")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_lw_turn")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_hi")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_hi_overtake")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_s_landing")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_s_start")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_s_front")
                        && MotionModule::motion_kind(module_accessor) != smash::hash40("special_air_s_front_dash")
                        && WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) == false {
                            COUNTER_START[ENTRY_ID] = true;
                            ArticleModule::generate_article(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,false,0);
                        }
                    }
                }
            }
            if COUNTER_START[ENTRY_ID]
            && COUNTER_TIMER[ENTRY_ID] > 58.0 {
                if StopModule::is_damage(module_accessor) {
                    COUNTER_TIMER[ENTRY_ID] = -1.0;
                }
                if COUNTER_TIMER[ENTRY_ID] == 59.0 {
                    CHECK_HIT[ENTRY_ID] = true;
                    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        let speed  = smash::phx::Vector3f { x: 0.2, y: 0.2, z: 0.2 };
                        KineticModule::mul_accel(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        KineticModule::mul_speed(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    }
                }
            }
            if CHECK_HIT[ENTRY_ID] {
                DamageModule::set_damage_mul(module_accessor,0.0001);
                if CaptureModule::is_capture(module_accessor)
                || StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_THROWN
                || WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) {
                    COUNTER_TIMER[ENTRY_ID] = -1.0;
                }
                else if WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) > 0.0 {
                    COUNTER_HIT[ENTRY_ID] = true;
                }
            }
            if COUNTER_HIT[ENTRY_ID] {
                COUNTER_POWER[ENTRY_ID] = WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) * 10000.0;
                let mut attacker_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
                let mut mask: u32 = 0x1;
                let mut counter = 0;
                loop {
                    if ((attacker_id as u32) & !mask) == 0 {
                        break;
                    }
                    counter += 1;
                    mask |= mask << 1;
                }
                attacker_id &= 1 << counter;
                let array_id_f = (attacker_id as f32).log2();
                let mut array_id_i: i32 = -1;
                if array_id_f.is_normal() || array_id_f == 0.0 {
                    array_id_i = array_id_f as i32;
                }
                ATTACKER_ENTRY_ID[ENTRY_ID] = array_id_i;
                if ATTACKER_ENTRY_ID[ENTRY_ID] >= 0
                && ATTACKER_ENTRY_ID[ENTRY_ID] < 8 {
                    HitModule::set_whole(FIGHTERS[ATTACKER_ENTRY_ID[ENTRY_ID] as usize] as *mut smash::app::BattleObjectModuleAccessor,smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                }
                let launch_speed = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
                let facing_dirn = PostureModule::lr(module_accessor);
                if launch_speed > 0.0 && facing_dirn > 0.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                else if launch_speed < 0.0 && facing_dirn < 0.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                else if ATTACKER_ENTRY_ID[ENTRY_ID] >= 0
                && ATTACKER_ENTRY_ID[ENTRY_ID] < 8 {
                    let pos_x_1 = PostureModule::pos_x(FIGHTERS[ATTACKER_ENTRY_ID[ENTRY_ID] as usize] as *mut smash::app::BattleObjectModuleAccessor);
                    let pos_x_2 = PostureModule::pos_x(module_accessor);
                    if pos_x_2 > pos_x_1 {
                        PostureModule::set_lr(module_accessor, -1.0);
                    }
                    else {
                        PostureModule::set_lr(module_accessor, 1.0);
                    }
                }
                if ItemModule::is_attach_item(module_accessor,smash::app::ItemKind(*ITEM_KIND_RIPSTICKFLOWER)) {
                    ItemModule::eject_attach(module_accessor,smash::app::ItemKind(*ITEM_KIND_RIPSTICKFLOWER),true,false);
                }
                else if ItemModule::is_attach_item(module_accessor,smash::app::ItemKind(*ITEM_KIND_WALKMUSHFLOWER)) {
                    ItemModule::eject_attach(module_accessor,smash::app::ItemKind(*ITEM_KIND_WALKMUSHFLOWER),true,false);
                }
                KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
                MotionModule::change_motion(module_accessor, smash::phx::Hash40::new("special_n"), 0.0, 0.0, false, 0.0, false, false);
                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT,true);
                COUNTER_HIT[ENTRY_ID] = false;
                CHECK_HIT[ENTRY_ID] = false;
                COUNTER_INVINCIBILITY[ENTRY_ID] = true;
                COUNTER_TIMER[ENTRY_ID] = 25.0;
                MOTION_CHANGE[ENTRY_ID] = true;
                ATTACKED[ATTACKER_ENTRY_ID[ENTRY_ID] as usize] = true;
                WorkModule::set_float(module_accessor,0.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
            }
            if COUNTER_TIMER[ENTRY_ID] < 25.0
            && COUNTER_TIMER[ENTRY_ID] > 0.0 {
                CHECK_HIT[ENTRY_ID] = false;
                if COUNTER_INVINCIBILITY[ENTRY_ID] {
                    acmd!(lua_state, {
                        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
                        DamageModule::set_damage_mul(0.0001)
                    });
                }
                else if StopModule::is_damage(module_accessor) {
                    COUNTER_TIMER[ENTRY_ID] = -1.0;
                    MOTION_CHANGE[ENTRY_ID] = true;
                    DamageModule::set_damage_mul(module_accessor,1.0);
                }
            }
            if COUNTER_TIMER[ENTRY_ID] < 0.0 {
                COUNTER_INVINCIBILITY[ENTRY_ID] = false;
                COUNTER_POWER[ENTRY_ID] = 0.0;
                acmd!(lua_state, {
                    sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
                    DamageModule::set_damage_mul(1.0)
                });
            }
            if COUNTER_TIMER[ENTRY_ID] < 0.0
            && ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
                if MOTION_CHANGE[ENTRY_ID] == false {
                    ArticleModule::remove_exist(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
                if StatusModule::status_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_WAIT,true);
                }
                COUNTER_START[ENTRY_ID] = false;
                COUNTER_TIMER[ENTRY_ID] = 65.0;
                COUNTER_POWER[ENTRY_ID] = 0.0;
                MOTION_CHANGE[ENTRY_ID] = false;
            }
            if COUNTER_START[ENTRY_ID] {
                COUNTER_TIMER[ENTRY_ID] -= 1.0;
                if COUNTER_TIMER[ENTRY_ID] > 0.0
                && MOTION_CHANGE[ENTRY_ID] == false {
                    KineticModule::clear_speed_energy_id(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    if PostureModule::lr(module_accessor) == -1.0 {
                        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("appeal_hi_l"),65.0 - COUNTER_TIMER[ENTRY_ID],0.0,0.0);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("appeal_hi_r"),65.0 - COUNTER_TIMER[ENTRY_ID],0.0,0.0);
                    }
                }
            }
        }
    }
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, arg2: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,arg2);
    ENTRY_ID = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter_kind == *FIGHTER_KIND_MASTER {
        if COUNTER_START[ENTRY_ID] && COUNTER_HIT[ENTRY_ID] == false {
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

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_hooks!(arrow);
    skyline::install_hook!(is_enable_transition_term_replace);
}
