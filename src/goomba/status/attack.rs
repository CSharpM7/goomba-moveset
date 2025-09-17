use crate::imports::imports_status::*;

unsafe extern "C" fn attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
	fighter.sub_shift_status_main(L2CValue::Ptr( attacklw3_main_loop as *const () as _)) 
}
unsafe extern "C" fn attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_AttackLw3_Main().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {return 0.into();}
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT) 
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        }
    }
    0.into()
}

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
unsafe extern "C" fn attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_AttackS4(true.into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
	fighter.sub_shift_status_main(L2CValue::Ptr( attacks4_main_loop as *const () as _)) 
}
unsafe extern "C" fn attacks4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
pub unsafe extern "C" fn attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fighter.main_shift(attackair_main_loop)
}
unsafe extern "C" fn attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_motion(Hash40::new("attack_air_lw")) {return fighter.status_AttackAir_Main();}
    
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_CHECK_FOR_DIVE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_CHECK_FOR_DIVE);

        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let dive_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
        if fighter.global_table[STICK_Y].get_f32() > dive_stick_y {
            attackair_lw_swoop(fighter);
        }
        else {
            attackair_lw_dive(fighter);
        }
    }
    fighter.status_AttackAir_Main()
}

unsafe extern "C" fn attackair_lw_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw2"), frame, 1.0, 1.0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_DIVE);
    fighter.main_shift(attackair_lw_dive_loop)
}

unsafe extern "C" fn attackair_lw_dive_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    fighter.status_AttackAir_Main()
}

unsafe extern "C" fn attackair_lw_swoop(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_DIVE);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.2
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        speed_y*-1.25
    );
    fighter.main_shift(attackair_lw_swoop_loop)
}

unsafe extern "C" fn attackair_lw_swoop_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL);
        
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let stop_speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            sum_speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        //Do I really gotta do all this?
        let ACCEL_MUL = 0.5;
        let MAX_MUL = 1.0;
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            air_accel_x_mul * ACCEL_MUL
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            air_accel_x_add * ACCEL_MUL
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * MAX_MUL,
            0.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * MAX_MUL,
            0.0
        );
    }
    fighter.status_AttackAir_Main()
}

unsafe extern "C" fn attackair_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec();
    if fighter.is_motion(Hash40::new("attack_air_lw")) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        && MotionModule::frame(fighter.module_accessor) > 10.0 {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn attackair_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    fighter.sub_attack_air_uniq_process_exit()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, attacklw3_main);

	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attacks4_start_end);
	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attacks4_hold_end);
	agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, attacks4_end);
	agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, attacks4_main);

	agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attackair_main);
	agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, attackair_exit);
}