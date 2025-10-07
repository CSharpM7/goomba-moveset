use crate::imports::imports_agent::*;

pub const LIFE: i32 = 180;
pub const BRAKE_X_INIT: f32 = 0.001;
pub const FRIENDLY_FIRE_COOLDOWN: i32 = 30;
pub const GRAVITY: f32 = 0.2;
pub const SPEED_X: f32 = 1.875;
pub const OTTOTTO_CHECK_MUL: f32 = 1.0;
pub const TIMEOUT_COOLDOWN: i32 = 15;

pub unsafe extern "C" fn redshell_haved_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
pub unsafe extern "C" fn redshell_haved_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        let link_created = LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner_id);
        has_link = link_created & 1 != 0;
    }
    if has_link {
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
        //LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POS as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_FLIP as u8}, true);
    } 
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let rot_y = PostureModule::rot_y(owner, 0);
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("have"));
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("rot"));
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: rot_y, z: 0.0}, 0);
    let parent_bone = Hash40::new("throw");

    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("have"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    LinkModule::set_constraint_rot_offset(weapon.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0});

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 0.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(redshell_haved_main_loop as *const () as _))
}

unsafe extern "C" fn redshell_haved_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn redshell_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn redshell_reset_attack(weapon: &mut smashline::L2CWeaponCommon) {
    let motion = MotionModule::motion_kind(weapon.module_accessor);
    let new_motion = Hash40::new("fly");
    if motion == hash40("fly") {
        let new_motion = Hash40::new("turn");
        WorkModule::set_int(weapon.module_accessor, 10, REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN);
    }
    MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, new_motion, 0.0, 1.0, 1.0);
}
unsafe extern "C" fn redshell_set_correct_kinetics(weapon: &mut smashline::L2CWeaponCommon) {
    let lr = PostureModule::lr(weapon.module_accessor);
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let is_ground = situation == *SITUATION_KIND_GROUND;

    if !is_ground {
        let rot_y = PostureModule::rot_x(weapon.module_accessor, 0);
        PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(0.0,rot_y,0.0), 0);
    }
    else {
        redshell_set_angle(weapon);
    }

    let correct = if is_ground {*GROUND_CORRECT_KIND_GROUND_OTTOTTO} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(correct));
    
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = if weapon.is_grounded() {0.0} else {KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)};
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );

    redshell_update_brake(weapon,1.0);
}

unsafe extern "C" fn redshell_update_brake(weapon: &mut smashline::L2CWeaponCommon, mul_brake: f32) {
    let lr = PostureModule::lr(weapon.module_accessor);
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let status = StatusModule::status_kind(weapon.module_accessor);
    let is_ground = situation == *SITUATION_KIND_GROUND;

    let mut brake_x_var = WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
    if is_ground {brake_x_var *= mul_brake;}

    WorkModule::set_float(weapon.module_accessor,brake_x_var, REDSHELL_INSTANCE_FLOAT_BRAKE_X);

    let applied_brake_x = if !is_ground && status == REDSHELL_STATUS_KIND_SHOOT {0.0} else {brake_x_var};

    let grav = if weapon.is_grounded() {0.0} else {-GRAVITY};
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        applied_brake_x*-lr,
        grav
    );
}

