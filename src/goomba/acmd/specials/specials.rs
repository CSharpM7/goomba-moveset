use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    }
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,11.0,4.0);
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE_RANGE(agent,11.0,16.0,3.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_HOP);
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_GRAVITY);
    }
    frame(agent.lua_state_agent, 17.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.0, 0.0, 5.5, 3.5, 0.0, 5.5, 9.7, 
        1.4 /* Speed/Damage */, 1.5 /* Speed/Damage */, 50 /* Threshold */, false, 1.0 /* Life Mult */, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 20, 0, 82, 7.0, 0.0, 5.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 4.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_CONTROL);
    }
    frame(agent.lua_state_agent, 29.0);
    FT_MOTION_RATE_RANGE(agent,29.0,49.0,10.0);
    frame(agent.lua_state_agent, 49.0);
    FT_MOTION_RATE_RANGE(agent,49.0,67.0,7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_GRAVITY);
    }
    frame(agent.lua_state_agent, 67.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    }
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if agent.is_grounded() {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_NONE);
        }
    }
    
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pichu_tail_arc"), Hash40::new("pichu_tail_arc"), Hash40::new("top"), 0, 3, 2, 0, 360, 15, 0.9, true, *EF_FLIP_YZ);
        //LAST_EFFECT_SET_RATE(agent,1.9);
    }
    frame(agent.lua_state_agent, 18.0); //17.0
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 4, -3, 0, 30, 0, 1.7, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(agent,0.4,0.1,0.0);
    }
}

unsafe extern "C" fn sound_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_dash_start"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_tailswing"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        sequence!(agent,hash40("kuribo"),hash40("vc_pichu_attack04"),hash40("vc_pichu_attack05"),hash40(""));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_swing_m"));
    }
}

unsafe extern "C" fn expression_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_01_mantle"), 0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 0, 300, 0.5, 12, 10, 30, 20, 50);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}
pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_specialsend", game_specialsend, Priority::Default);
	agent.acmd("game_specialairsend", game_specialsend, Priority::Default);
	agent.acmd("effect_specialsend", effect_specialsend, Priority::Default);
	agent.acmd("effect_specialairsend", effect_specialsend, Priority::Default);
	agent.acmd("sound_specialsend", sound_specialsend, Priority::Default);
	agent.acmd("sound_specialairsend", sound_specialsend, Priority::Default);
	agent.acmd("expression_specialsend", expression_specialsend, Priority::Default);
	agent.acmd("expression_specialairsend", expression_specialsend, Priority::Default);
}
