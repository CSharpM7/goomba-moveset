//mod acmd;
mod status;
use crate::singleslot::MOD_SLOTS;
use crate::vars::*;

pub fn install_hook(hookstatus: bool) {
    smashline::update_weapon_count(*WEAPON_KIND_PICHU_MONSTERBALL, 2);
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

    //acmd::install(agent);
    if !hookstatus {
        status::install(agent);
    }
    agent.install();
}

use crate::imports::imports_status::*;

pub unsafe fn init(module_accessor: *mut BattleObjectModuleAccessor) {
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("wing"), false);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("helmet"), false);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("superleaf_ear"), false);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("superleaf_tail"), false);

    LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
    LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
    LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_MODEL_SCALE as u8}, true);

    let owner = &mut *sv_battle_object::module_accessor(
        (WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32
    );
    let num_goombas = ArticleModule::get_num(owner, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);

    MotionModule::change_motion(module_accessor, Hash40::new("entry_r"), 0.0, 20.0, false, 0.0, false, false);
    let status = StatusModule::status_kind(module_accessor);
    println!("Goombas: {num_goombas} Status: {status}");
}