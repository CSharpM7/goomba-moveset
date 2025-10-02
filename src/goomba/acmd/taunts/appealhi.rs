use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    FT_MOTION_RATE_RANGE(agent,6.0,26.0,12.0);
    frame(agent.lua_state_agent, 26.0);
    //FT_MOTION_RATE(agent,1.0);
    FT_MOTION_RATE_RANGE(agent,26.0,33.0,12.0);

    frame(agent.lua_state_agent, 33.0);
    FT_MOTION_RATE_RANGE(agent,33.0,45.0,15.0);
    frame(agent.lua_state_agent, 45.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_rise"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_squat"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_appeal_h01"));
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_appeal01"));
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing02"));
    }
}

unsafe extern "C" fn expression_appealhir(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 27, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 27, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_appealhir", game_appealhir, Priority::Default);
	agent.acmd("game_appealhil", game_appealhir, Priority::Default);
	agent.acmd("effect_appealhir", effect_appealhir, Priority::Default);
	agent.acmd("effect_appealhil", effect_appealhir, Priority::Default);
	agent.acmd("sound_appealhir", sound_appealhir, Priority::Default);
	agent.acmd("sound_appealhil", sound_appealhir, Priority::Default);
	agent.acmd("expression_appealhir", expression_appealhir, Priority::Default);
	agent.acmd("expression_appealhil", expression_appealhir, Priority::Default);
}
