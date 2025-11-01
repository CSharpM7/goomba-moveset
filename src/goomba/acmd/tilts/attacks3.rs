use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let mut attack_height = *ATTACK_HEIGHT_MIDDLE;
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_s3_hi") {
            attack_height = *ATTACK_HEIGHT_HIGH;
        }
        else if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_s3_lw") {
            attack_height = *ATTACK_HEIGHT_LOW;
        }
        sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 9.5, 361, 104, 0, 32, 4.1, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 9.5, 361, 104, 0, 32, 4.6, 6.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::HIT_NODE(agent, Hash40::new("toel"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 18.0);
    FT_MOTION_RATE_RANGE(agent,18.0,34.0,7.0);
    frame(agent.lua_state_agent, 34.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_attacks3(agent: &mut L2CAgentBase) {
    let mut attack_height = *ATTACK_HEIGHT_MIDDLE;
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_s3_hi") {
            attack_height = *ATTACK_HEIGHT_HIGH;
        }
        else if MotionModule::motion_kind(agent.module_accessor) == hash40("attack_s3_lw") {
            attack_height = *ATTACK_HEIGHT_LOW;
        }
        if attack_height == *ATTACK_HEIGHT_HIGH {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 6.7, 1.8, -25, -45, 45, 0.9, true, *EF_FLIP_YZ);
        }
        else if attack_height == *ATTACK_HEIGHT_MIDDLE {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 4.7, 2.5, 0, -60, 20, 1.0, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 2, 4, 1.8, 15, -60, 3, 1.0, true, *EF_FLIP_YZ);
        }
        common_effect_color(agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn expression_attacks3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}


pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attacks3", game_attacks3, Priority::Default);
	agent.acmd("effect_attacks3", effect_attacks3, Priority::Default);
	agent.acmd("sound_attacks3", sound_attacks3, Priority::Default);
	agent.acmd("expression_attacks3", expression_attacks3, Priority::Default);
}