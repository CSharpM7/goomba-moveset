#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    static_mut_refs
)]
#![deny(
    deprecated
)]
#[macro_use]
extern crate lazy_static;

mod goomba;
mod imports;
mod singleslot;
//mod data;

pub mod vars;

static mut SHOULD_INSTALL : bool = true;

pub fn install_hook() {
    goomba::install_hook();
}

#[no_mangle]
pub fn smashline_install() {
    println!("[smashline_kuribo::main] Reloading...");
    install();
}
/*
#[no_mangle]
pub fn smashline_uninstall() {
    uninstall();
}
 */
pub fn install() {   
    #[cfg(not(feature = "devhook"))] 
    goomba::install();
}
pub fn uninstall() {
    println!("Uninstalling...");
}

#[skyline::main(name = "goomba")]
pub fn main() {
    #[cfg(feature = "dev")]{
        smashline_install();
        return;
    }
    #[cfg(feature = "devhook")]{
        for _ in 0..10 {
            println!("              PREINSTALL");
        }
    }
    
    //quick_validate_install();
    singleslot::install();
}

pub fn install_after_mount() {
    unsafe {
        if !SHOULD_INSTALL {
            println!("[smashline_kuribo::main] Could not load plugin");
            return;
        }
    }

	install_hook();
    #[cfg(feature = "devhook")]{
        println!("[smashline_kuribo::main] Dev Hook Installed");
        for _ in 0..2 {
            println!("");
        }
        return;
    }
    
    install();

    println!("[smashline_kuribo::main] Loading...");
    for _ in 0..2 {
        println!("");
    }
}