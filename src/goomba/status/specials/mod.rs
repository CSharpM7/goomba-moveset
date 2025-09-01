mod specialn;
mod specials;
mod specialhi;
mod speciallw;

pub fn install(agent: &mut smashline::Agent) {
    specialn::install(agent);
    specials::install(agent);
    specialhi::install(agent);
    speciallw::install(agent);
}

use crate::imports::imports_status::*;
unsafe extern "C" fn status_update_motion(fighter: &mut L2CFighterCommon, is_loop: bool) {
    let mot_g =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    let mot_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    if !is_loop {
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        let motion = if situation == *SITUATION_KIND_GROUND {mot_g} else {mot_a};
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        fighter.sub_change_motion_by_situation(Hash40::new_raw(mot_g).into(), Hash40::new_raw(mot_a).into(), true.into());
    }
}
unsafe extern "C" fn status_kinetic_correct(fighter: &mut L2CFighterCommon, ground_kinetic: i32, air_kinetic: i32, walk_off: bool) {
    fighter.sub_change_kinetic_type_by_situation(ground_kinetic.into(), air_kinetic.into());
    fighter.sub_set_ground_correct_by_situation((!walk_off).into());
}
unsafe extern "C" fn status_on_situation_update(fighter: &mut L2CFighterCommon, 
    is_loop: bool, ground_kinetic: i32, air_kinetic: i32, walk_off: bool) 
{
    status_update_motion(fighter,is_loop);
    status_kinetic_correct(fighter,ground_kinetic,air_kinetic,walk_off);
}