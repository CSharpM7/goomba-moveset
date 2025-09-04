use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_XLU);

        ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, false, -1);
        let shoe = get_article_boma(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
        accessories::init_shoe(shoe);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 80, 100, 80, 0, 7.0, 0.0, 7.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_BOUNCE);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 5.0, 0.0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("yoshi_hip_drop"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, false);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_special_l01"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_yoshi_special_l01"), 30);
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        if !ArticleModule::is_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, false, -1);
            let shoe = get_article_boma(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
            accessories::init_shoe(shoe);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE);
    }
    frame(agent.lua_state_agent, FIGHTER_GOOMBA_SPECIAL_LW_SPIKE_FRAME);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FALL);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_BOUNCE);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);

        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 275, 68, 0, 15, 6.4, 0.0, 3.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 68, 0, 70, 6.4, 0.0, 4.7, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FROM_GROUND) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 2.0, 0.0, 0, 0, 0, 1.25, true);
        }
    }
    frame(agent.lua_state_agent, FIGHTER_GOOMBA_SPECIAL_LW_SPIKE_FRAME);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 3, 2, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn sound_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("vc_yoshi_jump02"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_special_l01"));
    }
}

unsafe extern "C" fn expression_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, FIGHTER_GOOMBA_SPECIAL_LW_SPIKE_FRAME);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_speciallwlanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_speciallwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 5.0, 0.0, 0, 0, 0, 1.25, true);
    }
}

unsafe extern "C" fn sound_speciallwlanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_special_l02"));
    }
}

unsafe extern "C" fn expression_speciallwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_specialairlwhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //ArticleModule::remove_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_status_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ACCESSORIES_STATUS_KIND_EJECTED);

        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
        macros::SET_SPEED_EX(agent, 0, 2.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn effect_specialairlwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
    }
}

unsafe extern "C" fn sound_specialairlwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_special_l02"));
    }
}

unsafe extern "C" fn expression_specialairlwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_speciallw", game_speciallw, Priority::Default);
	agent.acmd("effect_speciallw", effect_speciallw, Priority::Default);
	agent.acmd("sound_speciallw", sound_speciallw, Priority::Default);
	agent.acmd("expression_speciallw", expression_speciallw, Priority::Default);

	agent.acmd("game_specialairlw", game_specialairlw, Priority::Default);
	agent.acmd("effect_specialairlw", effect_specialairlw, Priority::Default);
	agent.acmd("sound_specialairlw", sound_specialairlw, Priority::Default);
	agent.acmd("expression_specialairlw", expression_specialairlw, Priority::Default);

	agent.acmd("game_speciallwlanding", game_speciallwlanding, Priority::Default);
	agent.acmd("effect_speciallwlanding", effect_speciallwlanding, Priority::Default);
	agent.acmd("sound_speciallwlanding", sound_speciallwlanding, Priority::Default);
	agent.acmd("expression_speciallwlanding", expression_speciallwlanding, Priority::Default);

	agent.acmd("game_specialairlwhit", game_specialairlwhit, Priority::Default);
	agent.acmd("effect_specialairlwhit", effect_specialairlwhit, Priority::Default);
	agent.acmd("sound_specialairlwhit", sound_specialairlwhit, Priority::Default);
	agent.acmd("expression_specialairlwhit", expression_specialairlwhit, Priority::Default);
}
