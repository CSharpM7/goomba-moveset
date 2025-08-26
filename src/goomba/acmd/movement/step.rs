use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_steppose(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 2.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
        }
    }
}

unsafe extern "C" fn effect_steppose(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_step_jump"), Hash40::new("top"), 0, 5, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_hit_critical"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        //macros::EFFECT(agent, Hash40::new("sys_piyo"), Hash40::new("top"), 0, 5, 2, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_steppose(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_appear02"));
    }
}

unsafe extern "C" fn expression_steppose(agent: &mut L2CAgentBase) {
    let animcmd: &mut L2CFighterAnimcmdExpressionCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdExpressionCommon::expression_StepPoseRumble(animcmd);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_steppose", game_steppose, Priority::Default);
	agent.acmd("effect_steppose", effect_steppose, Priority::Default);
	agent.acmd("sound_steppose", sound_steppose, Priority::Default);
	agent.acmd("expression_steppose", expression_steppose, Priority::Default);

	agent.acmd("game_stepposeair", game_steppose, Priority::Default);
	agent.acmd("effect_stepposeair", effect_steppose, Priority::Default);
	agent.acmd("sound_stepposeair", sound_steppose, Priority::Default);
}
