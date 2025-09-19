use crate::imports::imports_agent::*;

pub const LIFE: f32 = 40.0;
pub const GRAVITY: f32 = 0.1;
pub unsafe extern "C" fn accessories_haved_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
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
pub unsafe extern "C" fn accessories_haved_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(accessories_haved_main_loop as *const () as _))
}

unsafe extern "C" fn accessories_haved_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn accessories_ejected_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
pub unsafe extern "C" fn accessories_ejected_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    LinkModule::remove_model_constraint(weapon.module_accessor,true);
    GroundModule::set_rhombus_offset(weapon.module_accessor, &Vector2f{ x: 0.0, y: 4.0});
    
    WorkModule::set_float(weapon.module_accessor,LIFE, ACCESSORIES_INSTANCE_FLOAT_LIFE);

    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_lr = PostureModule::lr(owner);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        owner_lr*-1.0,
        2.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        -GRAVITY
    );
    sv_kinetic_energy!(
        set_brake,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.03
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        3.0,
        2.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        2.0
    );
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        4.0,
        0.0,
        0.0
    ); 
    weapon.fastshift(L2CValue::Ptr(accessories_ejected_main_loop as *const () as _))
}

unsafe extern "C" fn accessories_ejected_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let situation = StatusModule::situation_kind(weapon.module_accessor);
    let mut die = false;

    if !StatusModule::is_changing(weapon.module_accessor)
    && StatusModule::is_situation_changed(weapon.module_accessor) {
        if situation == *SITUATION_KIND_GROUND {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            sv_kinetic_energy!(
                set_speed,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                0.0,
                0.0
            );
            KineticModule::unable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            KineticModule::unable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
        }
        else {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        }
    }
    if situation == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x,
            0.0
        );
        die=true;
    }
    WorkModule::add_float(weapon.module_accessor, -1.0, ACCESSORIES_INSTANCE_FLOAT_LIFE);
    let count = WorkModule::get_float(weapon.module_accessor,ACCESSORIES_INSTANCE_FLOAT_LIFE);
    //println!("Life: {count}");
    if count <= 0.0 {
        die = true;
    }
    if die {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        let pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_erace_smoke"),
            &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
            &Vector3f{x:0.0,y:0.0,z:0.0},
            1.1,
            0,
            -1,
            false,
            0
        );
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Pre, ACCESSORIES_STATUS_KIND_HAVED, accessories_haved_pre);
	agent.status(Main, ACCESSORIES_STATUS_KIND_HAVED, accessories_haved_main);
	agent.status(End, ACCESSORIES_STATUS_KIND_HAVED, empty_status);

	agent.status(Pre, ACCESSORIES_STATUS_KIND_EJECTED, accessories_ejected_pre);
	agent.status(Main, ACCESSORIES_STATUS_KIND_EJECTED, accessories_ejected_main);
	agent.status(End, ACCESSORIES_STATUS_KIND_EJECTED, empty_status);
}