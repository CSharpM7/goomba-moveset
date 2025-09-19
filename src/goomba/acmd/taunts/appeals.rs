use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_appealsr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

        ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, false, -1);
        let book = get_article_boma(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
        accessories::init_book(book);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_appealsr(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_appeal_l01"));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_pichu_appeal_l01"));
    }
}

unsafe extern "C" fn expression_appealsr(agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_appealsr", game_appealsr, Priority::Default);
	agent.acmd("game_appealsl", game_appealsr, Priority::Default);
	agent.acmd("effect_appealsr", effect_appealsr, Priority::Default);
	agent.acmd("effect_appealsl", effect_appealsr, Priority::Default);
	agent.acmd("sound_appealsr", sound_appealsr, Priority::Default);
	agent.acmd("sound_appealsl", sound_appealsr, Priority::Default);
	agent.acmd("expression_appealsr", expression_appealsr, Priority::Default);
	agent.acmd("expression_appealsl", expression_appealsr, Priority::Default);
}
