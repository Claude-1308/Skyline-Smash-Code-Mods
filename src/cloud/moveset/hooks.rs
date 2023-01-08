use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash::lua2cpp::L2CFighterBase;
use skyline::hooks::InlineCtx;
use skyline::libc::*;

use crate::moveset::*;

#[skyline::hook(offset=0x8dc140)]
pub unsafe fn limit_manager(amount: f32, module_accessor: *mut BattleObjectModuleAccessor, unk: u64) {
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) == false {
        let gauge = WorkModule::get_float(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        let effect = WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
        let mut tot = gauge + amount;
        if tot >= 100.0 {
            WorkModule::set_float(module_accessor,100.0,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
            if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL) == LIMIT_BREAK_LEVEL_4 {
                if smash::app::lua_bind::FighterManager::is_final(fighter_manager) == false {
                    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
                    smash::app::lua_bind::FighterManager::set_final(fighter_manager,FighterEntryID(entry_id),smash::app::FighterAvailableFinal(*(smash::lib::lua_const::FighterAvailableFinal::DEFAULT)),0u32);
                }
            }
            else {
                let vec3 = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                EffectModule::req_follow(module_accessor,Hash40::new("cloud_limitbreak_aura"),Hash40::new("hip"),&vec3,&vec3,2.0,false,0x8000000,0,-1,0,0,false,false);
                EffectModule::req_common(module_accessor,Hash40::new("cloud_limitbreak"),0.0);
                EffectModule::req_follow(module_accessor,Hash40::new_raw(0x16102a334bu64),Hash40::new("top"),&vec3,&vec3,1.0,true,0,0,-1,0,0,false,false);
                if effect != 0 {
                    EffectModule::detach(module_accessor,effect as u32,0);
                    let limit_gauge_offset_y = WorkModule::get_param_float(module_accessor,hash40("param_special_lw"),hash40("limit_gauge_offset_y"));
                    let vec3 = Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0};
                    let color = FighterUtil::get_team_color(module_accessor) as i32;
                    EffectModule::req_follow(module_accessor,Hash40::new_raw(0x14d013ba16u64),Hash40::new("top"),&vec3,&vec3,1.0,true,0,0,color,0,0,false,false);
                    EffectModule::set_rot(module_accessor,effect as u32,&vec3);
                    WorkModule::set_int64(module_accessor,effect as i64,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
                }
                WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
                WorkModule::on_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_WAZA);
                //WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
                FighterUtil::flash_eye_info(module_accessor);
                SoundModule::play_se(module_accessor,Hash40::new("se_cloud_special_l03"),true,false,false,false,enSEType(0));
            }
            //let limit_break_clear_frame = WorkModule::get_param_int(module_accessor,hash40("param_special_lw"),hash40("limit_break_clear_frame"));
            WorkModule::set_int(module_accessor,i32::MAX,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_CLEAR_FRAME);
        }
        else {
            if tot <= 0.0 {
                tot = 0.0;
            }
            if effect != 0 {
                EffectModule::set_frame(module_accessor,effect as u32,tot);
            }
            WorkModule::set_float(module_accessor,tot,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
            if unk != 0 {
                WorkModule::on_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE);
            }
        }
    }
}

/*#[skyline::hook(offset = 0x8dd8f0, inline)]
pub unsafe fn limit_break_off_fix(ctx: &mut InlineCtx) {
    let module_accessor: &mut BattleObjectModuleAccessor = std::mem::transmute(*ctx.registers[23].x.as_ref());
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_RELEASE_LIMIT) == false {
        *ctx.registers[0].w.as_mut() = 0;
    }
    else {
        *ctx.registers[0].w.as_mut() = 1;
    }
}*/

#[skyline::from_offset(0x6dd280)]
pub fn waza_customize(lua_module: u64,status: i32,customize_to: i32);

#[skyline::hook(offset=0x8dd7d0)]
pub unsafe fn cloud_waza_setup(unk: u8, battle_object: *mut BattleObject) {
    let module_accessor = (*(battle_object)).module_accessor;
    if WorkModule::is_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_WAZA) {
        let lua_module = *(module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        waza_customize(lua_module,*FIGHTER_STATUS_KIND_SPECIAL_N,*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2);
        waza_customize(lua_module,*FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2);
        WorkModule::off_flag(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SET_WAZA);
    }
    if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL) {
        let lua_module = *(module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        waza_customize(lua_module,*FIGHTER_STATUS_KIND_SPECIAL_LW,*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1);
    }
    original!()(unk,battle_object)
}

#[fighter_reset]
pub fn cloud_status_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_CLOUD {
            fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(cloud_move_customizer as *const () as _));
        }
    }
}

/*#[agent_reset]
pub fn cloud_wave_status_init(weapon: &mut L2CFighterBase) {
    unsafe {
        if utility::get_kind(&mut *weapon.module_accessor) == *WEAPON_KIND_CLOUD_WAVE {
            weapon.sv_set_status_func(WEAPON_CLOUD_WAVE_STATUS_KIND_METEOR.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(meteor_main as *const () as *mut c_void));
            weapon.sv_set_status_func(WEAPON_CLOUD_WAVE_STATUS_KIND_METEOR_BLAST.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(meteor_blast_main as *const () as *mut c_void));
            weapon.sv_set_status_func(WEAPON_CLOUD_WAVE_STATUS_KIND_METEOR_BLAST.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(meteor_blast_end as *const () as *mut c_void));
        }
    }
}*/

unsafe extern "C" fn cloud_move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    let limit_level = WorkModule::get_int(fighter.module_accessor, FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_LEVEL);
    if limit_level == LIMIT_BREAK_LEVEL_1 {
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(crossslash_pre as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(crossslash_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(crossslash_end as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), &mut *(empty as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), &mut *(empty as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(crossslash_pre as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(crossslash2_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(crossslash2_end as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(crossslash_pre as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(crossslash3_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_CROSS_SLASH_3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(crossslash3_end as *const () as *mut c_void));
        }
        else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(braver_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(braver_end as *const () as *mut c_void));
        }
    }
    else if limit_level == LIMIT_BREAK_LEVEL_2 {
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(climhazzard_pre as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(climhazzard_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(climhazzard_end as *const () as *mut c_void));
        }
    }
    else if limit_level == LIMIT_BREAK_LEVEL_3 {
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(meteorrain_pre as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(meteorrain_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(meteorrain_end as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), &mut *(empty as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), &mut *(empty as *const () as *mut c_void));
        }
        if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(finishing_touch_main as *const () as *mut c_void));
            fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(finishing_touch_end as *const () as *mut c_void));
        }
    }
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(special_n_pre as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_n_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(empty as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(), &mut *(special_n_init as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_N.into(), LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(), &mut *(special_n_exec as *const () as *mut c_void));
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_S.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_s_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_s2_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_s3_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(special_s3_end as *const () as *mut c_void));
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), &mut *(braver_pre as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_lw_stance_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), &mut *(empty as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), &mut *(special_lw_strike_main as *const () as *mut c_void));
        fighter.sv_set_status_func(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END.into(), LUA_SCRIPT_STATUS_FUNC_CHECK_DAMAGE.into(), &mut *(empty as *const () as *mut c_void));
    }
    return L2CValue::I32(0)
}

pub unsafe fn empty(_fighter: &mut L2CFighterCommon) -> L2CValue {
    return L2CValue::I32(0)
}