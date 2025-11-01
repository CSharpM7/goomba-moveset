use smash::{lib::lua_const::*, hash40};

#[cfg(not(feature = "devhook"))]
mod acmd;
#[cfg(not(feature = "devhook"))]
mod agent_init;
#[cfg(not(feature = "devhook"))]
mod frame;
#[cfg(not(feature = "devhook"))]
mod dev_only;

#[cfg(not(feature = "dev"))]
mod vtable;

mod status;
pub mod accessories;
pub mod redshell;
pub mod tower;

pub mod articles {
    pub use {
        super::accessories::*,
        super::redshell::*,
        super::tower::*,
    };
}
use crate::singleslot::MOD_SLOTS;

pub fn install_hook() {
    println!("[smashline_kuribo::kuribo] Installing Hook");
    let mut hookstatus = false;

    let agent = &mut smashline::Agent::new("pichu");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    #[cfg(feature = "hookstatus")] {
        println!("[smashline_kuribo::kuribo] Installing Status Scripts in Hook");
        hookstatus=true;
        

        agent_init::install(agent);
        status::install(agent);
    }
    #[cfg(not(feature = "dev"))] 
    vtable::install();

    agent.install();
    accessories::install_hook(hookstatus);
    redshell::install_hook(hookstatus);
    tower::install_hook(hookstatus);

    
    println!("[smashline_kuribo::kuribo] ");
}
#[cfg(not(feature = "devhook"))]
pub fn install() {
    println!("[smashline_kuribo::kuribo] Installing Under Slots:");
    crate::singleslot::print_slots();
    let mut hookstatus = true;
    let agent = &mut smashline::Agent::new("pichu");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);
    
    #[cfg(feature = "dev")]
    dev_only::install(agent);

    acmd::install(agent);
    frame::install(agent);
    #[cfg(not(feature = "hookstatus"))]
    {
        println!("[smashline_kuribo::kuribo] Installing Status Scripts");
        hookstatus=false;
        agent_init::install(agent);
        status::install(agent);
    }
    agent.install();

    accessories::install(hookstatus);
    redshell::install(hookstatus);
    tower::install(hookstatus);
    println!("[smashline_kuribo::kuribo] ");
}