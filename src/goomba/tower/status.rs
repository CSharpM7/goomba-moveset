use crate::imports::imports_agent::*;

pub unsafe extern "C" fn tower_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner = &mut *sv_battle_object::module_accessor(
        (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32
    );
    let num_goombas = ArticleModule::get_num(owner, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);

    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("wing"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("helmet"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("superleaf_ear"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("superleaf_tail"), false);
    
    LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
    LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
    LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_MODEL_SCALE as u8}, true);

    let owner_scale = ModelModule::scale(owner);
    ModelModule::set_scale(weapon.module_accessor, owner_scale);

    let status = StatusModule::status_kind(weapon.module_accessor);
    println!("Goombas: {num_goombas} Status: {status}");
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::set_frame_sync_anim_cmd(weapon.module_accessor, 20.0, true, true, false);


    weapon.fastshift(L2CValue::Ptr(tower_main_loop as *const () as _))
}

unsafe extern "C" fn tower_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) 
    && MotionModule::frame(weapon.module_accessor) > 30.0 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub unsafe extern "C" fn tower_frame(fighter: &mut L2CFighterCommon) {
    println!("!");
}

pub fn install(agent: &mut smashline::Agent) {
    println!("PLEASE");
	agent.status(Main, 0, tower_main);
	//agent.on_start(tower_main);
    //agent.on_line(Main, tower_frame);
}