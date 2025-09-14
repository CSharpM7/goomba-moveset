use crate::imports::imports_acmd::*;

mod attack;
//mod attacks3;
//mod attackhi3;
mod attacklw3;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
    //attacks3::install(agent);
    //attackhi3::install(agent);
    attacklw3::install(agent);
}
