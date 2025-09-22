use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_steam3"), Hash40::new("head"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 4, 0, 90, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 72.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_steam3"), false, true);
    }
}

unsafe extern "C" fn sound_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing04_win03"));
        macros::STOP_SE(agent, Hash40::new("se_pichu_special_n01_win03"));
    }
    frame(agent.lua_state_agent, 104.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_rise_win03"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("effect_win3", effect_win3, Priority::Default);
	agent.acmd("sound_win3", sound_win3, Priority::Default);
}
