use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_goomba1win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_LAND, TOWER_INSTANCE_INT_STEP);
    }
    frame(agent.lua_state_agent, 101.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_CLOUD, TOWER_INSTANCE_INT_STEP);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_COIN, TOWER_INSTANCE_INT_STEP);
    }
}

unsafe extern "C" fn game_goomba2win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_LAND, TOWER_INSTANCE_INT_STEP);
    }
    frame(agent.lua_state_agent, 111.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_CLOUD, TOWER_INSTANCE_INT_STEP);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, TOWER_STEP_COIN, TOWER_INSTANCE_INT_STEP);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_entryl", empty_acmd, Priority::Default);
	agent.acmd("game_entryr", empty_acmd, Priority::Default);
	agent.acmd("effect_entryl", empty_acmd, Priority::Default);
	agent.acmd("effect_entryr", empty_acmd, Priority::Default);
	agent.acmd("sound_entryl", empty_acmd, Priority::Default);
	agent.acmd("sound_entryr", empty_acmd, Priority::Default);

	agent.acmd("game_goomba1win2", game_goomba1win2, Priority::Default);
	agent.acmd("effect_goomba1win2", empty_acmd, Priority::Default);
	agent.acmd("sound_goomba1win2", empty_acmd, Priority::Default);

	agent.acmd("game_goomba2win2", game_goomba2win2, Priority::Default);
	agent.acmd("effect_goomba2win2", empty_acmd, Priority::Default);
	agent.acmd("sound_goomba2win2", empty_acmd, Priority::Default);
}