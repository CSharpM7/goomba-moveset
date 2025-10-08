use crate::imports::imports_acmd::*;

unsafe extern "C" fn sound_squatwait1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let req_se = WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_SQUAT_FLAG_REQUEST_SQUAT_SE);
        let was_lw3 = StatusModule::prev_status_kind(agent.module_accessor, 0) == *FIGHTER_STATUS_KIND_ATTACK_LW3;
        if req_se && !was_lw3 {
            macros::PLAY_SE(agent, Hash40::new("se_pichu_squat"));
        }
    }
}

unsafe extern "C" fn sound_jumpfront(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_jump"));
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
            macros::PLAY_SE(agent, Hash40::new("se_pichu_jump01"));
        }
    }
    wait(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
}

unsafe extern "C" fn sound_jumpback(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_jump"));
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) {
            macros::PLAY_SE(agent, Hash40::new("se_pichu_jump01"));
        }
    }
    wait(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
}

unsafe extern "C" fn sound_jumpaerialback(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_jump02"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
    /*
    wait(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    } 
    */
}

pub fn install(agent: &mut Agent) {
	agent.acmd("sound_squatwait1", sound_squatwait1, Priority::Default);
	agent.acmd("sound_jumpfront", sound_jumpfront, Priority::Default);
	agent.acmd("sound_jumpback", sound_jumpback, Priority::Default);
	agent.acmd("sound_jumpaerialback", sound_jumpaerialback, Priority::Default);
}