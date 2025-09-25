use crate::imports::imports_status::*;

const DECIDE_DIRECTION_SETS_LR: bool = true;
const JUMP_SPEED_MUL: f32 = 1.25;
const STICK_MUL: f32 = 42.0;

pub unsafe extern "C" fn specialhi_start_init(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}
unsafe extern "C" fn specialhi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_hi").into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.main_shift(specialhi_start_main_loop)
}

unsafe extern "C" fn specialhi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_REVERSE_LR) 
    && !DECIDE_DIRECTION_SETS_LR {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_REVERSE_LR);
        let stick_x = fighter.global_table[STICK_X].get_f32().abs();
        let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("status_start_turn_stick_x"));
        if lr_stick_x <= stick_x {
        //if stick_x * lr < 0.0 && stick_x.abs() >= threshold {
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }

    let changed = fighter.sub_set_ground_correct_by_situation(true.into()).get_bool();
    fighter.sub_set_special_start_inherit_common_kinetic_setting(Hash40::new("param_special_hi").into());
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        let start_stop_y_frame_air = 5.0;//WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_stop_y_frame_air"));
        if changed {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            if MotionModule::frame(fighter.module_accessor) < start_stop_y_frame_air {
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0.0
                );
            }
        }
        if start_stop_y_frame_air <= MotionModule::frame(fighter.module_accessor) + 1.0 {
            let fall_speed_y = 0.1*WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -fall_speed_y
            );
        }
    }

    0.into()
}

pub unsafe extern "C" fn specialhi_start_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    specialhi_apply_angle(fighter);
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        JUMP_SPEED_MUL
    );
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
	fighter.sub_shift_status_main(L2CValue::Ptr( specialhi_main_loop as *const () as _)) 
}
unsafe extern "C" fn specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
	}
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_ENABLE_LANDING) {
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        fighter.sub_set_ground_correct_by_situation(0xa0.into());
    }
    0.into()
}

unsafe extern "C" fn specialhi_apply_angle(fighter: &mut L2CFighterCommon){
    let mut angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLOAT_ANGLE);
    sv_kinetic_energy!(
        set_angle,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        angle
    );
}

unsafe extern "C" fn specialhi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DECIDE_DIRECTION) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
        let stick_min = 0.25;
        let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let mut is_reverse = stick_x.signum() != PostureModule::lr(fighter.module_accessor);
        if stick_x.abs() <= 0.25 {
            stick_x = 0.0;
            is_reverse = false;
        }
        let mut angle = -STICK_MUL*stick_x;
        if DECIDE_DIRECTION_SETS_LR {
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        }
        else {
            let angle_forward = STICK_MUL;
            let angle_back = STICK_MUL/2.0;
            //Set posture module where x is STICK_MUL*stick?
            angle = if is_reverse {
                angle.clamp(-angle_back, angle_back)
            }
            else {
                angle.clamp(-angle_forward, angle_forward)
            };
        }
        
        //println!("Stick X: {stick_x} Rev: {is_reverse} Angle: {angle}");
        angle = angle.to_radians();
        WorkModule::set_float(fighter.module_accessor, angle,FIGHTER_GOOMBA_SPECIAL_HI_FLOAT_ANGLE);
        specialhi_apply_angle(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE);
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0
        );
    }
    
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, empty_status);
	agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_start_main);
	agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, specialhi_exec);
	agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, specialhi_start_end);
    
	agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, empty_status);
	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, specialhi_pre);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, specialhi_main);
	agent.status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, specialhi_exec);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, empty_status);
}