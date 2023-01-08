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
}