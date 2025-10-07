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

pub fn install(agent: &mut Agent) {
	agent.acmd("sound_squatwait1", sound_squatwait1, Priority::Default);
}
