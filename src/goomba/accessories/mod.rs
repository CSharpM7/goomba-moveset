mod acmd;
mod status;
use crate::singleslot::MOD_SLOTS;
use crate::vars::*;

pub fn install_hook(hookstatus: bool) {
    unsafe {
        FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES += FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST +
         smashline::clone_weapon("murabito", *smash::lib::lua_const::WEAPON_KIND_MURABITO_UMBRELLA, 
    "pikachu", "accessories",true);
    }

    if !hookstatus {return;}
    let agent = &mut smashline::Agent::new("pikachu_accessories");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);
    status::install(agent);
    agent.install();
}

pub fn install(hookstatus: bool) {
    #[cfg(feature = "dev")]{
        unsafe {
            FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES += FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST;
        }
    }

    let agent = &mut smashline::Agent::new("pikachu_accessories");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    acmd::install(agent);
    if !hookstatus {
        status::install(agent);
    }
    agent.install();
}

use crate::imports::imports_status::*;
pub unsafe fn is_goomba_accessories(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    return true;//lua_bind::MotionModule::is_anim_resource(module_accessor, Hash40::new("win_2"));
}
unsafe fn init_common(module_accessor: *mut BattleObjectModuleAccessor) {
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollipop"), false);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollitop"), false);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("boot"), false);

    /*
    let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = sv_battle_object::module_accessor(owner_id);
    let owner_status = StatusModule::status_kind(owner);
    if LinkModule::is_model_constraint(module_accessor) {
        if LinkModule::is_model_constraint_mutual(module_accessor) {
        }
        LinkModule::remove_model_constraint(module_accessor,true);
    }
    let mut has_link = LinkModule::is_link(module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    if !has_link {
        let link_created = LinkModule::link(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,owner_id);
        has_link = link_created & 1 != 0;
    }
    if has_link {
        LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SLOW as u8}, true);
        LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
        //LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_POS as u8}, true);
        LinkModule::set_attribute(module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_FLIP as u8}, true);
    } 
    */
}
pub unsafe fn init_lolipop(module_accessor: *mut BattleObjectModuleAccessor) {
    init_common(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollipop"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollitop"), true);
    
    let parent_bone = Hash40::new("haver");
    LinkModule::set_model_constraint_pos_ort(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("haver"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
}