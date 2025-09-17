use crate::imports::imports_acmd::*;

const DAMAGE: f32 = 3.5;
const ANGLE_G: u64 = 70;
const ANGLE_A: u64 = 367;
const KBG_G: i32 = 40;
const KBG_A: i32 = 25;
const FKB_G: i32 = 0;
const FKB_A: i32 = 0;
const BKB_G: i32 = 50;
const BKB_A: i32 = 50;

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), DAMAGE, ANGLE_A, KBG_A, FKB_A, BKB_A, 3.5, 0.0, 3.0, -4.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("btail2"), DAMAGE, ANGLE_A, KBG_A, FKB_A, BKB_A, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("btail3"), DAMAGE, ANGLE_A, KBG_A, FKB_A, BKB_A, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), DAMAGE, ANGLE_G, KBG_G, FKB_G, BKB_G, 3.5, 0.0, 3.0, -4.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 4, 0, Hash40::new("btail2"), DAMAGE, ANGLE_G, KBG_G, FKB_G, BKB_G, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 5, 0, Hash40::new("btail3"), DAMAGE, ANGLE_G, KBG_G, FKB_G, BKB_G, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        
        for i in 0..5 {
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, i, 4.0, false);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.2, 41, 136, 0, 48, 3.5, 0.0, 3.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("btail2"), 6.5, 41, 136, 0, 48, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("btail3"), 6.5, 41, 136, 0, 48, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let flip_z_rot = 0.0;//if PostureModule::lr(agent.module_accessor) > 0.0 {0.0} else {180.0};
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc3"), Hash40::new("pichu_tail_arc3"), Hash40::new("top"), 0, 4.5, -2, 5, 195, 40.0+flip_z_rot, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent,1.5);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("pichu_tail_arc3"),false,false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let flip_z_rot = 0.0;
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc3"), Hash40::new("pichu_tail_arc3"), Hash40::new("top"), 0, 4.5, -2, 5, 230, 40.0+180.0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent,1.5);
    }
}

unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_tailswing"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_tailswing"));
    }
}

unsafe extern "C" fn expression_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attackairb", game_attackairb, Priority::Default);
	agent.acmd("effect_attackairb", effect_attackairb, Priority::Default);
	agent.acmd("sound_attackairb", sound_attackairb, Priority::Default);
	agent.acmd("expression_attackairb", expression_attackairb, Priority::Default);
}
