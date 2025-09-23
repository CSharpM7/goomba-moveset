pub fn init() {
    read_toml();
}

const IDENTIFIER: &str = "goomba.toml";

use serde_derive::Deserialize;
use serde::Serialize;
use skyline::nn;
use std::fs;
use toml;

#[derive(Default,Deserialize, Serialize)]
struct ConfigToml {
    first_boot: Option::<bool>
}

fn create_toml(config_file: String) {
    use toml::{map::Map, Value}; // 0.5.1
    
    let mut default_toml = ConfigToml::default();
    default_toml.first_boot = Some(true);

    let toml_string = toml::to_string(&default_toml).expect("Could not encode TOML value");
    println!("{}", toml_string);
    fs::write(config_file, toml_string).expect("Could not write to file!");
}

pub fn read_toml() {
    unsafe {
        //READ TOML
        let mut needs_toml = false;
        let config_file = format!("{}/{}", "sd:/ultimate/movesets",IDENTIFIER);
        let contents = match fs::read_to_string(config_file.as_str()) {
            Ok(c) => c,
            Err(_) => {
                println!("[smashline_kuribo] No kuribo.toml");
                create_toml(config_file);
                show_first_time_message();
                return;
            }
        };
        let data: ConfigToml = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                println!("[smashline_kuribo] Error reading kuribo.toml");
                create_toml(config_file);
                show_first_time_message();
                return;
            }
        };
        if !data.first_boot.is_some(){
            show_first_time_message();
        }
    }
}
pub fn show_first_time_message() {
    use skyline_web::*;
    skyline_web::dialog_ok::DialogOk::ok("Don't forget to check the Tips page for info on Goomba!");
}