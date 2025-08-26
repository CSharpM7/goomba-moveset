use smash::{lib::lua_const::*, hash40};

mod acmd;
mod agent_init;
mod frame;
mod status;
//mod vtable;
//pub mod common;

mod temp;

//pub mod speeder;
//pub mod surfboard;
pub mod accessories;
//pub mod tongue;

pub mod articles {
    pub use {
        //super::speeder::*,
        //super::surfboard::*,
        super::accessories::*,
        //super::tongue::*,
    };
}
use crate::singleslot::MOD_SLOTS;

pub fn install_hook() {
    println!("[smashline_kuribo::kuribo] Installing Hook");
    let mut hookstatus = false;
	//vtable::install();
    #[cfg(feature = "hookstatus")] {
        println!("[smashline_kuribo::kuribo] Installing Status Scripts in Hook");
        hookstatus=true;
        
        let agent = &mut smashline::Agent::new("dedede");
        let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
        agent.set_costume(slots);

        agent_init::install(agent);
        status::install(agent);
        agent.install();
    }
    accessories::install_hook(hookstatus);
    println!("[smashline_kuribo::kuribo] ");
}
pub fn install() {
    println!("[smashline_kuribo::kuribo] Installing Under Slots:");
    crate::singleslot::print_slots();
    let mut hookstatus = true;
    let agent = &mut smashline::Agent::new("pichu");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    acmd::install(agent);
    frame::install(agent);
    temp::install(agent);
    #[cfg(not(feature = "hookstatus"))]
    {
        println!("[smashline_kuribo::kuribo] Installing Status Scripts");
        hookstatus=false;
        agent_init::install(agent);
        status::install(agent);
    }
    agent.install();

    //speeder::install(hookstatus);
    accessories::install(hookstatus);
    println!("[smashline_kuribo::kuribo] ");
}