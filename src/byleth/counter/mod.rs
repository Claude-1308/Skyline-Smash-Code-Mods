use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use smash::lib::lua_const::*;
use acmd::{acmd,acmd_func};
use smash::hash40;

static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_CONTINUE_MOTION : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_HIT : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD : [f32; 8] = [65.0; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_INTANGIBILITY : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER : [f32; 8] = [0.0; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID : [i32; 8] = [0; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE : [bool; 8] = [false; 8];
static mut FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_SITUATION_PREVIOUS : [i32; 8] = [0; 8];
static mut FIGHTERS : [u64; 8] = [0; 8];
static mut ENTRY_ID : usize = 0;
static mut CALLER_ID : usize = 0;
static mut OBJECT_ID : u32 = 0;

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
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] > 0.0 && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] < 9.0 {
                    dmgarrow = 9.0;
                    mul = 0;
                    kbg = 90;
                    bkb = 50;
                    hitlag = 1.5;
                    size = 10.0;
                }
                else if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] > 9.0 {
                    dmgarrow = FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] * 1.25;
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
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[i] {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[i] = -1.0;
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_MASTER {
            if ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI)
            || ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID] == false {
                    if CaptureModule::is_capture(module_accessor)
                    || StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_THROWN
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_WAIT
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_DAMAGE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_CONTINUE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_EAT
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_SPOT
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_STAND
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_STAND_FB
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_REFLECT_LR
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE 
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_AIR 
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY 
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR 
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U 
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FALL
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLIP
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SWIM
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLEEP
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_B
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_F
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FURAFURA
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_WAIT
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CATCH
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CLIMB
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP1
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP2
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP3
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ATTACK
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ESCAPE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ROBBED
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_THROW
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SWALLOWED
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_AIR_LASSO
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCHED_REFLET
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD
                    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
                    || WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) {
                        FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID] = false;
                        FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_CONTINUE_MOTION[ENTRY_ID] = false;
                    }
                    else {
                        StatusModule::init_settings(module_accessor,smash::app::SituationKind(*SITUATION_KIND_NONE),*FIGHTER_KINETIC_TYPE_UNIQ,*GROUND_CORRECT_KIND_KEEP as u32,smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),true,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,*FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT as i32,0);
                        FighterStatusModuleImpl::set_fighter_status_data(module_accessor,false,*FIGHTER_TREADED_KIND_NO_REAC,false,false,false,*FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK as u64,0,*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,0);
                        FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID] = true;
                        FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_CONTINUE_MOTION[ENTRY_ID] = true;
                        ArticleModule::generate_article(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,false,0);
                        CALLER_ID = ENTRY_ID;
                    }
                }
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID]
            && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] > 58.0 {
                if StopModule::is_damage(module_accessor) {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] = -1.0;
                }
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] == 59.0 {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_CONTINUE_MOTION[ENTRY_ID] = false;
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT[ENTRY_ID] = true;
                }
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_CONTINUE_MOTION[ENTRY_ID] {
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] == 63.0 {
                    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        GroundModule::correct(module_accessor,smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    }
                    else {
                        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                        GroundModule::correct(module_accessor,smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    }
                }
                else if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] == 60.0 {
                    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        let speed  = smash::phx::Vector3f { x: 0.2, y: 0.2, z: 0.2 };
                        KineticModule::mul_accel(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        KineticModule::mul_speed(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        KineticModule::mul_accel(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        KineticModule::mul_speed(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        KineticModule::mul_accel(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        KineticModule::mul_speed(module_accessor, &speed, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    }
                    else if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
                        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
                        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        KineticModule::unable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    }
                }
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT[ENTRY_ID] {
                DamageModule::set_damage_lock(module_accessor,true);
                if CaptureModule::is_capture(module_accessor)
                || StatusModule::prev_status_kind(module_accessor,0) == *FIGHTER_STATUS_KIND_THROWN
                || WorkModule::is_flag(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI) {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] = -1.0;
                }
                else if WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) > 0.0 {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_HIT[ENTRY_ID] = true;
                }
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_HIT[ENTRY_ID] {
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] = WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
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
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] = array_id_i;
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] >= 0
                && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] < 8 {
                    HitModule::set_whole(FIGHTERS[FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] as usize] as *mut smash::app::BattleObjectModuleAccessor,smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                }
                let launch_speed = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
                let facing_dirn = PostureModule::lr(module_accessor);
                if launch_speed > 0.0 && facing_dirn > 0.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                else if launch_speed < 0.0 && facing_dirn < 0.0 {
                    PostureModule::reverse_lr(module_accessor);
                }
                else if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] >= 0
                && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] < 8 {
                    let pos_x_1 = PostureModule::pos_x(FIGHTERS[FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] as usize] as *mut smash::app::BattleObjectModuleAccessor);
                    let pos_x_2 = PostureModule::pos_x(module_accessor);
                    if pos_x_2 > pos_x_1 {
                        PostureModule::set_lr(module_accessor, -1.0);
                    }
                    else {
                        PostureModule::set_lr(module_accessor, 1.0);
                    }
                }
                KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
                MotionModule::change_motion(module_accessor, smash::phx::Hash40::new("special_n"), 0.0, 0.0, false, 0.0, false, false);
                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT,true);
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_HIT[ENTRY_ID] = false;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT[ENTRY_ID] = false;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_INTANGIBILITY[ENTRY_ID] = true;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] = 25.0;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] = true;
                WorkModule::set_float(module_accessor,0.0,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] < 25.0 {
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT[ENTRY_ID] = false;
                DamageModule::set_damage_lock(module_accessor,false);
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_INTANGIBILITY[ENTRY_ID] {
                    HitModule::set_whole(module_accessor,smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] == 24.0 {
                        ArticleModule::remove_exist_object_id(FIGHTERS[FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_ATTACKER_ENTRY_ID[ENTRY_ID] as usize] as *mut smash::app::BattleObjectModuleAccessor,OBJECT_ID);
                    }
                }
                if StopModule::is_damage(module_accessor) {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] = -1.0;
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] = true;
                    ArticleModule::remove_exist(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] < 0.0 {
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_INTANGIBILITY[ENTRY_ID] = false;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] = 0.0;
                HitModule::set_whole(module_accessor,smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] < 0.0
            && ControlModule::check_button_off(module_accessor,*CONTROL_PAD_BUTTON_APPEAL_HI) {
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] == false {
                    ArticleModule::remove_exist(module_accessor,*FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
                if StatusModule::status_kind(module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_WAIT,true);
                }
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID] = false;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] = 65.0;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_WORK_FLOAT_ATTACK_POWER[ENTRY_ID] = 0.0;
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] = false;
                HitModule::set_whole(module_accessor,smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                WorkModule::enable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
            }
            if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLAG_SHIELD[ENTRY_ID] {
                FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] -= 1.0;
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] == 64.0 {
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_SITUATION_PREVIOUS[ENTRY_ID] = StatusModule::situation_kind(module_accessor);
                }
                if StatusModule::situation_kind(module_accessor) != FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_SITUATION_PREVIOUS[ENTRY_ID]
                && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] == false {
                    KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                    GroundModule::correct(module_accessor,smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_SITUATION_PREVIOUS[ENTRY_ID] = StatusModule::situation_kind(module_accessor);
                }
                if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID] > 0.0
                && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_MOTION_CHANGE[ENTRY_ID] == false {
                    if PostureModule::lr(module_accessor) == -1.0 {
                        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("appeal_hi_l"),65.0 - FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID],0.0,0.0);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(module_accessor,smash::phx::Hash40::new("appeal_hi_r"),65.0 - FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_FLOAT_SHIELD[ENTRY_ID],0.0,0.0);
                    }
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
                    WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
                }
                WorkModule::unable_transition_term(module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
            }
        }
    }
}

pub fn once_per_weapon_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let weapon_module_accessor = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        if FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_CHECK_HIT[CALLER_ID] {
            if AttackModule::is_infliction_status(weapon_module_accessor, *COLLISION_KIND_MASK_HIT)
            && FIGHTER_MASTER_STATUS_KIND_APPEAL_HI_HIT[CALLER_ID] {
                let battle_object = smash::app::sv_system::battle_object(weapon.lua_state_agent);
                let article = &mut smash::app::Article{battle_object: *battle_object};
                OBJECT_ID = Article::get_battle_object_id(article) as u32;
            }
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
    acmd::add_hooks!(arrow);
}
