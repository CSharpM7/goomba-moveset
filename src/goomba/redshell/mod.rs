mod acmd;
mod status;
use crate::singleslot::MOD_SLOTS;
use crate::vars::*;

pub fn install_hook(hookstatus: bool) {
    unsafe {
        FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL += FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST +
     smashline::clone_weapon("miiswordsman", *smash::lib::lua_const::WEAPON_KIND_MIISWORDSMAN_CHAKRAM, 
"pichu", "redshell",true);
        println!("[smashline_kuribo::kuribo] (HOOK) Redshell assigned to {}",FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL);
    }

    if !hookstatus {return;}
    let agent = &mut smashline::Agent::new("pichu_redshell");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);
    status::install(agent);
    agent.install();
}

pub fn install(hookstatus: bool) {
    #[cfg(feature = "dev")]{
        unsafe {
            FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL = FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST + 1;
            println!("[smashline_kuribo::kuribo] (DEV) Redshell assigned to {}",FIGHTER_GOOMBA_GENERATE_ARTICLE_REDSHELL);
        }
    }

    let agent = &mut smashline::Agent::new("pichu_redshell");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    acmd::install(agent);
    if !hookstatus {
        status::install(agent);
    }
    agent.install();
}