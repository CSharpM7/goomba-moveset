use crate::imports::imports_status::*;

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_Appeal()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}