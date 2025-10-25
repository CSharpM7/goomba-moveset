use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 99.0);
    if macros::is_excute(agent) {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(agent.module_accessor, Hash40::new("hip"), &mut pos,false); 
        let eff = EffectModule::req(
            agent.module_accessor,
            Hash40::new("sys_down_smoke"),
            &Vector3f{x:pos.x,y:0.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.1,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(agent.module_accessor, eff, -1);
    }
    frame(agent.lua_state_agent, 122.0);
    if macros::is_excute(agent) {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(agent.module_accessor, Hash40::new("footr"), &mut pos,false); 
        let eff = EffectModule::req(
            agent.module_accessor,
            Hash40::new("sys_landing_smoke_s"),
            &Vector3f{x:pos.x,y:0.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.25,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(agent.module_accessor, eff, -1);
        EffectModule::set_rate(agent.module_accessor, eff, 1.5);
    }
}

unsafe extern "C" fn effect_win3(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_landing01"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_landing01"));
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_rise"));
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_step_right_m"));
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_appeal_h01"));
    }
    frame(agent.lua_state_agent, 123.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_landing01"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_win3", game_win3, Priority::Default);
	agent.acmd("effect_win3", effect_win3, Priority::Default);
	agent.acmd("sound_win3", sound_win3, Priority::Default);
}