pub unsafe extern "C" fn redshell_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    snap_to_owner(weapon,Hash40::new("have"),Hash40::new("haver"));
    0.into()
}
pub unsafe extern "C" fn redshell_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_pos = *PostureModule::pos(owner);
    let owner_scale = PostureModule::scale(owner);
    let lr = PostureModule::lr(owner);
    PostureModule::set_lr(weapon.module_accessor, lr);
    let new_pos = Vector3f::new(owner_pos.x+(6.0*lr*owner_scale), owner_pos.y+(3.8*owner_scale), owner_pos.z);
    PostureModule::set_pos(weapon.module_accessor, &new_pos);
    //snap_to_owner(weapon, Hash40::new("have"), Hash40::new("throw"));

    let rot_y = PostureModule::rot_y(owner, 0);
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("have"));
    ModelModule::clear_joint_srt(weapon.module_accessor, Hash40::new("rot"));
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 0.0, y: rot_y, z: 0.0}, 0);

    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    ModelModule::set_visibility(weapon.module_accessor, true);

    WorkModule::set_int(weapon.module_accessor, LIFE, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, LIFE, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_float(weapon.module_accessor, BRAKE_X_INIT, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
    WorkModule::off_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
    WorkModule::set_int(weapon.module_accessor, 0, REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN);
    WorkModule::off_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_FRIENDLY_FIRE);
    WorkModule::set_int(weapon.module_accessor, 0, REDSHELL_INSTANCE_INT_OTTOTTO_COUNTDOWN);
    WorkModule::set_int(weapon.module_accessor, *BATTLE_OBJECT_ID_INVALID, REDSHELL_INSTANCE_INT_EFF);
    WorkModule::set_int(weapon.module_accessor, FRIENDLY_FIRE_COOLDOWN, REDSHELL_INSTANCE_INT_FRIENDLY_FIRE_COUNTDOWN);
    
    
    let speed_x = SPEED_X*lr;//KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = 1.0;//KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    redshell_set_correct_kinetics(weapon);

    weapon.fastshift(L2CValue::Ptr(redshell_fly_main_loop as *const () as _))
}

unsafe extern "C" fn redshell_start_friendly_fire(weapon: &mut smashline::L2CWeaponCommon) {
    //if !WorkModule::is_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_FRIENDLY_FIRE) {
    if WorkModule::get_int(weapon.module_accessor,REDSHELL_INSTANCE_INT_FRIENDLY_FIRE_COUNTDOWN) == 0 {
        let status = StatusModule::status_kind(weapon.module_accessor);
        if status != REDSHELL_STATUS_KIND_FURAFURA {
            WorkModule::on_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_FRIENDLY_FIRE);
            TeamModule::set_team(weapon.module_accessor, -1, true);
            HitModule::set_no_team(weapon.module_accessor, true);
            ReflectModule::set_team_no(weapon.module_accessor, *TEAM_NONE);
        }
    }
}
unsafe extern "C" fn redshell_set_angle(weapon: &mut smashline::L2CWeaponCommon) {
    let mut rot_z = 0.0f32;
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    if situation == *SITUATION_KIND_GROUND {
        let touch_flag = GroundModule::get_touch_moment_flag(weapon.module_accessor);
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let lr = PostureModule::lr(weapon.module_accessor);
            let ground_normal = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let mut ground_angle = (ground_normal.y.abs()).atan2(ground_normal.x).to_degrees() - (90.0);
            //println!("Ground: X: {} Y: {} :{ground_angle}",ground_normal.x,ground_normal.y);
            rot_z = ground_angle;
        }
    }
    if rot_z.abs() > 70.0 {rot_z = 0.0;}
    let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(rot_x,0.0,rot_z), 0);
}

