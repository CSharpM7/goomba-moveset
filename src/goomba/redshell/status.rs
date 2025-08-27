use crate::imports::imports_agent::*;

pub const LIFE: i32 = 300;
pub const BRAKE_X_INIT: f32 = 0.001;

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
    println!("Artcile Spawned");
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if LinkModule::is_model_constraint(weapon.module_accessor) {
        println!("Haved");
    }
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
    let parent_bone = Hash40::new("haver");
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("have"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
         
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);

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
unsafe extern "C" fn redshell_set_situation(weapon: &mut smashline::L2CWeaponCommon) {
    let is_ground = false;//GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let situation = if is_ground {*SITUATION_KIND_GROUND} else {*SITUATION_KIND_AIR};
    StatusModule::set_situation_kind(weapon.module_accessor, SituationKind(situation), false);
}
unsafe extern "C" fn redshell_set_kinetics(weapon: &mut smashline::L2CWeaponCommon) {
    let lr = PostureModule::lr(weapon.module_accessor);
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let is_ground = situation == *SITUATION_KIND_GROUND;
    let correct = if is_ground {*GROUND_CORRECT_KIND_GROUND} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(correct));

    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut brake_x = WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
    let mut accel_y = 0.0;
    println!("Is ground? {is_ground}");
    if is_ground {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
    }
    else {
        println!("GRAVITY PLEASE");
        accel_y = -0.2;
        brake_x = 0.0;
    }
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        brake_x*-lr,
        accel_y
    );
}
pub unsafe extern "C" fn redshell_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    snap_to_owner(weapon,Hash40::new("have"),Hash40::new("haver"));
    0.into()
}
pub unsafe extern "C" fn redshell_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("Artcile Shot");
    
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    let mut has_link = LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        LinkModule::unlink(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    }
    //Set Motion
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_pos = *PostureModule::pos(owner);
    let lr = PostureModule::lr(owner);
    PostureModule::set_lr(weapon.module_accessor, lr);
    //PostureModule::set_pos(weapon.module_accessor, &Vector3f{x:owner_pos.x, y:owner_pos.y, z:owner_pos.z});

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    ModelModule::set_visibility(weapon.module_accessor, true);

    WorkModule::set_int(weapon.module_accessor, LIFE, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, LIFE, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_float(weapon.module_accessor, BRAKE_X_INIT, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
    WorkModule::off_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
    
    let speed_x = 1.0*lr;//KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = -0.1;//KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    redshell_set_situation(weapon);
    redshell_set_kinetics(weapon);

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
    weapon.fastshift(L2CValue::Ptr(redshell_fly_main_loop as *const () as _))
}

unsafe extern "C" fn redshell_fly_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if !StatusModule::is_changing(weapon.module_accessor)
    && StatusModule::is_situation_changed(weapon.module_accessor) {
        let prev_situation = StatusModule::prev_situation_kind(weapon.module_accessor);
        println!("New situation: {prev_situation} > {situation}");
        redshell_set_kinetics(weapon);
    }
    if situation == *SITUATION_KIND_GROUND {
        let mut lr = PostureModule::lr(weapon.module_accessor);
        if GroundModule::is_ottotto(weapon.module_accessor, 1.5)
        || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            lr *= -1.0;
            PostureModule::reverse_lr(weapon.module_accessor);
            //PostureModule::update_rot_y_lr(weapon.module_accessor);
            KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: -0.9, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            let brake_x = 2.0 * WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
            WorkModule::set_float(weapon.module_accessor,brake_x, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
            sv_kinetic_energy!(
                set_accel,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                brake_x*-lr,
                0.0
            );
        }
        let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if speed_x.abs() < 0.05
        || speed_x.signum() != lr {
            return redshell_kill(weapon);
        }
        else if speed_x.abs() < 0.5 && life > 0 {
            life = -1;
            WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            println!("Asshole");
        }
        let brake_x = WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
        println!("Life: {life} Speed: {speed_x} Brake: {brake_x}");
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            brake_x*-lr,
            0.0
        );
    }

    if life <= 0 {
        if situation == *SITUATION_KIND_AIR {
            return redshell_kill(weapon);
        }
        let can_brake = !WorkModule::is_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
        println!("Has brake: {can_brake}");
        if can_brake {
            println!("BIG BRAKE");
            WorkModule::on_flag(weapon.module_accessor, REDSHELL_INSTANCE_FLAG_BIG_BRAKE);
            let mut brake_x = 10.0 * WorkModule::get_float(weapon.module_accessor, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
            WorkModule::set_float(weapon.module_accessor,brake_x, REDSHELL_INSTANCE_FLOAT_BRAKE_X);
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_greenshell_trace"), false, false);
            AttackModule::clear_all(weapon.module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn redshell_fly_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE,0);
    0.into()
}
unsafe extern "C" fn redshell_kill(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_erace_smoke"),
        &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        0.625,
        0,
        -1,
        false,
        0
    );
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
	agent.status(Init, 0, empty_status);
	agent.status(Pre, 0, redshell_haved_pre);
	agent.status(Main, 0, redshell_haved_main);
	agent.status(End, 0, empty_status);

	agent.status(Init, 1, empty_status);
	agent.status(Pre, 1, redshell_haved_pre);
	agent.status(Main, 1, empty_status);
	agent.status(End, 1, empty_status);

	agent.status(Pre, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_pre);
	agent.status(Init, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_init);
	agent.status(Main, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_main);
	agent.status(Exec, REDSHELL_STATUS_KIND_SHOOT, redshell_fly_exec);
	agent.status(End, REDSHELL_STATUS_KIND_SHOOT, empty_status);
}