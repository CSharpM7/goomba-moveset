use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_rise"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_left_l"));
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_right_l"));
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing03"));
    }
}

unsafe extern "C" fn expression_appeallwr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	//agent.acmd("game_appeallwr", game_appeallwr, Priority::Default);
	//agent.acmd("game_appeallwl", game_appeallwr, Priority::Default);
	agent.acmd("effect_appeallwr", effect_appeallwr, Priority::Default);
	agent.acmd("effect_appeallwl", effect_appeallwr, Priority::Default);
	agent.acmd("sound_appeallwr", sound_appeallwr, Priority::Default);
	agent.acmd("sound_appeallwl", sound_appeallwr, Priority::Default);
	agent.acmd("expression_appeallwr", expression_appeallwr, Priority::Default);
	agent.acmd("expression_appeallwl", expression_appeallwr, Priority::Default);
}
