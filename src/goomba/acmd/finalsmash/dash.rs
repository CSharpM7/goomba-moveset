use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_VORTEX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CAM_ZOOM_OUT(agent);

        macros::ATTACK(agent, 1, 0, Hash40::new("head"), 10.0, 95, 0, 10, 107, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 95, 0, 10, 107, 6.0, 0.0, 6.0, -12.0, Some(0.0), Some(6.0), Some(12.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);

        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 60.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 60.0, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0); 
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    for i in 1..5 {
        if macros::is_excute(agent) {
            AttackModule::set_size(agent.module_accessor, 0, (i as f32)*10.0)
        }
        wait(agent.lua_state_agent, 1.0); 
    }
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR);
    }
}

unsafe extern "C" fn effect_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_finaldash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
}

unsafe extern "C" fn expression_finaldash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false,0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_bounce"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_finalfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 274, 132, 0, 57, 50.0, 0.0, 52.0, -2.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_finalfall(agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_finalfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_03"));
    }
}

unsafe extern "C" fn expression_finalfall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false,0);
    }
}

unsafe extern "C" fn game_finaldashend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 65, 120, 0, 60, 12.0, 0.0, 24.0, -35.0, Some(0.0), Some(24.0), Some(35.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 65, 120, 0, 60, 12.0, 0.0, 8.0, -20.0, Some(0.0), Some(8.0), Some(20.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);

        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, false, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, false, -1.0, false);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_RELEASE_OPPONENTS);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR);
    }
}

unsafe extern "C" fn effect_finaldashend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 4.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 4.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        EffectModule::remove_screen(agent.module_accessor, Hash40::new("bg_pichu_final"), -1);
    }
}

unsafe extern "C" fn sound_finaldashend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final04"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_final06"));
    }
}

unsafe extern "C" fn expression_finaldashend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false,0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attack_critical"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_finaldash", game_finaldash, Priority::Default);
	agent.acmd("effect_finaldash", effect_finaldash, Priority::Default);
	agent.acmd("sound_finaldash", sound_finaldash, Priority::Default);
	agent.acmd("expression_finaldash", expression_finaldash, Priority::Default);
    
	agent.acmd("game_finalfall", game_finalfall, Priority::Default);
	agent.acmd("effect_finalfall", effect_finalfall, Priority::Default);
	agent.acmd("sound_finalfall", sound_finalfall, Priority::Default);
	agent.acmd("expression_finalfall", expression_finalfall, Priority::Default);

	agent.acmd("game_finaldashend", game_finaldashend, Priority::Default);
	agent.acmd("effect_finaldashend", effect_finaldashend, Priority::Default);
	agent.acmd("sound_finaldashend", sound_finaldashend, Priority::Default);
	agent.acmd("expression_finaldashend", expression_finaldashend, Priority::Default);
}
