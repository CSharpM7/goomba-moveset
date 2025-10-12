use crate::imports::imports_status::*;

const FALL_SPEED_Y: f32 = -20.0;

pub unsafe extern "C" fn final_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    let final_statuses = [*FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH,*FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END,*FIGHTER_PIKACHU_STATUS_KIND_FINAL_END];
    if !(final_statuses.contains(&next_status))
    //|| true
    {
        ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
        smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
        AttackModule::clear_all(fighter.module_accessor);
    }
    0.into()
}
pub unsafe extern "C" fn rainbow_color(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    //let frame = app::sv_information::get_remaining_time_as_frame();
    let mut r = (frame*10.0).sin();
    let mut g = (frame*10.0).cos();
    let mut b = (frame*10.0).tan();
    r = (r+1.0)*0.5;
    g= (g+1.0)*0.5;
    b= (b+1.0)*0.5;
    ColorBlendModule::set_main_color(fighter.module_accessor, &Vector4f{ x: r, y: g, z: b, w: 0.7 }, 
        &Vector4f{ x: 1.0, y: 0.5, z: 0.5, w: 0.5 }, 
    1.0, 0.5, 10, true);
}

pub unsafe extern "C" fn final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
    AreaModule::set_whole(fighter.module_accessor, false);
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KO_SURVIVE);

    let scale = ModelModule::scale(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, scale, FIGHTER_GOOMBA_INSTANCE_FLOAT_FINAL_START_SCALE);

    let mot_g = Hash40::new("final");
    let mot_a = Hash40::new("final_air");
    let motion = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);

	fighter.sub_shift_status_main(L2CValue::Ptr( final_main_loop as *const () as _))
}

unsafe extern "C" fn final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    rainbow_color(fighter);

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor)  {
        let mot_g = Hash40::new("final");
        let mot_a = Hash40::new("final_air");
        fighter.sub_change_motion_by_situation(mot_g.into(), mot_a.into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        //WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_IS_FINAL);
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH.into(), false.into());
        return 1.into();
    }
    0.into()

}

unsafe extern "C" fn final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("goomba_magic_bright2"), false, false);
    
    return final_common_end(fighter);
}
pub unsafe extern "C" fn final_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = 0;
    //if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_SPECIAL_LW_FLAG_FROM_GROUND) {*FIGHTER_STATUS_ATTR_START_TURN} else {0};
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
        *FS_SUCCEEDS_KEEP_ATTACK
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
unsafe extern "C" fn final_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_FALL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_dash"), 0.0, 1.0, false, 0.0, false, false);
    if fighter.global_table[IS_STOP].get_bool() {
        //uhh
    }

	fighter.sub_shift_status_main(L2CValue::Ptr( final_dash_main_loop as *const () as _)) 
}
unsafe extern "C" fn final_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_AIR_STOP {
            let sum = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //println!("Air stop ({sum})");

            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                sum
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -0.1
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -1.0
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -1.0
            );
        }

        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_FALL);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR) {
        ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    }
    else {
        rainbow_color(fighter);
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
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_FALL) {
        if speed_y <= 1.0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn final_dash_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        //I feel like we should switch the flag off but uhhhh yoshi doesnt
        if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(gravity_energy, FALL_SPEED_Y);
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy, 0.0);
    }
    0.into()
}

unsafe extern "C" fn final_dash_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn final_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_INHERIT_CURSOR |
            *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_FINAL) as u32,
        ( *FIGHTER_POWER_UP_ATTACK_BIT_FINAL) as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn final_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_FALL);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_IS_END_PHASE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_dash_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
	fighter.sub_shift_status_main(L2CValue::Ptr( final_landing_main_loop as *const () as _)) 
}
unsafe extern "C" fn final_landing_gravity(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

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

    }
}

unsafe extern "C" fn final_landing_common_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        let situation_prev = fighter.global_table[PREV_SITUATION_KIND].get_i32();
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_IS_END_PHASE) {
            let mot_g = Hash40::new("final2");
            let mot_a = Hash40::new("final_air2");
            let motion = if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
            fighter.sub_change_motion_by_situation((mot_g).into(), (mot_a).into(), true.into());
        }
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_set_ground_correct_by_situation((true).into());
        final_landing_gravity(fighter);

        if situation != situation_prev &&
        (situation == *SITUATION_KIND_AIR) {
            AttackModule::clear_all(fighter.module_accessor);
        }
	}

    0.into()
}

unsafe extern "C" fn final_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    final_landing_common_loop(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_CANCEL_COLOR) {
        rainbow_color(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        let mot_g = Hash40::new("final2");
        let mot_a = Hash40::new("final_air2");
        let motion = if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

        ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_IS_END_PHASE);
        return fighter.sub_shift_status_main(L2CValue::Ptr( final_landing_end_loop as *const () as _)) ;
    }

    0.into()
}
unsafe extern "C" fn final_landing_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    } 
    final_landing_common_loop(fighter);
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        
	}
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(), false.into());
    }

    0.into()
}
unsafe extern "C" fn final_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    final_common_end(fighter)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_FINAL, final_main);
    agent.status(End, *FIGHTER_STATUS_KIND_FINAL, final_end);

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, final_dash_pre);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, final_dash_main);
	agent.status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, empty_status);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, final_common_end);
	agent.status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, empty_status);
    agent.status(CheckAttack, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, final_dash_attack);

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END, final_landing_pre);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END, final_landing_main);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END, final_landing_end);
	agent.status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH_END, empty_status);
}