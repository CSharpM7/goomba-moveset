use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 9.0, 45.0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            REQ_FINAL_START_CAMERA(agent,Hash40::new("d04final.nuanmb"), false);
            macros::FT_START_CUTIN(agent);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 70, 30, 0, 5.5, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 70, 30, 0, 5.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, false, -1.0, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, false, -1.0, false);
        }
        else {
            //PostureModule::scale(agent.module_accessor, 3, 0);
            //0x103370(1867797157, 2.2);
            CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, 2.2, 0.0, 0.0);
            macros::FT_START_CUTIN(agent);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 361, 70, 30, 0, 5.5, 0.0, 0.0, 5.0, Some(0.0), Some(10.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 361, 70, 30, 0, 5.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, false, -1.0, false);
            AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, false, -1.0, false);
        }
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        //StatusModule::change_status_force(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, false);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 50, 100, 75, 0, 5.5, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 20, 100, 40, 0, 5.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(-5.0), 0.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_pichu_final"), false, false, false);
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 1..5 {
        if macros::is_excute(agent) {
            //macros::EFFECT_FOLLOW(agent, Hash40::new("pichu_final_hold"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
        wait(agent.lua_state_agent, 10.0);
    }
    //frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("pichu_final_hold"), false, false);
        /*
		lua_args!(agent, Hash40::new("pichu_final_sphere_start"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
        agent.pop_lua_stack(1); 
        */
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.9, false);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_pichu_final01"));
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_final05"));
    }
}

unsafe extern "C" fn expression_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        app::FighterUtil::flash_eye_info(agent.module_accessor);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}


pub fn install(agent: &mut Agent) {
	agent.acmd("game_final", game_final, Priority::Default);
	agent.acmd("game_finalair", game_final, Priority::Default);
	agent.acmd("effect_final", effect_final, Priority::Default);
	agent.acmd("effect_finalair", effect_final, Priority::Default);
	agent.acmd("sound_final", sound_final, Priority::Default);
	agent.acmd("sound_finalair", sound_final, Priority::Default);
	agent.acmd("expression_final", expression_final, Priority::Default);
	agent.acmd("expression_finalair", expression_final, Priority::Default);
}
