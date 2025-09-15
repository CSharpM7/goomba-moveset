use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,9.0,5.0);
    frame(agent.lua_state_agent, 9.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 9.0, 83, 110, 0, 40, 5.8, 4.9, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 9.0, 83, 110, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    FT_MOTION_RATE_RANGE(agent,15.0,31.0,8.0);
    frame(agent.lua_state_agent, 31.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc3"), Hash40::new("pichu_tail_arc3"), Hash40::new("top"), 1, 5, 0, 10, -40, -110, 1, true, *EF_FLIP_YZ);
        //macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 6, -0.5, 32, -75, -138, 1.1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
        common_effect_color(agent);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackhard_s01"));
        PLAY_VC(agent, Hash40::new("vc_pichu_attack06"), 0.5);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attackhi3", game_attackhi3, Priority::Default);
	agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Default);
	agent.acmd("sound_attackhi3", sound_attackhi3, Priority::Default);
	agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Default);
}
