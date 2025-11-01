use crate::imports::imports_agent::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Appeal();
    return to_return;
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_3"), 0.0, 1.0, false, 0.0, false, false);
    
    //return fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _));
}

unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_3_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return 0.into();
}

pub unsafe extern "C" fn common_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_PICHU && status != *FIGHTER_STATUS_KIND_STANDBY {
        if LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL) {
            /*
            println!("status: {status}");
            let parent = LinkModule::get_parent_object_id(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL) as u32;
            let parent_boma = sv_battle_object::module_accessor(parent);
            let target_x = WorkModule::get_float(parent_boma, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
            println!("Parent Target: {target_x}");
             */
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

pub unsafe extern "C" fn dev_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        println!("Status: {status}");
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    /*
    agent.acmd("game_entryr", game_entryr, Priority::High);
    agent.acmd("game_entryl", game_entryr, Priority::High);
    agent.acmd("game_win1", game_entryr, Priority::High);
    agent.acmd("game_win2", game_entryr, Priority::High);
    agent.acmd("game_win3", game_entryr, Priority::High);
    */
    agent.on_line(Main, dev_frame);
    
    smashline::Agent::new("fighter")
    .on_line(Main, common_frame)
    .install(); 
}