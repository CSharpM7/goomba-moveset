//mod acmd;
mod status;
use crate::singleslot::MOD_SLOTS;
use crate::vars::*;

pub fn install_hook(hookstatus: bool) {
    unsafe {
        FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES += FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST +
            smashline::clone_weapon("peach", *ACCESSORIES_KIND, 
        "pichu", "accessories",false);
        println!("[smashline_kuribo::kuribo] (HOOK) Accessories assigned to {}",FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
    }

    if !hookstatus {return;}
    let agent = &mut smashline::Agent::new("pichu_accessories");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);
    status::install(agent);
    agent.install();
}

pub fn install(hookstatus: bool) {
    #[cfg(feature = "dev")]{
        unsafe {
            FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES = FIGHTER_GOOMBA_GENERATE_ARTICLE_LAST + 0;
            println!("[smashline_kuribo::kuribo] (DEV) Accessories assigned to {}",FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES);
        }
    }

    let agent = &mut smashline::Agent::new("pichu_accessories");
    let slots = (*MOD_SLOTS.read().unwrap()).to_vec();
    agent.set_costume(slots);

    //acmd::install(agent);
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
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("book"), false);
}
pub unsafe fn init_lolipop(module_accessor: *mut BattleObjectModuleAccessor) {
    init_common(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollipop"), true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("lollitop"), true);
    
    let parent_bone = Hash40::new("haver");
    LinkModule::set_model_constraint_pos_ort(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("food"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    MotionModule::change_motion(module_accessor, Hash40::new("null"), 0.0, 0.0, false, 0.0, false, false);
}
pub unsafe fn init_shoe(module_accessor: *mut BattleObjectModuleAccessor) {
    init_common(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("boot"), true);
    
    LinkModule::remove_model_constraint(module_accessor,true);
    let parent_bone = Hash40::new("hip");
    LinkModule::set_model_constraint_pos_ort(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("boot"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);

    MotionModule::change_motion(module_accessor, Hash40::new("null"), 0.0, 0.0, false, 0.0, false, false);
}
pub unsafe fn init_book(module_accessor: *mut BattleObjectModuleAccessor) {
    init_common(module_accessor);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("book"), true);
    
    let parent_bone = Hash40::new("throw");
    LinkModule::set_model_constraint_pos_ort(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,Hash40::new("have"),parent_bone,
    (*CONSTRAINT_FLAG_MTX 
         | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32,true);
    MotionModule::change_motion(module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
}