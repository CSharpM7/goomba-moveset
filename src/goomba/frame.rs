use crate::imports::imports_agent::*;

unsafe fn ssm_scale(boma: &mut BattleObjectModuleAccessor){
    if ModelModule::scale(boma) == WorkModule::get_param_float(boma, hash40("scale"), 0) {
        ModelModule::set_scale(boma, 0.85);
        AttackModule::set_attack_scale(boma, 0.85, true);
        GrabModule::set_size_mul(boma, 0.85);
    };
}

pub unsafe extern "C" fn rash_frame(fighter: &mut L2CFighterCommon) {
    let boma = &mut *fighter.module_accessor;
    ssm_scale(boma);
    
    let frame = MotionModule::frame(fighter.module_accessor);
    let status = StatusModule::status_kind(fighter.module_accessor);
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let is_flip = MotionModule::is_flip(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let rot_y = PostureModule::rot_y(fighter.module_accessor,0);
    /* 
    if (status == *FIGHTER_STATUS_KIND_LANDING
    || status == *FIGHTER_STATUS_KIND_LANDING_LIGHT)
    ||
    (status == *FIGHTER_STATUS_KIND_WAIT
    && frame <= 0.0) {
        println!("F: {is_flip} S: {status} LR: {lr} RotY: {rot_y} ");
    }*/
    if [*FIGHTER_STATUS_KIND_TURN,*FIGHTER_STATUS_KIND_TURN_RUN,*FIGHTER_STATUS_KIND_TURN_DASH].contains(&status) {
        if frame < 2.0 {
            //println!("Flip Joint");
        }
        if ItemModule::get_have_item_size(fighter.module_accessor, 0) != *ITEM_SIZE_HEAVY as u64 {
            let item_joint = if lr > 0.0 {Hash40::new("haver")} else {Hash40::new("haver")};
            ItemModule::set_have_item_constraint_joint(fighter.module_accessor,item_joint,0);
        }
        else {
            let item_joint = if lr > 0.0 {Hash40::new("throw")} else {Hash40::new("throw")};
            ItemModule::set_have_item_constraint_joint(fighter.module_accessor,item_joint,0);
        }
        /* 
        if !LinkModule::is_link(fighter.module_accessor,*FIGHTER_LINK_NO_ITEM_TEMPORARY) {
            let link_created = LinkModule::link(fighter.module_accessor,*FIGHTER_LINK_NO_ITEM_TEMPORARY,fighter.battle_object_id);
            if link_created & 1 != 0 {
                println!("Linked");
                //LinkModule::set_model_constraint_pos_ort(owner_boma,*LINK_NO_CONSTRAINT,Hash40::new_raw(0x10489b2b69),Hash40::new("rot"),
                LinkModule::set_model_constraint_pos_ort(fighter.module_accessor,*FIGHTER_LINK_NO_ITEM_TEMPORARY,Hash40::new_raw(0xad79540c0),Hash40::new("haver"),
                (*CONSTRAINT_FLAG_ONE_NODE
                | *CONSTRAINT_FLAG_POSITION 
                | *CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_TRANSLATE
                ) as u32,false);
        
                let pos_offset = Vector3f{x:0.0,y:0.0,z:0.0};
                let rot_offset = Vector3f{x:0.0,y:90.0,z:0.0};
                LinkModule::set_constraint_translate_offset(fighter.module_accessor, &pos_offset);
                LinkModule::set_constraint_rot_offset(fighter.module_accessor, &rot_offset);
            }
        }*/
    }
}

pub unsafe extern "C" fn rash_frame_init(fighter: &mut L2CFighterCommon) {
    //rash_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    //agent.on_line(Main, rash_frame_init);
}
