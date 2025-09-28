use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if is_kuribo_rpg(agent.module_accessor) {
            let lr = PostureModule::lr(agent.module_accessor);
            let new_flip = lr > 0.0;
    
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pichu_facen_mouth"), !new_flip);
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("pichu_facen_mouthflip"), new_flip);
        }
    }
}

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}
unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_pichu_dash_start"), 20);
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_pichu_step_right_m"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_pichu_step_left_m"));
    }
}

unsafe extern "C" fn expression_dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn effect_run(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, -1, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 1, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
            
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_run(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_pichu_step_right_m"));
        }
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_pichu_step_left_m"));
        }
                
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_run(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn game_runbrake(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_TURN_RUN);
    }
}

unsafe extern "C" fn effect_runbrake(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        //macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_runbrake(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_dash_stop"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_pichu_dash_stop"), 30);
    }
}

unsafe extern "C" fn game_turnrun(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_STOP);
    }
}

unsafe extern "C" fn effect_turnrun(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7.5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -7.5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_turnrun(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_dash_stop"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_pichu_dash_stop"), 30);
    }
}

unsafe extern "C" fn expression_turnrun(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x23c33f3bdc));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_turn", game_turn, Priority::Default);

	agent.acmd("game_dash", game_dash, Priority::Default);
	agent.acmd("game_turndash", game_turndash, Priority::Default);
	agent.acmd("effect_dash", effect_dash, Priority::Default);
	agent.acmd("effect_turndash", effect_dash, Priority::Default);
	agent.acmd("sound_dash", sound_dash, Priority::Default);
	agent.acmd("sound_turndash", sound_dash, Priority::Default);
	agent.acmd("expression_dash", expression_dash, Priority::Default);
	agent.acmd("expression_turndash", expression_dash, Priority::Default);

	agent.acmd("effect_run", effect_run, Priority::Default);
	agent.acmd("sound_run", sound_run, Priority::Default);
	agent.acmd("expression_run", expression_run, Priority::Default);

    /*
	agent.acmd("game_runbrakel", game_runbrake, Priority::Default);
	agent.acmd("game_runbraker", game_runbrake, Priority::Default);
	agent.acmd("effect_runbrakel", effect_runbrake, Priority::Default);
	agent.acmd("effect_runbraker", effect_runbrake, Priority::Default);
	agent.acmd("sound_runbrakel", sound_runbrake, Priority::Default);
	agent.acmd("sound_runbraker", sound_runbrake, Priority::Default);
    
	agent.acmd("game_turnrun", game_turnrun, Priority::Default);
	agent.acmd("effect_turnrun", effect_turnrun, Priority::Default);
	agent.acmd("sound_turnrun", sound_turnrun, Priority::Default);
	agent.acmd("expression_turnrun", expression_turnrun, Priority::Default); 
    */
}
