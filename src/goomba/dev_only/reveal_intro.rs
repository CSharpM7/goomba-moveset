use crate::imports::imports_agent::*;

pub static mut GOOMBA_CINEMA_STATE: i32 = 0;

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
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            GOOMBA_CINEMA_STATE = 1;
            set_status_all(fighter,*FIGHTER_STATUS_KIND_FURAFURA_STAND);
        }
        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            GOOMBA_CINEMA_STATE = 2;
            //ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
            let damage = DamageModule::damage(fighter.module_accessor,0);
            DamageModule::add_damage(fighter.module_accessor, 60.0-damage, 0);
        }
        else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            add_damage_all(fighter);
            GOOMBA_CINEMA_STATE = 3;
        }
    }
    else if fighter_kind == *FIGHTER_KIND_PICHU {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if status == *FIGHTER_STATUS_KIND_ENTRY {
            GOOMBA_CINEMA_STATE = 0;
        }
        else if GOOMBA_CINEMA_STATE > 0 {
            PostureModule::set_lr(fighter.module_accessor, 1.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            if GOOMBA_CINEMA_STATE == 1 {
                VisibilityModule::set_whole(fighter.module_accessor, false);
                let new_pos = Vector3f::new(-70.0,40.0,0.0);
                PostureModule::set_pos(fighter.module_accessor, &new_pos);
                if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                }
                KineticModule::clear_speed_all(fighter.module_accessor);
            }
            else {
                VisibilityModule::set_whole(fighter.module_accessor, true);
                if PostureModule::pos_y(fighter.module_accessor) > 30.0 && GOOMBA_CINEMA_STATE == 2 {
                    if status != *FIGHTER_STATUS_KIND_ATTACK_AIR {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                    }
                    else if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_lw") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    let common = &mut smashline::Agent::new("fighter");
    common.on_line(Main, cinema_frame);
    common.install();
}