use crate::imports::imports_status::*;
use super::*;

pub const HOP_SPEED_Y: f32 = 0.5;
pub const HOP_GRAVITY_CHANGE_THRESHOLD: f32 = 0.625;
pub const HOP_GRAVITY_ACCEL_FACTOR: f32 = 0.75;
pub const HOP_GRAVITY_LIMIT_FACTOR: f32 = 0.5;
pub const HOP_CONTROL_ACCEL_FACTOR: f32 = 0.25;
pub const HOP_CONTROL_LIMIT_FACTOR: f32 = 0.25;
pub const BRAKE_FACTOR: f32 = 0.5;

pub unsafe extern "C" fn specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn specials_gravity(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let speed_y = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        let normal_gravity_hop = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_HAS_HOP)
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_GRAVITY);

        let normal_gravity_speed = speed_y > HOP_GRAVITY_CHANGE_THRESHOLD;
        if normal_gravity_hop || normal_gravity_speed {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        else {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y * HOP_GRAVITY_ACCEL_FACTOR
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable * HOP_GRAVITY_LIMIT_FACTOR
            );
        }
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_accel_x_mul * HOP_CONTROL_ACCEL_FACTOR
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * HOP_CONTROL_LIMIT_FACTOR
        );
    }
}

unsafe extern "C" fn specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_hop = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SPECIAL_S_DISABLE_HOP);
    WorkModule::set_flag(fighter.module_accessor, has_hop, FIGHTER_GOOMBA_SPECIAL_S_HAS_HOP);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_GRAVITY);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_REFLECT_SFX);

    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let mot_g = hash40("special_s_end");
    let mot_a = hash40("special_air_s_end");
    WorkModule::set_int64(fighter.module_accessor, mot_g as i64,*FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, mot_a as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    status_on_situation_update(fighter,false,*FIGHTER_KINETIC_TYPE_GROUND_STOP,*FIGHTER_KINETIC_TYPE_AIR_STOP,false);
    
    let reset_type = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    {ENERGY_STOP_RESET_TYPE_GROUND} else {ENERGY_STOP_RESET_TYPE_AIR};
    let brake_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    {hash40("ground_brake")} else {hash40("air_brake_x")};

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        reset_type,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    let brake = WorkModule::get_param_float(fighter.module_accessor, brake_param, 0);
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        brake*BRAKE_FACTOR
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(specials_main_loop as *const () as _))
}

unsafe extern "C" fn specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        status_on_situation_update(fighter,true,*FIGHTER_KINETIC_TYPE_GROUND_STOP,*FIGHTER_KINETIC_TYPE_AIR_STOP,false);
        let brake_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        {hash40("ground_brake")} else {hash40("air_brake_x")};
        let brake = WorkModule::get_param_float(fighter.module_accessor, brake_param, 0);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            brake
        );
        specials_gravity(fighter);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

pub unsafe extern "C" fn specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_HOP) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SPECIAL_S_DISABLE_HOP) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_HOP);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SPECIAL_S_DISABLE_HOP);
            let speed_y = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    HOP_SPEED_Y.max(speed_y)
                );
            }
        }
    }
    
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_S_CONTROL);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
    specials_gravity(fighter);

    0.into()
}

pub unsafe extern "C" fn specials_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, empty_status);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_exit);
}