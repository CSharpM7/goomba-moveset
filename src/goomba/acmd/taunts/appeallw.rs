use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
    }
}
unsafe extern "C" fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_rise"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_left_l"));
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_right_l"));
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_left_l"));
    }
    frame(agent.lua_state_agent, 83.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_right_l"));
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_step_left_m"));
    }
}

unsafe extern "C" fn expression_appeallwr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 83.0);
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
