use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let friendly_fire = WorkModule::is_flag(agent.module_accessor, REDSHELL_INSTANCE_FLAG_FRIENDLY_FIRE);
        let lr = PostureModule::lr(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 65, 40, 0, 60, 3.25, 0.0, 0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 45, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, friendly_fire, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
}
unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("goomba_greenshell_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent,1.0,2.0,1.0);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            if agent.is_grounded() {
                macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("have"), 0, -3, 0, 0, 0, 0, 0.5, false, *EF_FLIP_NONE);
                let handle = EffectModule::get_last_handle(agent.module_accessor);
                WorkModule::set_int(agent.module_accessor, handle as i32, REDSHELL_INSTANCE_INT_EFF);
            }
        }
        wait(agent.lua_state_agent, 10.0);
    }
}
unsafe extern "C" fn effect_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("goomba_greenshell_trace"), false, false);
    }
}

unsafe extern "C" fn effect_furafura(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("goomba_greenshell_trace"), false, false);
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("have"), 0, -3, 0, 0, 0, 0, 1, true);
        let handle = EffectModule::get_last_handle(agent.module_accessor);
        WorkModule::set_int(agent.module_accessor, handle as i32, REDSHELL_INSTANCE_INT_EFF);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_fly", game_fly, Priority::Default);
	agent.acmd("game_turn", game_turn, Priority::Default);
	agent.acmd("effect_fly", effect_fly, Priority::Default);
	agent.acmd("effect_turn", effect_turn, Priority::Default);
	agent.acmd("effect_furafura", effect_furafura, Priority::Default);
}
