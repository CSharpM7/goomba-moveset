use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        let can_gen = WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN);
        if can_gen {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, false, -1);
            ArticleModule::change_status_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, REDSHELL_STATUS_KIND_HAVED);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_N_FLAG_SHOOT);

        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) {
            ArticleModule::change_status_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, REDSHELL_STATUS_KIND_SHOOT);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 3.0, 361, 0, 10, 40, 4.1, 4.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 12.5, 75, 89, 0, 48, 4.1, 4.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.5, 75, 89, 0, 48, 4.1, 0.0, 4.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor,1,false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.0, 75, 89, 0, 48, 4.1, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_misfire"), Hash40::new("throw"), 0.0, 2.0, 0.0, 0, 0, 0, 2.0, true);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) 
        && agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) 
        && agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 6, 0, 0, 81, 90, 0.75, true);
        common_effect_color(agent);
        LAST_EFFECT_SET_RATE(agent,0.6);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_pichu_special_s01"));
        sequence!(agent,hash40("kuribo"),hash40("vc_pichu_attack04"),hash40("vc_pichu_attack05"),hash40(""));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent, Hash40::new("vc_pichu_special_n01"),0.75);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) {
            macros::PLAY_SE(agent, Hash40::new("se_item_greenshell_hit"));
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_FLAG_CAN_GEN) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        else {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
        }
    }
}
pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_specialn", game_specialn, Priority::Default);
	agent.acmd("game_specialairn", game_specialn, Priority::Default);
	agent.acmd("effect_specialn", effect_specialn, Priority::Default);
	agent.acmd("effect_specialairn", effect_specialn, Priority::Default);
	agent.acmd("sound_specialn", sound_specialn, Priority::Default);
	agent.acmd("sound_specialairn", sound_specialn, Priority::Default);
	agent.acmd("expression_specialn", expression_specialn, Priority::Default);
	agent.acmd("expression_specialairn", expression_specialn, Priority::Default);
}
