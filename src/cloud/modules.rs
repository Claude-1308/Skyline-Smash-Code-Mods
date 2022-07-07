pub mod CloudModule {
    
    use smash::lib::lua_const::*;
    use smash::lua2cpp::L2CAgentBase;
    use smash::phx::*;
    use smash::hash40;
    use smash::app::lua_bind::*;
    use smash::app::*;
    use smash_script::macros::*;
    use std::collections::HashMap;
    
    use crate::moveset::*;

    pub unsafe fn trail_generator(fighter: &mut L2CAgentBase, trail_length: u64) {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_blue"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        else {
            /*if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_red"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            }*/
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), trail_length, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }

    pub unsafe fn flare_generator(fighter: &mut L2CAgentBase, cam_offset: f32) {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        /*let (mut x, mut y) = (0.0,0.0);
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi2_fall") {
            x = -0.04;
            y = 0.1;
        }*/
        if WorkModule::is_flag(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
        }
        else {
            /*if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_FIRE {
                EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), x, y, 0, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
                LAST_EFFECT_SET_COLOR(fighter,1,0.55,0);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ICE {
                EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB)), Hash40::new("haver"), x, y, 0, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_ELEC {
                EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), x, y, 0, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
                LAST_EFFECT_SET_COLOR(fighter,1,1,0);
            }
            else if WorkModule::get_int(module_accessor,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE) == MAGIC_TYPE_WIND {
                EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), x, y, 0, 0, 0, 0, 1, true);
                LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
                LAST_EFFECT_SET_COLOR(fighter,0.5,1,0);
            }*/
            EFFECT_FOLLOW_WORK(fighter, Hash40::new_raw(WorkModule::get_int64(module_accessor,*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE)), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, cam_offset);
        }
    }

    /*pub unsafe fn speedline_recolor(magic_type: i32) -> (f32,f32,f32) {
        return match magic_type {
            MAGIC_TYPE_FIRE => (1.0,0.55,0.0),
            MAGIC_TYPE_ICE => (0.0,1.0,1.0),
            MAGIC_TYPE_ELEC => (1.0,1.0,0.0),
            MAGIC_TYPE_WIND => (0.5,1.0,0.0),
            _ => (0.0,1.0,1.0),
        };
    }

    pub unsafe fn materia_sound(agent: &mut L2CAgentBase, sound_1: u64, sound_2: u64) {
        let rng = sv_math::rand(hash40("cloud_wave"),100);
        if rng < 50 {
            PLAY_SE(agent,Hash40::new_raw(sound_1));
        }
        else {
            PLAY_SE(agent,Hash40::new_raw(sound_2));
        }
    }

    pub unsafe fn materia_change(module_accessor: &mut BattleObjectModuleAccessor, materia: i32) {
        WorkModule::set_int(module_accessor,materia,FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_MAGIC_TYPE);
    }*/

    pub unsafe fn change_sword(module_accessor: &mut BattleObjectModuleAccessor, sword: u64) {
        let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        if color_id % 2 == 0 && SWORDS[color_id as usize] {
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new_raw(sword),true);
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bastar_sword_r"),false);
        }
    }

    pub unsafe fn reset_swords(module_accessor: &mut BattleObjectModuleAccessor) {
        if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
            let list = [hash40("iron_sword_r"), hash40("twin_stingers_r"), hash40("hardedge_r"), hash40("mythril_saber_r")];
            for sword in list.iter() {
                ModelModule::set_mesh_visibility(module_accessor,Hash40::new_raw(*sword),false);
            }
            ModelModule::set_mesh_visibility(module_accessor,Hash40::new("bastar_sword_r"),true);
        }
    }

    pub unsafe fn get_params(param_type: &str, param: &str) -> f32 {
        let json_string = std::fs::read_to_string("mods:/fighter/cloud/param/param.json").unwrap();
        let params: HashMap<String, HashMap<String, f32>> = serde_json::from_str(json_string.as_str()).unwrap();
        return *params.get(param_type).map(|private| private.get(param)).flatten().unwrap();
    }
}