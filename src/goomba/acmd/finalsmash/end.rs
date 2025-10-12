use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_final2(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn effect_final2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_final2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if agent.is_grounded() {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_pichu_landing02"));
    }
}

unsafe extern "C" fn expression_final2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_final2", game_final2, Priority::Default);
	agent.acmd("game_finalair2", game_final2, Priority::Default);
	agent.acmd("effect_final2", effect_final2, Priority::Default);
	agent.acmd("effect_finalair2", effect_final2, Priority::Default);
	agent.acmd("sound_final2", sound_final2, Priority::Default);
	agent.acmd("sound_finalair2", sound_final2, Priority::Default);
	agent.acmd("expression_final2", expression_final2, Priority::Default);
	agent.acmd("expression_finalair2", expression_final2, Priority::Default);
}
