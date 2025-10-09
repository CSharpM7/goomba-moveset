use crate::imports::imports_agent::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Appeal();
    //fighter.change_status(FIGHTER_STATUS_KIND_ENTRY.into(),false.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);

    return to_return;
    return fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _));
}

unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_2_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

pub unsafe extern "C" fn common_frame(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if speed_x.abs() > 0.5 {
            println!("Speed X: {speed_x}");
        }
    }
}

unsafe extern "C" fn game_entryr(agent: &mut L2CAgentBase) {
    use crate::imports::imports_acmd::*;
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
        //let goomba_tower = get_article_boma(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
        //tower::init(goomba_tower);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
        //let goomba_tower = get_article_boma(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
        //tower::init(goomba_tower);
    }
    frame(agent.lua_state_agent, 47.0);
    FT_MOTION_RATE(agent,0.5);
    frame(agent.lua_state_agent, 60.0);
    FT_MOTION_RATE(agent,0.375);
    frame(agent.lua_state_agent, 81.0);
    FT_MOTION_RATE(agent,1.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    agent.acmd("game_entryr", game_entryr, Priority::High);
    agent.acmd("game_entryl", game_entryr, Priority::High);
    agent.acmd("game_win1", game_entryr, Priority::High);
    agent.acmd("game_win2", game_entryr, Priority::High);
    agent.acmd("game_win3", game_entryr, Priority::High);
    /*
    let agent = &mut smashline::Agent::new("fighter")
    .on_line(Main, common_frame)
    .install(); 
    */
}