use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_finalattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_INT_VORTEX_TIME_COUNT);
        //StatusModule::change_status_request(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, false);
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 366, 90, 100, 0, 13.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

unsafe extern "C" fn expression_finalattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_08_elecatksp"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_finalattack2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_INT_VORTEX_TIME_COUNT);
        //StatusModule::change_status_request(agent.module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, false);
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 366, 90, 100, 0, 13.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

pub fn install(agent: &mut Agent) {
	agent.acmd("game_finalattack", game_finalattack, Priority::Default);
	agent.acmd("game_finalattack2", game_finalattack2, Priority::Default);
	agent.acmd("game_finalattackfinish", game_finalattack, Priority::Default);
	agent.acmd("expression_finalattack", expression_finalattack, Priority::Default);
	agent.acmd("expression_finalattack2", expression_finalattack, Priority::Default);
	agent.acmd("expression_finalattackfinish", expression_finalattack, Priority::Default);
}