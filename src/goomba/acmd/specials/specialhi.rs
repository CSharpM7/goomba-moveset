use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_REVERSE_LR);
        WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
        //notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {   
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }   
    //frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_wing_flying"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent,1.25);
    } 
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_special_s02_m"));
    }
}

unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let special_hi_angle = WorkModule::get_float(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLOAT_ANGLE).to_degrees().abs();
        let attack_angle_adjust = (special_hi_angle/2.25) as u64;
        let kbg = if agent.is_grounded() {100} else {115};

        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 3.5, (87-attack_angle_adjust) as u64, kbg, 160, 0, 4.75, 0.0, 4.75, -3.0, Some(0.0), Some(4.75), Some(6.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.9, 367, 55, 90, 0, 5.0, 0.0, 12.0, 4.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.9, 367, 55, 90, 0, 5.0, 0.0, 12.0, -3.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.9, 367, 55, 90, 0, 5.0, 0.0, 6.0, 4.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.9, 367, 55, 90, 0, 5.0, 0.0, 6.0, -3.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.75, 65, 140, 0, 50, 9.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
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
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("goomba_cyclone_r"), Hash40::new("goomba_cyclone_l"), Hash40::new("top"), 0, 16, 0, 0, 0, 0, 0.8, true, *EF_FLIP_NONE);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 2.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_wing_special"), Hash40::new("top"), 0, 5, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent,1.25);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_special_s02_l"));
    }
    //frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_pichu_special_h01"));
        //macros::PLAY_STATUS(agent, Hash40::new("vc_pichu_008"));
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_special_s02_s"));
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