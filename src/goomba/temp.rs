use crate::imports::imports_acmd::*;
use crate::imports::imports_agent::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        println!("Gen?");
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, false, -1);
        if ArticleModule::is_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES) {
            println!("Gened");
            ArticleModule::change_motion(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, Hash40::new("s"), false, -1.0);
        }
    }
}

pub unsafe extern "C" fn landing_air_main(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let mot = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let original = fighter.status_LandingAttackAir();
    let mut motion_rate = 1.0;
    let landing_motion = match mot {
        0xc3a4e2597 => hash40("landing_air_n"),
        0xc3495ada5 => hash40("landing_air_f"),
        0xc33f869bc => hash40("landing_air_b"),
        0xdde67d935 => hash40("landing_air_hi"),
        0xd40042152 => hash40("landing_air_lw"),
        _ => hash40("landing_heavy")
    };
    let landing_frame = match mot {
        0xc3a4e2597 => 7.0,//n
        0xc3495ada5 => 12.0,//f
        0xc33f869bc => 18.0,//b
        0xdde67d935 => 11.0,//hi
        0xd40042152 => 22.0,//lw
        _ => 20.0
    };
    if landing_frame != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(landing_motion.into(),landing_frame.into()).get_f32();
    }
    //println!("Landing lag: {landing_frame} Motion Rate: {motion_rate}");
    MotionModule::set_rate(fighter.module_accessor,motion_rate.max(1.0));
    return original;
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main,*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,landing_air_main);
    //agent.acmd("game_attack11", game_attack11,Priority::Default);
    //agent.acmd("game_attackairn", game_attack11,Priority::Default);
}