use crate::imports::imports_agent::*;

unsafe fn ssm_scale(boma: &mut BattleObjectModuleAccessor) {
    if ModelModule::scale(boma) == WorkModule::get_param_float(boma, hash40("scale"), 0) {
        //0.92
        let goomba_size = 1.0375;
        ModelModule::set_scale(boma, goomba_size);
        AttackModule::set_attack_scale(boma, goomba_size, true);
        GrabModule::set_size_mul(boma, goomba_size);
    }; 
}

unsafe fn superleaf_visibility(boma: &mut BattleObjectModuleAccessor) {
    let has_superleaf_item = WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF);
    let superleaf_expression = WorkModule::is_flag(boma, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    let is_visible_mesh = has_superleaf_item || superleaf_expression;
    let is_visible_item = !has_superleaf_item;
   // println!("Item: {has_superleaf_item} Expression: {superleaf_expression}");

    ModelModule::set_mesh_visibility(boma, Hash40::new("superleaf_ear"), is_visible_mesh);
    ModelModule::set_mesh_visibility(boma, Hash40::new("superleaf_tail"), superleaf_expression);
    ItemModule::set_attach_item_visibility(boma, is_visible_item, *ATTACH_ITEM_GROUP_HAT as u8);
    ItemModule::set_attach_item_visibility(boma, !superleaf_expression, *ATTACH_ITEM_GROUP_HIP as u8);
}

pub unsafe extern "C" fn goomba_frame(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    ssm_scale(boma);
    superleaf_visibility(boma);
}

pub unsafe extern "C" fn goomba_frame_init(fighter: &mut L2CFighterCommon) {
    goomba_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, goomba_frame_init);
}
