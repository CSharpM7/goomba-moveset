use crate::imports::imports_status::*;

pub unsafe extern "C" fn specialn_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let exists = ArticleModule::is_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL);
    let can_gen = !exists && ArticleModule::is_generatable(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL);
    println!("Shell exists: {exists} Can gen: {can_gen}");
    WorkModule::set_flag(fighter.module_accessor, can_gen, FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN);
    0.into()
}

pub unsafe extern "C" fn specialn_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let param = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    //fighter.sub_remove_exist_article_at_status_end(param.into(), FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL.into());
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_N_FLAG_SHOOT) {
        ArticleModule::change_status_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, REDSHELL_STATUS_KIND_FURAFURA);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_init);
	agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, specialn_end);
}