use crate::imports::imports_agent::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Appeal();
    
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry_r"), false, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);

    return to_return;
    //fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _))
}
unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_1_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}
pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
}