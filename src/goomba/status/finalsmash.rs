use crate::imports::imports_status::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
    pub fn dead_range(lua_state: u64) -> Vector4f; 
}
unsafe fn get_blastzone_top(lua_state_agent: u64) -> f32 {
    return dead_range(lua_state_agent).z;
}

const ANIMATION_HEIGHT: f32 = 270.0;
const FALL_SPEED_Y: f32 = -25.0;

pub unsafe extern "C" fn final_common_flags(fighter: &mut L2CFighterCommon,is_final: bool) {
    WorkModule::set_flag(fighter.module_accessor,is_final, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    WorkModule::set_flag(fighter.module_accessor,is_final, *FIGHTER_INSTANCE_WORK_ID_FLAG_KO_SURVIVE);

    WorkModule::set_flag(fighter.module_accessor,!is_final, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::set_flag(fighter.module_accessor,!is_final, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);

    AreaModule::set_whole(fighter.module_accessor, !is_final);
}

pub unsafe extern "C" fn final_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    let final_statuses = [*FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT,*FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH,*FIGHTER_PIKACHU_STATUS_KIND_FINAL_END];
    if !(final_statuses.contains(&next_status))
    || true
    {
        final_common_flags(fighter,false);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
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
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK.into(), false.into());
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
        *GROUND_CORRECT_KIND_NONE as u32,
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
            (*FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_INHERIT_CURSOR |
                *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_FINAL) as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW) as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn final_dash_set_motion_mul(fighter: &mut L2CFighterCommon) {
    let top_blastzone = get_blastzone_top(fighter.lua_state_agent);
    let current_y = PostureModule::pos_y(fighter.module_accessor);
    let mul = (top_blastzone-current_y) / ANIMATION_HEIGHT;
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        1.0,
        mul.max(1.0)
    );
}

unsafe extern "C" fn final_raycast(fighter: &mut L2CFighterCommon,x:f32) -> bool {
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_length = ANIMATION_HEIGHT*2.0;
    let start_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_Y);

    return GroundModule::ray_check_hit_pos(
        fighter.module_accessor,
        &Vector2f{x: x, y: start_y},
        &Vector2f{x: 0.0, y: -ray_length},
        ground_hit_pos,
        true
    );
}
unsafe extern "C" fn final_set_target_x(fighter: &mut L2CFighterCommon) {
    let mut target_x = PostureModule::pos_x(fighter.module_accessor);
    let ray_start_x = (dead_range(fighter.lua_state_agent).y + dead_range(fighter.lua_state_agent).x);
    println!("Middle blast: {ray_start_x}");
    WorkModule::set_float(fighter.module_accessor, ray_start_x, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
    let ray_start_y = get_blastzone_top(fighter.lua_state_agent)-50.0;
    WorkModule::set_float(fighter.module_accessor, ray_start_y, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_Y);
    println!("Top blast: {ray_start_y}");
    let max_tries = 10;
    let step = 10;

    if !final_raycast(fighter,target_x) {
        for i in 0..max_tries*2 {
            let i_as_x = if i < max_tries {i*step} else {(i-(max_tries*2))*step};
            let test_x = ray_start_x + (i_as_x as f32);
            println!("Test at {test_x}");
            if final_raycast(fighter,test_x) {
                target_x = test_x as f32;
                break;
            }
        }
    }
    WorkModule::set_float(fighter.module_accessor, target_x, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
    let new_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
    println!("Set to : {new_x}");
}

unsafe extern "C" fn final_dash_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_FLAG_ATTACK_HIT);
    WorkModule::set_int(fighter.module_accessor, 30,*FIGHTER_PIKACHU_STATUS_FINAL_WORK_INT_VORTEX_TIME_COUNT);
    final_set_target_x(fighter);
    0.into()
}

unsafe extern "C" fn final_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    final_common_flags(fighter,true);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_FALL);
    WorkModule::set_float(fighter.module_accessor, -25.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);

    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK {
        final_dash_set_motion_mul(fighter);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_dash"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        println!("Attack status");
    }
    if fighter.global_table[IS_STOP].get_bool() {
        //uhh
    }

	fighter.sub_shift_status_main(L2CValue::Ptr( final_dash_main_loop as *const () as _)) 
}
unsafe extern "C" fn final_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_FLAG_ATTACK_HIT) {
            //fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        //WorkModule::on_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_IS_FINAL);
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_2.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

unsafe extern "C" fn final_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK {
        println!("Needs x");
        final_hit_init(fighter);
    } 
    */
    
    final_common_flags(fighter,true);
    ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);

    let new_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_FLOAT_HIT_POS_X);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let new_pos = Vector3f::new(new_x,pos_y,0.0);
    PostureModule::set_pos(fighter.module_accessor, &new_pos);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_fall"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

	fighter.sub_shift_status_main(L2CValue::Ptr( final_fall_main_loop as *const () as _)) 
}
unsafe extern "C" fn final_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_AIR_STOP {
            final_common_flags(fighter,false);

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
    /* 
    let count = fighter.global_table[STATUS_FRAME].get_i32();
    let pass_frame = 5;
    if count > pass_frame {
        if !fighter.sub_fighter_do_control_passable().get_bool() {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
    }
    else {
        GroundModule::set_passable_check(fighter.module_accessor, true);
    }
    */
    //let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if //speed_y <= 1.0 && 
    fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH.into(), false.into());
        return 0.into();
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
    let start_num_hit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_INT_ATTACK_HIT_NUM);
    let to_return = smashline::original_status(CheckAttack, fighter, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH)(fighter,param_2,param_3);
    
    let current_num_hit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_FINAL_WORK_INT_ATTACK_HIT_NUM);
    if current_num_hit == start_num_hit {
        return to_return;
    }
    
    let object_id = (&param_3["object_id_"]).get_u32();
    let opponent_boma = sv_battle_object::module_accessor(object_id);
    LinkModule::link(opponent_boma, *FIGHTER_LINK_NO_FINAL, fighter.battle_object_id);

    //StatusModule::set_status_kind_interrupt(opponent_boma, *FIGHTER_STATUS_KIND_IKE_FINAL_DAMAGE_FLY);
    StatusModule::change_status_request(opponent_boma, *FIGHTER_STATUS_KIND_PIKACHU_FINAL_DAMAGE_FLY,false);
    println!("Time to fly");
    return to_return;
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
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GOOMBA_FINAL_FLAG_IS_END_PHASE)
        && false
        {
            let mot_g = Hash40::new("final_end");
            let mot_a = Hash40::new("final_air_end");
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
        if situation == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(),false.into());
            return 1.into();
        }

        let mot_g = Hash40::new("final_end");
        let mot_a = Hash40::new("final_air_end");
        let motion = mot_g;//if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
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
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        if situation == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(),false.into());
            return 1.into();
        }
	}
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
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

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, final_dash_pre);
	agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, final_dash_init);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, final_dash_main);
	agent.status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, empty_status);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, final_common_end);
	agent.status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, empty_status);
    agent.status(CheckAttack, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK, final_dash_attack);

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT, final_dash_pre);
	agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT, final_dash_init);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT, final_dash_main);
	agent.status(Exec, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT, empty_status);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_HIT, final_common_end);

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_2, final_dash_pre);
	agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_2, empty_status);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_2, final_fall_main);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_2, final_common_end);

	agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, final_landing_pre);
	agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, empty_status);
	agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, final_landing_main);
	agent.status(End, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, final_landing_end);
	agent.status(Exit, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_ATTACK_FINISH, empty_status);
}