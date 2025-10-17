use crate::imports::imports_status::*;

unsafe extern "C" fn entry_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, false, -1);
    //let block = get_article_boma(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
    //accessories::init_block(block);
    0.into()
}

unsafe extern "C" fn entry_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_Entry()
}

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_Appeal()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_ENTRY, entry_init);
    agent.status(End, *FIGHTER_STATUS_KIND_ENTRY, entry_end);

    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);
}