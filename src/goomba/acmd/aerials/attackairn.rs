use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 55, 110, 0, 30, 6.0, 1.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 55, 100, 0, 25, 5.0, 1.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 6.0, 55, 100, 0, 15, 4.0, 1.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    FT_MOTION_RATE_RANGE(agent,20.0,41.0,28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 41.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.7, true, 0.6);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 0, 0, 270, 0.7, true, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 0, 0, 270, 0.5, true, 1);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 0, 0, 270, 0.6, true, 0.7);
    }
}

unsafe extern "C" fn sound_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_attackair_n01"));
    }
}

unsafe extern "C" fn expression_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attackairn", game_attackairn, Priority::Default);
	agent.acmd("effect_attackairn", effect_attackairn, Priority::Default);
	agent.acmd("sound_attackairn", sound_attackairn, Priority::Default);
	agent.acmd("expression_attackairn", expression_attackairn, Priority::Default);
}