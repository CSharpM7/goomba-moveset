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


mod imports;
mod singleslot;
mod goomba;

pub mod vars;

//mod training;

static mut SHOULD_INSTALL : bool = true;
static mut SHOULD_INSTALL_SSM : bool = true;

fn show_error(error_message: &str) {
    use skyline::nn::{err, settings};

    let message = "Goomba installation might be incomplete! \nSelect Cancel to continue, or Details for more information.";
    let details_append = "You can always edit sd:/ultimate/movesets/goomba.toml to skip the plugin validation checks if this is a false error.";
    let details = format!("{} \n{}", error_message, details_append);
    let mut message_bytes = String::from(message).into_bytes();
    let mut details_bytes = String::from(details.as_str()).into_bytes();

    if message_bytes.len() >= 2048 {
        message_bytes.truncate(2044);
        message_bytes.append(&mut String::from("...\0").into_bytes());
    }
    if details_bytes.len() >= 2048 {
        details_bytes.truncate(2044);
        details_bytes.append(&mut String::from("...\0").into_bytes());
    }
    unsafe {
        let error = err::ApplicationErrorArg::new_with_messages(
            1,
            core::str::from_utf8(&message_bytes).unwrap().as_bytes().as_ptr(),
            core::str::from_utf8(&details_bytes).unwrap().as_bytes().as_ptr(),
            &settings::LanguageCode_Make(settings::Language_Language_English)
        );

        err::ShowApplicationError(&error);
    }
}

pub fn quick_validate_install() {
    //plugin checks
    //std::fs::metadata(modFolder.clone()).is_ok()
    let has_param_config = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libparam_config.nro").is_ok();
    let has_csk = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libthe_csk_collection.nro").is_ok();
    let has_nro_hook = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libnro_hook.nro").is_ok();
    let has_smashline = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_plugin.nro").is_ok();
    let has_skyline = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/exefs/").is_ok();
    //let toreturn = has_param_config && has_csk && has_nro_hook && has_smashline && has_skyline;

    if !has_param_config {
        show_error("Param Config not found! Please install libparam_config.nro to the plugin folder!");
    }
    if !has_csk {
        show_error("CSK Collection not found! Please install libthe_csk_collection.nro to the plugin folder!");
    }
    if !has_nro_hook {
        show_error("NRO Hook not found! Please install libnro_hook.nro to the plugin folder!");
    }
    if !has_smashline {
        show_error("Smashline 2 not found! Please install smashline_plugin.nro to the plugin folder!");
    }
    if !has_skyline {
        show_error("Skyline not found! Please install skyline before using movesets!");
    }

    //return toreturn;
}

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