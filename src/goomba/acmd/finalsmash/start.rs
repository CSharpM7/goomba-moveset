use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 9.0, 45.0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            REQ_FINAL_START_CAMERA(agent,Hash40::new("d04final.nuanmb"), false);
            macros::FT_START_CUTIN(agent);
        }
        else {
            //PostureModule::scale(agent.module_accessor, 3, 0);
            //0x103370(1867797157, 2.2);
            CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, 2.2, 0.0, 0.0);
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_pichu_final"), false, false, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("goomba_magic_bright2"), Hash40::new("throw"), 0, 10, 0, 0, 0, 0, 1.7, false);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("goomba_magic_bright2"),false,false);
        if agent.is_grounded() {
            //macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final02"));
    }
}

unsafe extern "C" fn expression_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        app::FighterUtil::flash_eye_info(agent.module_accessor);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}


pub fn install(agent: &mut Agent) {
	agent.acmd("game_final", game_final, Priority::Default);
	agent.acmd("game_finalair", game_final, Priority::Default);
	agent.acmd("effect_final", effect_final, Priority::Default);
	agent.acmd("effect_finalair", effect_final, Priority::Default);
	agent.acmd("sound_final", sound_final, Priority::Default);
	agent.acmd("sound_finalair", sound_final, Priority::Default);
	agent.acmd("expression_final", expression_final, Priority::Default);
	agent.acmd("expression_finalair", expression_final, Priority::Default);
}
