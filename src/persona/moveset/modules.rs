pub mod JackModule {
    
    use smash::lib::lua_const::*;
    use smash::lua2cpp::L2CAgentBase;
    use smash::phx::*;
    use smash::hash40;
    use smash::app::lua_bind::*;
    use smash::app::*;
    use smash_script::macros::*;
    use std::collections::HashMap;
    
    use crate::moveset::*;

    use parking_lot::RwLock;
    use lazy_static::*;
    
    lazy_static! {
        pub static ref JACK_PARAMS: RwLock<HashMap<String, HashMap<String, f32>>> = RwLock::new({
            let json_string = std::fs::read_to_string("sd:/ultimate/mods/c01-7 Kasumi + personas/fighter/jack/param/param.json").unwrap();
            serde_json::from_str(json_string.as_str()).unwrap()
        });
    }

    pub unsafe fn decide_box_property(persona_kind: i32) -> (u64,i32) {
        match persona_kind {
            PERSONA_KIND_ARSENE | PERSONA_KIND_SATANAEL | PERSONA_KIND_RAOUL => {
                return (hash40("collision_attr_curse_poison"),*COLLISION_SOUND_ATTR_FIRE);
            },
            PERSONA_KIND_HACHIMAKI => {
                return (hash40("collision_attr_elec"),*COLLISION_SOUND_ATTR_ELEC);
            },
            PERSONA_KIND_AME => {
                return (hash40("collision_attr_ice"),*COLLISION_SOUND_ATTR_FREEZE);
            },
            PERSONA_KIND_SETANTA => {
                return (hash40("collision_attr_aura"),*COLLISION_SOUND_ATTR_FIRE);
            },
            PERSONA_KIND_OKUNINUSHI => {
                return (hash40("collision_attr_purple"),*COLLISION_SOUND_ATTR_FIRE);
            },
            PERSONA_KIND_ORPHEOUS => {
                return (hash40("collision_attr_fire"),*COLLISION_SOUND_ATTR_FIRE);
            },
            PERSONA_KIND_KAGUYA | PERSONA_KIND_CENDRILLON | PERSONA_KIND_VANADIS | PERSONA_KIND_ELLA => {
                return (hash40("collision_attr_magic"),*COLLISION_SOUND_ATTR_MAGIC);
            },
            _ => {
                return (hash40("collision_attr_cutup"),*COLLISION_SOUND_ATTR_CUTUP);
            },
        }
    }

    pub unsafe fn sumi_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool { //to determine if it is violet or joker
        let color_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        if SUMI.contains(&color_id) {
            return true;
        }
        else {
            return false;
        }
    }

    pub unsafe fn get_params(param_type: &str, param: &str) -> f32 {
        return *JACK_PARAMS.read().get(param_type).and_then(|private| private.get(param)).unwrap();
    }
      
    pub unsafe fn reload_params() {
        *JACK_PARAMS.write() = {
            let json_string = std::fs::read_to_string("sd:/ultimate/mods/c01-7 Kasumi + personas/fighter/jack/param/param.json").unwrap();
            serde_json::from_str(json_string.as_str()).unwrap()
        };
    }
}