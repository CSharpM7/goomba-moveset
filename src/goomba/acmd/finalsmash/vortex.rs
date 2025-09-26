use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_activate(agent: &mut L2CAgentBase) {
    //if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_final_sphere_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    //}
}
unsafe extern "C" fn effect_deactivate(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}
unsafe extern "C" fn sound_normal(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
        }
        wait(agent.lua_state_agent, 9.0+10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
        }
        wait(agent.lua_state_agent, 8.0+10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
        }
        wait(agent.lua_state_agent, 10.0+10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
        }
        wait(agent.lua_state_agent, 9.0+10.0);
    }
}
pub fn install(agent: &mut Agent) {
    let agent = &mut smashline::Agent::new("pichu_vortex");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

	agent.acmd("sound_normal", empty_acmd, Priority::Default);
	agent.acmd("effect_activate", effect_activate, Priority::Default);
	agent.acmd("effect_deactivate", empty_acmd, Priority::Default);
    agent.install();
}