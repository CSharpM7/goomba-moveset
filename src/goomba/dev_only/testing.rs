use crate::imports::imports_agent::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Appeal();
    //fighter.change_status(FIGHTER_STATUS_KIND_ENTRY.into(),false.into());
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);
    let eff = if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {Hash40::new("goomba_magic_bright")}
    else if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_r") {Hash40::new("goomba_magic_bright2")}
    else {Hash40::new("goomba_magic_dark")};

    //use bright2

    //return to_return;
    let pos = *PostureModule::pos(fighter.module_accessor);
    EffectModule::req(
        fighter.module_accessor,
        eff,
        &Vector3f{x:pos.x,y:pos.y+8.0,z:pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        1.1,
        0,
        -1,
        false,
        0
    );
    return fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _));
}

unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut r = (frame*10.0).sin();
    let mut g = (frame*10.0).cos();
    let mut b = (frame*10.0).tan();
    r = (r+1.0)*0.5;
    g= (g+1.0)*0.5;
    b= (b+1.0)*0.5;
    ColorBlendModule::set_main_color(fighter.module_accessor, &Vector4f{ x: r, y: g, z: b, w: 0.7 }, 
        &Vector4f{ x: 1.0, y: 0.5, z: 0.5, w: 0.5 }, 
    1.0, 0.5, 10, true);

    return fighter.status_Appeal_Main();
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_2_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return 0.into();
}

pub unsafe extern "C" fn common_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if status == *FIGHTER_STATUS_KIND_PIKACHU_FINAL_DAMAGE_FLY {
        println!("pika damage");
        if LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL) {
            let parent = LinkModule::get_parent_object_id(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL) as u32;
            let parent_boma = sv_battle_object::module_accessor(parent);
            let target_x = WorkModule::get_float(parent_boma, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
            println!("Parent Target: {target_x}");
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
    //agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    /*
    agent.acmd("game_entryr", game_entryr, Priority::High);
    agent.acmd("game_entryl", game_entryr, Priority::High);
    agent.acmd("game_win1", game_entryr, Priority::High);
    agent.acmd("game_win2", game_entryr, Priority::High);
    agent.acmd("game_win3", game_entryr, Priority::High);
    */
    let agent = &mut smashline::Agent::new("fighter")
    .on_line(Main, common_frame)
    .install(); 
}