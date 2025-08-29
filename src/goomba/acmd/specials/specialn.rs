use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, false, -1);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("footr"), 10.0, 75, 99, 0, 38, 5.0, 0.0, -2.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) {
            //ArticleModule::shoot(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            ArticleModule::change_status_exist(agent.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL, REDSHELL_STATUS_KIND_SHOOT);
            WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_N_SHOOT);
            println!("Req shoot");
        }
        if agent.is_grounded() {
            /*
            println!("JUMP!");
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            StatusModule::set_situation_kind(agent.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            sv_kinetic_energy!(
                set_speed,
                agent,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                4.0
            );
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR); 
            */
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("toer"), *HIT_STATUS_NORMAL);
        macros::ATK_POWER(agent, 0, 7);
        macros::ATK_POWER(agent, 1, 7);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {    
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0.0, 2.0, 0.0, 0, 0, 0, 1.0, true);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        let eff = if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) 
        {Hash40::new("sys_action_smoke_h")} else {Hash40::new("sys_run_smoke")};

        if agent.is_grounded() {
            macros::FOOT_EFFECT(agent, eff, Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 8, 0, 0, 81, 90, 0.85, true);
        common_effect_color(agent);
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
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) {
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
        if WorkModule::is_flag(agent.module_accessor,FIGHTER_GOOMBA_SPECIAL_N_CAN_GEN) {
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
