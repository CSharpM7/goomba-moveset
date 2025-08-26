use crate::imports::imports_acmd::*;

unsafe extern "C" fn disable_for_testing(color: i32) -> bool {
    return false;
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
	let status_exited = StatusModule::status_kind(fighter.module_accessor);
	let status_next = StatusModule::status_kind_next(fighter.module_accessor);
    //println!("{status_exited} > {status_next}");
    let situation = StatusModule::situation_kind(fighter.module_accessor);

    let is_reborn = [*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP].contains(&status_next);
    let is_hit = is_damage_status_next(fighter.module_accessor);
    let is_ground = (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&situation);

    let can_restore_specials = is_reborn || is_hit || is_ground;
    if can_restore_specials {
        let log = sv_battle_object::log_attack_kind(fighter.battle_object_id);
        let is_log_n = (log as i32) == *FIGHTER_LOG_ATTACK_KIND_SPECIAL_N; 
        let is_special_n = status_next == *FIGHTER_STATUS_KIND_SPECIAL_N || is_log_n;

        let is_log_s = (log as i32) == *FIGHTER_LOG_ATTACK_KIND_SPECIAL_S;
        let is_special_s = status_next == *FIGHTER_STATUS_KIND_SPECIAL_S || is_log_s;

        let is_log_hi = (log as i32) == *FIGHTER_LOG_ATTACK_KIND_SPECIAL_HI;
        let is_special_hi = status_next == *FIGHTER_STATUS_KIND_SPECIAL_HI || is_log_hi;

        if !is_special_n {
            //WorkModule::off_flag(fighter.module_accessor,FIGHTER_RASH_INSTANCE_DISABLE_SPECIAL_N);
        }
        if !is_special_s {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_GOOMBA_INSTANCE_FLAG_SPECIAL_S_DISABLE_HOP);
        }
        if !is_special_hi {
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
        }
        
    }
    return true.into();
}

unsafe extern "C" fn should_use_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let disable_flag = false;//WorkModule::is_flag(fighter.module_accessor, FIGHTER_RASH_INSTANCE_DISABLE_SPECIAL_N);

    let boma = fighter.module_accessor;
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    && disable_flag) {
        false.into()
    } else {
        true.into()
    }
}
unsafe extern "C" fn should_use_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let disable_flag =false; //WorkModule::is_flag(fighter.module_accessor, FIGHTER_RASH_INSTANCE_DISABLE_SPECIAL_S);

    let boma = fighter.module_accessor;
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    && disable_flag) 
    || disable_for_testing(color) 
    {
        false.into()
    } else {
        true.into()
    }
}
unsafe extern "C" fn should_use_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let disable_flag = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);

    let boma = fighter.module_accessor;
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    && disable_flag) 
    || disable_for_testing(color) 
    {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn set_hurtbox(fighter: &mut L2CFighterCommon) {
    let custom_hurtboxes = [
        //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
        [hash40("hip") as f64, 0.9, 0.0, 0.4, 0.9, 0.0, -0.4, 2.7, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
        //[hash40("bust") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.6, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64],
        [hash40("head") as f64, 2.5, 0.7, 0.0, 2.5, -0.2, 0.0, 3.6, *COLLISION_PART_HEAD as f64,*HIT_HEIGHT_HIGH as f64],
        [hash40("toer") as f64, 0.9, -0.3, 0.0, -1.0, -0.3, 0.0, 0.7, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64],
        [hash40("toel") as f64, 0.9, -0.3, 0.0, -1.0, -0.3, 0.0, 0.7, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64]
    ];
    for i in 0..custom_hurtboxes.len() {
        let hurtbox = custom_hurtboxes[i];
        let mut vec1 = Vector3f{x: hurtbox[1] as f32, y: hurtbox[2] as f32, z: hurtbox[3] as f32};
        let mut vec2 = Vector3f{x: hurtbox[4] as f32, y: hurtbox[5] as f32, z: hurtbox[6] as f32};
        FighterUtil::set_hit_data(fighter.module_accessor,i as i32,0,&vec1,&vec2,hurtbox[7] as f32,Hash40::new_raw(hurtbox[0] as u64),smash::app::CollisionPart(hurtbox[8] as i32),smash::app::HitHeight(hurtbox[9] as i32),smash::app::HitStatus(*HIT_STATUS_NORMAL),smash::app::CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
    }
}

unsafe extern "C" fn on_rebirth(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
}
unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_hurtbox(fighter);
    on_rebirth(fighter);

	fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
	fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(should_use_special_n as *const () as _));
	fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s as *const () as _));
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi as *const () as _));   
    
    //let status_func: &mut skyline::libc::c_void = std::mem::transmute(fighter.sv_get_status_func(&L2CValue::I32(*FIGHTER_STATUS_KIND_ENTRY),&L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)).get_ptr());
    //fighter.sv_set_status_func(L2CValue::I32(*FIGHTER_STATUS_KIND_APPEAL),L2CValue::I32(*LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN),status_func);
}
unsafe extern "C" fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    on_rebirth(fighter);
    fighter.status_pre_Rebirth()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
    agent.status(Pre, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_pre);
}
