use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_jump03"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_appeal02"));
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing02"));
    }
}

unsafe extern "C" fn expression_appealhir(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	//agent.acmd("game_appealhir", game_appealhir, Priority::Default);
	//agent.acmd("game_appealhil", game_appealhir, Priority::Default);
	agent.acmd("effect_appealhir", effect_appealhir, Priority::Default);
	agent.acmd("effect_appealhil", effect_appealhir, Priority::Default);
	agent.acmd("sound_appealhir", sound_appealhir, Priority::Default);
	agent.acmd("sound_appealhil", sound_appealhir, Priority::Default);
	agent.acmd("expression_appealhir", expression_appealhir, Priority::Default);
	agent.acmd("expression_appealhil", expression_appealhir, Priority::Default);
}
