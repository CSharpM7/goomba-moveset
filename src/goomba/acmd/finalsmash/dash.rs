use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_VORTEX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CAM_ZOOM_OUT(agent);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 30.0, 70, 80, 0, 50, 13.0, 0.0, 3.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

unsafe extern "C" fn effect_finaldash(agent: &mut L2CAgentBase) {
    for _ in 0..6 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.0, false);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

unsafe extern "C" fn sound_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_final02"));
        macros::PLAY_SE(agent, Hash40::new("se_common_fire_m_damage"));
    }
}

unsafe extern "C" fn expression_finaldash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_slashll"), 12, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn game_finaldashend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::SET_SPEED_EX(agent, 3, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::FT_MOTION_RATE(agent, 0.7);
        }
    }
}

unsafe extern "C" fn effect_finaldashend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EffectModule::remove_screen(agent.module_accessor, Hash40::new("bg_pichu_final"), -1);
        //macros::EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("pichu_final_elec"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.0, false);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_final_elec"), false, false);
    }
}

unsafe extern "C" fn sound_finaldashend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("vc_pichu_final01"));
        macros::PLAY_STATUS(agent, Hash40::new("vc_pichu_final02"));
    }
}

unsafe extern "C" fn expression_finaldashend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 21, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_finaldash", game_finaldash, Priority::Default);
	agent.acmd("effect_finaldash", effect_finaldash, Priority::Default);
	agent.acmd("sound_finaldash", sound_finaldash, Priority::Default);
	agent.acmd("expression_finaldash", expression_finaldash, Priority::Default);

	agent.acmd("game_finaldashend", game_finaldashend, Priority::Default);
	agent.acmd("game_finalairdashend", game_finaldashend, Priority::Default);
	agent.acmd("effect_finaldashend", effect_finaldashend, Priority::Default);
	agent.acmd("effect_finalairdashend", effect_finaldashend, Priority::Default);
	agent.acmd("sound_finaldashend", sound_finaldashend, Priority::Default);
	agent.acmd("sound_finalairdashend", sound_finaldashend, Priority::Default);
	agent.acmd("expression_finaldashend", expression_finaldashend, Priority::Default);
	agent.acmd("expression_finalairdashend", expression_finaldashend, Priority::Default);
}
