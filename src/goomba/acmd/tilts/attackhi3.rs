use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 105, 50, 0, 55, 5.4, 0.0, 4.8, 3.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 100, 50, 0, 55, 5.4, 0.0, 4.8, 1.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 1.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 1.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,7.0,13.0,5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 6.0, 103, 50, 0, 55, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 0.0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 13.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 2, 0, -90, 0, 0, 1.0, true, *EF_FLIP_YZ, 0.3);
        common_effect_color(agent);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("goomba_bite_line2"), Hash40::new("head"), 0.0, 2.0, 0.5, 0, 10, -50, 0.5, true);
        common_effect_color(agent);
        LAST_EFFECT_SET_RATE(agent,0.7);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("goomba_bite_line2"),false,false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("goomba_bite_line2"), Hash40::new("head"), 3.0, 2.0, 1.25, 0, 10, -60, 0.5, true);
        common_effect_color(agent);
        LAST_EFFECT_SET_RATE(agent,0.8);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("goomba_bite_s"), Hash40::new("goomba_bite_s"), Hash40::new("top"), -2.5, 17, 0, 0, 0, 60, 0.6, true, *EF_FLIP_YZ);
        common_effect_color(agent);
        LAST_EFFECT_SET_ALPHA(agent,0.6);
        LAST_EFFECT_SET_RATE(agent,0.8);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_DETACH_KIND(agent, Hash40::new("goomba_bite_s"), -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_rise"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_swing_s"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing01"));
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attackhi3", game_attackhi3, Priority::Default);
	agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Default);
	agent.acmd("sound_attackhi3", sound_attackhi3, Priority::Default);
	agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Default);
}
