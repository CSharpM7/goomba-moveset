use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_jumpaerialfront(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_jumpaerialfront(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dedede_jump02"));
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn effect_jumpaerialback(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_jumpaerialback(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dedede_jump02"));
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}
pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("effect_jumpaerialfront", effect_jumpaerialfront, Priority::Default);
	agent.acmd("sound_jumpaerialfront", sound_jumpaerialfront, Priority::Default);
	agent.acmd("effect_jumpaerialback", effect_jumpaerialback, Priority::Default);
	agent.acmd("sound_jumpaerialback", sound_jumpaerialback, Priority::Default);
}
