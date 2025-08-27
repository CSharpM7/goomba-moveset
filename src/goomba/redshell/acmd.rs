use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 50, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}
unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_greenshell_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(agent,2.0,1.0,0.0);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_fly", game_fly, Priority::Default);
	agent.acmd("effect_fly", effect_fly, Priority::Default);
}
