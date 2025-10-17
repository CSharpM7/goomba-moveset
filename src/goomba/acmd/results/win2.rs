use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_win2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("hat"), false);

        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
        //let goomba_tower = get_article_boma(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
        //tower::init(goomba_tower);
    }
    
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(agent.module_accessor, Hash40::new("head"), &mut pos,false); 
        let eff = EffectModule::req(
            agent.module_accessor,
            Hash40::new("sys_turn_smoke"),
            &Vector3f{x:pos.x,y:0.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            0.8,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(agent.module_accessor, eff, -1);
    }
}

unsafe extern "C" fn effect_win2(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_step_left_m"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_step_right_m"));
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_common_slip_01"));
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_pichu_landing_04")); //se_common_falldown_05
    }
    
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_win2", game_win2, Priority::Default);
	agent.acmd("effect_win2", effect_win2, Priority::Default);
	agent.acmd("sound_win2", sound_win2, Priority::Default);
}