unsafe extern "C" fn redshell_check_for_turn(weapon: &mut smashline::L2CWeaponCommon) {
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    if situation != *SITUATION_KIND_GROUND {return;}
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let ottotto_check = (OTTOTTO_CHECK_MUL*speed_x.abs()) + SPEED_X;
    //let near_check = speed_x.abs() + 5.0;
    let mut is_ottotto = false;
    let ottotto_cooled = true//WorkModule::count_down_int(weapon.module_accessor, REDSHELL_INSTANCE_INT_OTTOTTO_COUNTDOWN, 0)
    || WorkModule::get_int(weapon.module_accessor, REDSHELL_INSTANCE_INT_OTTOTTO_COUNTDOWN) == 0;
    
    if situation == *SITUATION_KIND_GROUND 
    && ottotto_cooled {
        if GroundModule::is_ottotto(weapon.module_accessor, ottotto_check) {
            is_ottotto = true;
            WorkModule::set_int(weapon.module_accessor, 2, REDSHELL_INSTANCE_INT_OTTOTTO_COUNTDOWN);
            //let is_near = GroundModule::is_near_cliff(weapon.module_accessor, lr, near_check);
        }
    }

    if is_ottotto || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        PostureModule::reverse_lr(weapon.module_accessor);
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: -0.8, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        redshell_update_brake(weapon,2.0);
        redshell_start_friendly_fire(weapon);
    }
}
//UNUSED
unsafe extern "C" fn redshell_check_for_rebound(weapon: &mut smashline::L2CWeaponCommon) {
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let is_cooldown = WorkModule::get_int(weapon.module_accessor, REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN) > 0;
    let has_attack = is_cooldown || AttackModule::is_attack(weapon.module_accessor, 0, false);
    let reflect = !is_cooldown && AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR);
    if !has_attack || reflect {
        AttackModule::clear_all(weapon.module_accessor);
        let frame = MotionModule::frame(weapon.module_accessor);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("turn"), frame, 1.0, false, 0.0, false, false);
        HitModule::set_no_team(weapon.module_accessor, true);

        PostureModule::reverse_lr(weapon.module_accessor);
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: -0.9, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        redshell_update_brake(weapon,2.0);
        WorkModule::on_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_FRIENDLY_FIRE);
        if reflect {
            let countdown = 12;
            WorkModule::set_int(weapon.module_accessor, countdown, REDSHELL_INSTANCE_INT_FRIENDLY_FIRE_COUNTDOWN);
            WorkModule::set_int(weapon.module_accessor, countdown, REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN);
            let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(
                set_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                SPEED_X*lr*1.5,
                speed_y
            );
        }
        else {
            WorkModule::set_int(weapon.module_accessor, 10, REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN);
        }
    }
}

unsafe extern "C" fn redshell_fly_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    println!("loop LR: {lr}");
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    WorkModule::set_float(weapon.module_accessor, speed_x, REDSHELL_INSTANCE_FLOAT_SPEED);
    let mut brake_x_var = WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
    redshell_set_angle(weapon);

    //Kill on contact
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK) {
        weapon.change_status(REDSHELL_STATUS_KIND_FURAFURA.into(), false.into());
        return 1.into();
    }

    //Countdown
    if !StopModule::is_stop(weapon.module_accessor) {
        if speed_x.abs () < 0.2 {
            let timeout = WorkModule::get_int(weapon.module_accessor,REDSHELL_INSTANCE_INT_TIME_OUT_COUNT);
            if WorkModule::count_down_int(weapon.module_accessor,REDSHELL_INSTANCE_INT_TIME_OUT_COUNT, 0) {
                weapon.change_status(REDSHELL_STATUS_KIND_FURAFURA.into(), false.into());
                return 1.into();
            }
        }
        else {
            WorkModule::set_int(weapon.module_accessor,TIMEOUT_COOLDOWN, REDSHELL_INSTANCE_INT_TIME_OUT_COUNT);
        }
        if WorkModule::count_down_int(weapon.module_accessor,REDSHELL_INSTANCE_INT_RESPAWN_ATTACK_COUNTDOWN, 0) {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
        }
        if WorkModule::count_down_int(weapon.module_accessor,REDSHELL_INSTANCE_INT_FRIENDLY_FIRE_COUNTDOWN, 0) {
            redshell_start_friendly_fire(weapon);
        }
    }
        
    if !StatusModule::is_changing(weapon.module_accessor)
    && StatusModule::is_situation_changed(weapon.module_accessor) {
        let eff = WorkModule::get_int(weapon.module_accessor, REDSHELL_INSTANCE_INT_EFF) as u32;
        if eff != *BATTLE_OBJECT_ID_INVALID as u32 {
            EffectModule::kill(weapon.module_accessor, eff, false, false);
        }
        let prev_situation = StatusModule::prev_situation_kind(weapon.module_accessor);
        redshell_set_correct_kinetics(weapon);
    }
    if situation == *SITUATION_KIND_GROUND || true {
        redshell_check_for_turn(weapon);
        //redshell_check_for_rebound(weapon);

        let lr = PostureModule::lr(weapon.module_accessor);
        if speed_x.abs() < 0.5 && life > 0 {
            life = -1;
            WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            WorkModule::on_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
            weapon.change_status(REDSHELL_STATUS_KIND_FURAFURA.into(), false.into());
            return 1.into();
        }
    }
    if life <= 1 {
        if situation == *SITUATION_KIND_AIR {
            return redshell_kill(weapon);
        }
        let can_brake = !WorkModule::is_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
        if can_brake {
            WorkModule::on_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
            redshell_update_brake(weapon,5.0);
            weapon.change_status(REDSHELL_STATUS_KIND_FURAFURA.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn redshell_fly_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let eff = WorkModule::get_int(weapon.module_accessor, REDSHELL_INSTANCE_INT_EFF);
    if eff != *BATTLE_OBJECT_ID_INVALID {
        let rot_z = PostureModule::rot_z(weapon.module_accessor,0);
        EffectModule::set_rot(weapon.module_accessor, eff as u32, &Vector3f::new(rot_z, 0.0, 0.0));
    }

    0.into()
}

unsafe extern "C" fn redshell_kill(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_erace_smoke"),
        &Vector3f{x:pos.x,y:pos.y+0.0,z:pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        0.95,
        0,
        -1,
        false,
        0
    );
    0.into()
}

pub unsafe extern "C" fn redshell_furafura_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
pub unsafe extern "C" fn redshell_furafura_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    let correct = if weapon.is_grounded() {*GROUND_CORRECT_KIND_GROUND} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(correct));

    let mut speed_x = WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_SPEED);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let from_slow = WorkModule::is_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
    let start_frame = if (from_slow && false) {0.0} else {89.0};

    if !WorkModule::is_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE) {
        PostureModule::reverse_lr(weapon.module_accessor);
        speed_x *= 0.5;
    }
    let lr = PostureModule::lr(weapon.module_accessor);
    speed_x = (speed_x.abs()).min(0.5);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        (lr*speed_x),
        speed_y
    );
    redshell_update_brake(weapon,3.0);
    
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("furafura"), 89.0, 4.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(redshell_furafura_main_loop as *const () as _))
}

