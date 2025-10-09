use crate::imports::imports_agent::*;

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = fighter.status_Appeal();
    fighter.change_status(FIGHTER_STATUS_KIND_ENTRY.into(),false.into());
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);

    return to_return;
    return fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _));
}

unsafe extern "C" fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("win_2_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

pub unsafe extern "C" fn common_frame(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if speed_x.abs() > 0.5 {
            println!("Speed X: {speed_x}");
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    /*
    let agent = &mut smashline::Agent::new("fighter")
    .on_line(Main, common_frame)
    .install(); 
    */
}