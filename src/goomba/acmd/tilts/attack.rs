use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 78, 0, 48, 5.25, 0.0, 4.9, 4.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.6);

    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("goomba_bite_line2"),  Hash40::new("top"), -12.0*lr, 3.8, 5.5, 0, 0, 0,0.6, true);
        common_effect_color(agent);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("goomba_bite_s"), Hash40::new("goomba_bite_s"), Hash40::new("top"), -10, 4, 4, 0, -90, 20, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_ALPHA(agent,0.4);
        common_effect_color(agent);
    }
}

unsafe extern "C" fn sound_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_swing_s"));
    }
}

unsafe extern "C" fn expression_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attack11", game_attack11, Priority::Default);
	agent.acmd("effect_attack11", effect_attack11, Priority::Default);
	agent.acmd("sound_attack11", sound_attack11, Priority::Default);
	agent.acmd("expression_attack11", expression_attack11, Priority::Default);
}
