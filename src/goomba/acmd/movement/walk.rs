use crate::imports::imports_acmd::*;

unsafe extern "C" fn effect_walkslow(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_walkslow(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_right_s"));
        }
        frame(agent.lua_state_agent, 53.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_left_s"));
        }
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_walkslow(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 99.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_walkmiddle(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }

        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_walkmiddle(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_right_m"));
        }
        frame(agent.lua_state_agent, 58.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_left_m"));
        }
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_walkmiddle(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 59.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_walkfast(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 3, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_walkfast(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_right_m"));
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_dedede_step_left_m"));
        }
            
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_walkfast(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
                
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("effect_walkslow", effect_walkslow, Priority::Default);
	agent.acmd("sound_walkslow", sound_walkslow, Priority::Default);
	agent.acmd("expression_walkslow", expression_walkslow, Priority::Default);
    
	agent.acmd("effect_walkmiddle", effect_walkmiddle, Priority::Default);
	agent.acmd("sound_walkmiddle", sound_walkmiddle, Priority::Default);
	agent.acmd("expression_walkmiddle", expression_walkmiddle, Priority::Default);

	agent.acmd("effect_walkfast", effect_walkfast, Priority::Default);
	agent.acmd("sound_walkfast", sound_walkfast, Priority::Default);
	agent.acmd("expression_walkfast", expression_walkfast, Priority::Default);
}
