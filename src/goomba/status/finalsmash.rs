use crate::imports::imports_status::*;

unsafe extern "C" fn final_dash_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(CheckAttack, *FIGHTER_PIKACHU_STATUS_KIND_FINAL_DASH, final_dash_attack);
}