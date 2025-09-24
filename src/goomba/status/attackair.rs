use crate::imports::imports_status::*;

pub unsafe extern "C" fn attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fighter.main_shift(attackair_main_loop)
}
unsafe extern "C" fn attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_motion(Hash40::new("attack_air_lw")) {return fighter.status_AttackAir_Main();}
    
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_DIVE_DECIDE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_DIVE_DECIDE);

        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let dive_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
        let should_dive = fighter.global_table[STICK_Y].get_f32() <= dive_stick_y;
        WorkModule::set_flag(fighter.module_accessor, should_dive, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_SWOOP) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_SWOOP);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING) {
            return attackair_lw_dive(fighter);
        }
        else {
            return attackair_lw_swoop(fighter);
        }
    }
    fighter.status_AttackAir_Main()
}

unsafe extern "C" fn attackair_lw_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING);
    /*
    let fx = EffectModule::req_follow(fighter.module_accessor, 
        Hash40::new("sys_smash_flash"), 
        Hash40::new("head"), 
        &Vector3f::new(4.0, 4.0, 0.0), 
        &VECTOR_ZERO,
        0.6, 
        true, 0, 0, 0, 0, 0, true, true
    ) as u32;
    EffectModule::set_rgb(fighter.module_accessor, fx, 1.0, 1.0, 0.0);
    */
    let frame = MotionModule::frame(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw2"), frame, 1.0, false, 0.0, false, false);
    //MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_lw2"), frame, 1.0, 1.0);
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
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING);

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
            sum_speed_x,
            0.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            1.2,
            0.0
        );
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        //KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
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

pub unsafe extern "C" fn attackair_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN as u32),
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn attackair_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING) {
        return attackair_lw_dive(fighter);
    }
    else {
        return attackair_lw_swoop(fighter);
    }

    //fighter.sub_shift_status_main(L2CValue::Ptr(attackair_lw_main_loop as *const () as _))
}

unsafe extern "C" fn attackair_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if fighter.is_grounded() {
            let next_status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) 
            {FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR} else {FIGHTER_STATUS_KIND_LANDING};
            fighter.change_status(next_status.into(), false.into());
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

pub unsafe extern "C" fn attackair_lw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attackair_main);
}