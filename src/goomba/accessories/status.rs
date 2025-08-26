use crate::imports::imports_agent::*;

pub unsafe extern "C" fn accessories_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
pub unsafe extern "C" fn accessories_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("Lolipop init");
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    ModelModule::set_visibility(weapon.module_accessor, true);
    return 0.into();

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
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("haver"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true); 
    0.into()
}

pub unsafe extern "C" fn accessories_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("s"), 0.0, 1.0, false, 0.0, false, false);
    ModelModule::set_visibility(weapon.module_accessor, true);
    
    weapon.fastshift(L2CValue::Ptr(accessories_main_loop as *const () as _))
}

unsafe extern "C" fn accessories_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    /*
	agent.status(Pre, 0, accessories_pre);
	agent.status(Init, 0, accessories_init);
	agent.status(Main, 0, accessories_main);
	agent.status(End, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status);
	agent.status(MapCorrection, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status);
	agent.status(FixCamera, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status);
	agent.status(Exit, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status);
	agent.status(Exec, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status);
	agent.status(ExecStop, *WEAPON_PICHU_DENGEKI_STATUS_KIND_REGULAR, empty_status); 
    */
}