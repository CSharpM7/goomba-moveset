use crate::imports::imports_status::*;

pub unsafe extern "C" fn attacks4_start_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let param = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    fighter.sub_remove_exist_article_at_status_end(param.into(), FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES.into());
    fighter.status_end_AttackS4Start()
}
pub unsafe extern "C" fn attacks4_hold_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let param = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    fighter.sub_remove_exist_article_at_status_end(param.into(), FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES.into());
    fighter.status_end_AttackS4Hold()
}
pub unsafe extern "C" fn attacks4_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let param = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    fighter.sub_remove_exist_article_at_status_end(param.into(), FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES.into());
    fighter.status_end_AttackS4()
}

//Why do i gotta do this
unsafe extern "C" fn  attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_AttackS4(true.into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
	fighter.sub_shift_status_main(L2CValue::Ptr( attacks4_main_loop as *const () as _)) 
}
unsafe extern "C" fn  attacks4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
        if combo < s4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            attack_s4_mtrans(fighter);
        }
    }
    else {
        attack_s4_mtrans(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

unsafe extern "C" fn attack_s4_mtrans(fighter: &mut L2CFighterCommon) {
    let combo = ComboModule::count(fighter.module_accessor);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    if combo != 0 {
        if combo == 1 { //2?
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        let motion = WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND) as u64;
        let restart_frame = WorkModule::get_float(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
        MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), restart_frame, 1.0, 0.0);
    }
}
pub fn install(agent: &mut smashline::Agent) {
	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attacks4_start_end);
	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attacks4_hold_end);
	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, attacks4_end);

	agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, attacks4_main);
}