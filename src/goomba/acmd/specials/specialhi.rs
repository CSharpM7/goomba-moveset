use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {    
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 4.5, 85, 100, 160, 0, 5.0, 0.0, 4.0, -3.0, Some(0.0), Some(4.0), Some(6.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    let step = 3;
    for _ in (2..(24-step)).step_by(step) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 367, 55, 90, 0, 5.0, 0.0, 6.0, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 367, 55, 90, 0, 5.0, 0.0, 6.0, -3.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 367, 55, 90, 0, 5.0, 0.0, 12.0, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 367, 55, 90, 0, 5.0, 0.0, 12.0, -3.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(agent.lua_state_agent, step as f32);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 65, 130, 0, 70, 9.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {  
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if PostureModule::lr(agent.module_accessor) < 0.0 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("wario_corkscrew_l"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        }
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("wario_corkscrew_r"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("wario_corkscrew_burst"), Hash40::new("top"), 0, 9, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wario_corkscrew_burst"), false, true);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_wario_special_h01"));
        macros::PLAY_STATUS(agent, Hash40::new("vc_wario_008"));
    }
    wait(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wario_swing_l"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 6);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 5, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_specialhistart", game_specialhistart, Priority::Default);
	agent.acmd("game_specialairhistart", game_specialhistart, Priority::Default);
	agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Default);
	agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Default);
	agent.acmd("sound_specialhistart", sound_specialhistart, Priority::Default);
	agent.acmd("sound_specialairhistart", sound_specialhistart, Priority::Default);
	agent.acmd("expression_specialhistart", expression_specialhistart, Priority::Default);
	agent.acmd("expression_specialairhistart", expression_specialhistart, Priority::Default);
    
	agent.acmd("game_specialairhi", game_specialhi, Priority::Default);
	agent.acmd("effect_specialairhi", effect_specialhi, Priority::Default);
	agent.acmd("sound_specialairhi", sound_specialhi, Priority::Default);
	agent.acmd("expression_specialairhi", expression_specialhi, Priority::Default);
}
