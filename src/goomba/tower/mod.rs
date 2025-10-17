mod acmd;
mod status;
use crate::singleslot::MOD_SLOTS;
use crate::vars::*;

pub fn install_hook(hookstatus: bool) {
    smashline::update_weapon_count(*smash::lib::lua_const::WEAPON_KIND_PICHU_MONSTERBALL, 2);
    //param_config::set_article_use_type(*WEAPON_KIND_PICHU_MONSTERBALL, *ARTICLE_USETYPE_FINAL);

    if !hookstatus {return;}
    let agent = &mut smashline::Agent::new("pichu_monsterball");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);
    status::install(agent);
    agent.install();
}

pub fn install(hookstatus: bool) {
    let agent = &mut smashline::Agent::new("pichu_monsterball");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    acmd::install(agent);
    if !hookstatus {
        status::install(agent);
    }
    agent.install();
}