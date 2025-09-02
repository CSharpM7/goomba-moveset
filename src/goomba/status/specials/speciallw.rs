use crate::imports::imports_status::*;

pub unsafe extern "C" fn speciallw_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    StatusModule::set_keep_situation_air(fighter.module_accessor, true);
    ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

pub unsafe extern "C" fn speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,//(*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN as u32),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND.into(), false.into());
        return 0.into()
    }
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FROM_GROUND);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    
    let move_x_mul_on_ground = 1.0;
    let move_y_mul_on_ground = 1.4;
    sv_kinetic_energy!(
        set_speed_mul_2nd,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        move_x_mul_on_ground,
        move_y_mul_on_ground
    );

	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_main_loop as *const () as _)) 
}
unsafe extern "C" fn speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND.into(), false.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE) {
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    }
    0.into()
}

pub unsafe extern "C" fn speciallw_end_common(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_interupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let status_next = StatusModule::status_kind_next(fighter.module_accessor);
    if !([FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND,FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING,FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_next)) {
        ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

pub unsafe extern "C" fn speciallw_pound_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FROM_GROUND) 
    {*FIGHTER_STATUS_ATTR_START_TURN} else {0};
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
            turn as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn speciallw_pound_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_GOOMBA_SPECIAL_LW_INT_COUNT);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FROM_GROUND) {
        let end_frame = MotionModule::end_frame(fighter.module_accessor);
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, end_frame, true, true, false);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_BOUNCE);
    }
    
    if fighter.global_table[IS_STOP].get_bool() {
        //uhh
    }

	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_pound_main_loop as *const () as _)) 
}
unsafe extern "C" fn speciallw_pound_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FALL);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_BOUNCE);
    }
    let count = fighter.global_table[STATUS_FRAME].get_i32();//WorkModule::get_int(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_INT_COUNT);
    let pass_frame = 5;
    if count > pass_frame {
        if !fighter.sub_fighter_do_control_passable().get_bool() {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
    }
    else {
        GroundModule::set_passable_check(fighter.module_accessor, true);
    }
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE) {
        if speed_y <= 0.0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn speciallw_pound_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into()); //?
    let param_speed_y = -4.8;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FALL) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(gravity_energy, param_speed_y);
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy, 0.0);
    }

    0.into()
}
unsafe extern "C" fn speciallw_pound_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let mut can_bounce = false;
    if (&param_3["object_category_"]).get_i32() == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        //println!("Hit a dude");
        if (&param_3["kind_"]).get_i32() == *COLLISION_KIND_HIT {
            //println!("Hit a dude not shielding");
            can_bounce = true;
            /* 
            let object_id = (&param_3["object_id_"]).get_u32();
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            */
        }
    }
    else {
        can_bounce = true;
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_BOUNCE) 
    && can_bounce {
        fighter.change_status(FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

pub unsafe extern "C" fn speciallw_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        ( *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn speciallw_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_landing_main_loop as *const () as _)) 
}
unsafe extern "C" fn speciallw_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
	}
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}
unsafe extern "C" fn speciallw_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    speciallw_end_common(fighter)
}

pub unsafe extern "C" fn speciallw_hit_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    StatusModule::set_keep_situation_air(fighter.module_accessor, true);
    KineticModule::clear_speed_all(fighter.module_accessor);
    0.into()
}
pub unsafe extern "C" fn speciallw_hit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
	fighter.sub_shift_status_main(L2CValue::Ptr( speciallw_hit_main_loop as *const () as _)) 
}
unsafe extern "C" fn speciallw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
	}
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_LANDING_ENABLE) {
        StatusModule::set_keep_situation_air(fighter.module_accessor, false);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, speciallw_init);
	agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, speciallw_pre);
	agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, speciallw_main);
	agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, empty_status);
	agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, speciallw_end_common);
	agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_N, speciallw_pound_attack);
    
	agent.status(Init, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, empty_status);
	agent.status(Pre, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, speciallw_pound_pre);
	agent.status(Main, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, speciallw_pound_main);
	agent.status(Exec, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, speciallw_pound_exec);
	agent.status(End, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, speciallw_end_common);
	agent.status(CheckAttack, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND, speciallw_pound_attack);
    
	agent.status(Init, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING, empty_status);
	agent.status(Pre, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING, speciallw_landing_pre);
	agent.status(Main, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING, speciallw_landing_main);
	agent.status(Exec, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING, empty_status);
	agent.status(End, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING, speciallw_landing_end);
    
	agent.status(Init, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT, speciallw_hit_init);
	agent.status(Pre, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT, speciallw_hit_pre);
	agent.status(Main, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT, speciallw_hit_main);
	agent.status(Exec, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT, empty_status);
	agent.status(End, FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT, speciallw_end_common);
}