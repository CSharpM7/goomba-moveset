use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 47.0);
    FT_MOTION_RATE(agent,0.5);
    frame(agent.lua_state_agent, 60.0);
    FT_MOTION_RATE(agent,0.375);
    frame(agent.lua_state_agent, 81.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_renga_break"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_ALPHA(agent,0.7);
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_entryr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_appear01"));
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pichu_appeal02"));
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing01"));
    }
}

unsafe extern "C" fn expression_entryr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //VisibilityModule::set_whole(agent.module_accessor, false);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false,0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
        //slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ItemModule::set_attach_item_visibility(agent.module_accessor, true,0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        //VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), true);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        //slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_entryr", game_entryr, Priority::Default);
	agent.acmd("game_entryl", game_entryr, Priority::Default);
	agent.acmd("effect_entryl", effect_entryr, Priority::Default);
	agent.acmd("effect_entryr", effect_entryr, Priority::Default);
	agent.acmd("sound_entryr", sound_entryr, Priority::Default);
	agent.acmd("sound_entryl", sound_entryr, Priority::Default);
	agent.acmd("expression_entryr", expression_entryr, Priority::Default);
	agent.acmd("expression_entryl", expression_entryr, Priority::Default);
}