unsafe extern "C" fn redshell_furafura_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    redshell_set_angle(weapon);
    
    if MotionModule::is_end(weapon.module_accessor) {
        return redshell_kill(weapon);
    }
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_x.abs() < 0.05 {
        KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: 0.0, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        redshell_update_brake(weapon,0.0);
    }
    if !StatusModule::is_changing(weapon.module_accessor)
    && StatusModule::is_situation_changed(weapon.module_accessor) 
    && MotionModule::frame(weapon.module_accessor) > 100.0{
        let prev_situation = StatusModule::prev_situation_kind(weapon.module_accessor);
        redshell_set_correct_kinetics(weapon);
    }
    redshell_check_for_turn(weapon);

    0.into()
}
unsafe extern "C" fn redshell_frame(weapon: &mut smashline::L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    if is_out_of_bounds(weapon.module_accessor,weapon.lua_state_agent) {
        redshell_kill(weapon);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, redshell_frame);

	agent.status(Init, REDSHELL_STATUS_KIND_HAVED, empty_status);
	agent.status(Pre, REDSHELL_STATUS_KIND_HAVED, redshell_haved_pre);
	agent.status(Main, REDSHELL_STATUS_KIND_HAVED, redshell_haved_main);
	agent.status(End, REDSHELL_STATUS_KIND_HAVED, empty_status);

	agent.status(Pre, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_pre);
	agent.status(Init, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_init);
	agent.status(Main, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_main);
	agent.status(Exec, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_exec);
	agent.status(End, REDSHELL_STATUS_KIND_SHOOT, empty_status);

	agent.status(Pre, REDSHELL_STATUS_KIND_FURAFURA, redshell_furafura_pre);
	agent.status(Init, REDSHELL_STATUS_KIND_FURAFURA, empty_status);
	agent.status(Main, REDSHELL_STATUS_KIND_FURAFURA, redshell_furafura_main);
	agent.status(Exec, REDSHELL_STATUS_KIND_FURAFURA, redshell_fly_exec);
	agent.status(End, REDSHELL_STATUS_KIND_FURAFURA, empty_status);
}