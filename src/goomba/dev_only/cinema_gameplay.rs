use crate::imports::imports_agent::*;


pub unsafe fn shrink_all(fighter: &mut L2CFighterCommon) {
    let my_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    for entry_id in 0..8 {
        if entry_id == my_id {continue;}
        // get the active battle object id and add it to the list
        let id = get_active_battle_object_id_from_entry_id(entry_id as u32).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        let object = get_battle_object_from_id(id);
        if object.is_null() { continue; }
        let object = unsafe { &mut *object };
        let boma = (*object).module_accessor;
        let fighter_kind = smash::app::utility::get_kind(&mut *boma);
        if fighter_kind != *FIGHTER_KIND_PICHU {continue; }

        PostureModule::set_scale(boma, 0.5, true);
        let target_damage = 50.0;
        let damage = DamageModule::damage(boma,0);
        DamageModule::add_damage(boma, target_damage-damage, 0);
        //ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_MUSHD), 0, 0, false, false);
        //ItemModule::use_item(boma, 0, false);
    }
}
pub unsafe fn add_damage_all(fighter: &mut L2CFighterCommon) {
    let my_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    for entry_id in 0..8 {
        if entry_id == my_id {continue;}
        // get the active battle object id and add it to the list
        let id = get_active_battle_object_id_from_entry_id(entry_id as u32).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        let object = get_battle_object_from_id(id);
        if object.is_null() { continue; }
        let object = unsafe { &mut *object };
        let boma = (*object).module_accessor;
        let fighter_kind = smash::app::utility::get_kind(&mut *boma);
        //if fighter_kind == *FIGHTER_KIND_PICHU {continue; }

        if DamageModule::damage(boma,0) < 300.0 {
            DamageModule::add_damage(boma, 900.0, 0);
        }
    }
}
pub unsafe fn set_status_all(fighter: &mut L2CFighterCommon, new_status: i32) {
    let my_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    for entry_id in 0..8 {
        if entry_id == my_id {continue;}
        // get the active battle object id and add it to the list
        let id = get_active_battle_object_id_from_entry_id(entry_id as u32).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        let object = get_battle_object_from_id(id);
        if object.is_null() { continue; }
        let object = unsafe { &mut *object };
        let boma = (*object).module_accessor;
        let fighter_kind = smash::app::utility::get_kind(&mut *boma);
        if fighter_kind == *FIGHTER_KIND_PICHU {continue; }

        StatusModule::change_status_request(boma, new_status, true);
        ControlModule::add_clatter_time(boma, 90000.0, 0);
    }
}

pub unsafe extern "C" fn cinema_frame(fighter: &mut L2CFighterCommon) {
    let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_PICHU {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    let common = &mut smashline::Agent::new("fighter");
    common.on_line(Main, cinema_frame);
    common.install();
}