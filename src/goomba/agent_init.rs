use crate::imports::imports_agent::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app15sv_fighter_util21get_kirifuda_positionEP9lua_Statei"]
    pub fn get_kirifuda_position(lua_state: u64,arg2: i32) -> skyline::nn::util::Vector3f; 
}
pub unsafe extern "C" fn set_kirifuda_pos(fighter: &mut L2CFighterCommon) {
    let fuda = get_kirifuda_position(fighter.lua_state_agent, *STAGE_KIRIFUDA_KIND_IKE_1);
    let fuda_x = fuda.value[0];
    let fuda_y = fuda.value[1];
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    //println!("Init X: {fuda_x} Y: {fuda_y}");

    WorkModule::set_float(fighter.module_accessor, fuda_x, FIGHTER_GOOMBA_INSTANCE_FLOAT_KIRIFUDA_X);
    WorkModule::set_float(fighter.module_accessor, fuda_y, FIGHTER_GOOMBA_INSTANCE_FLOAT_KIRIFUDA_Y);
}

unsafe extern "C" fn reset_meshes(fighter: &mut L2CFighterCommon,status_next: i32) {
    let is_smash = [*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_next);
    if !is_smash {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
    }
    let status = StatusModule::status_kind(fighter.module_accessor);
    let special_lw_statuses = [*FIGHTER_STATUS_KIND_SPECIAL_LW,FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_POUND,
    FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_LANDING,FIGHTER_GOOMBA_STATUS_KIND_SPECIAL_LW_HIT];
    
    let was_special_lw = special_lw_statuses.contains(&status) && !special_lw_statuses.contains(&status_next);
    if was_special_lw {
        ArticleModule::remove_exist(fighter.module_accessor, FIGHTER_GOOMBA_GENERATE_ARTICLE_ACCESSORIES, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn restore_specials(fighter: &mut L2CFighterCommon,status_next: i32) {
    let is_reborn = [*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP].contains(&status_next);
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    let is_ground = (&[*SITUATION_KIND_GROUND,*SITUATION_KIND_CLIFF]).contains(&situation);

    let can_restore_specials = is_reborn || is_ground;
    if can_restore_specials {
        let log = sv_battle_object::log_attack_kind(fighter.battle_object_id);
        let is_log_s = (log as i32) == *FIGHTER_LOG_ATTACK_KIND_SPECIAL_S;
        let is_special_s = status_next == *FIGHTER_STATUS_KIND_SPECIAL_S || is_log_s;

        if !is_special_s {
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_GOOMBA_INSTANCE_FLAG_SPECIAL_S_DISABLE_HOP);
        }
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
	let status_exited = StatusModule::status_kind(fighter.module_accessor);
	let status_next = StatusModule::status_kind_next(fighter.module_accessor);

    reset_meshes(fighter,status_next);
    restore_specials(fighter,status_next);
    
    return true.into();
}

unsafe extern "C" fn should_use_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    return true.into();
}

unsafe extern "C" fn set_hurtbox(fighter: &mut L2CFighterCommon) {
    let disabled_joints = [
        hash40("mimir1"),hash40("mimil1"),hash40("armr"),hash40("arml"),hash40("tail1")
    ];
    for joint in disabled_joints {
        HitModule::set_status_joint_default(fighter.module_accessor, Hash40::new_raw(joint), HitStatus(*HIT_STATUS_OFF), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new_raw(joint), HitStatus(*HIT_STATUS_OFF), 0);
    }

    let disabled_hurtbox = [hash40("neck") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, *COLLISION_PART_ETC as f64,*HIT_HEIGHT_CENTER as f64,*HIT_STATUS_OFF as f64];
    let custom_hurtboxes = [
        //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
        [hash40("hip") as f64, 0.9, 0.0, 0.4, 0.9, 0.0, -0.4, 1.7, *COLLISION_PART_BODY as f64,*HIT_HEIGHT_CENTER as f64,*HIT_STATUS_NORMAL as f64],
        [hash40("head") as f64, 2.5, 0.7, 0.0, 2.5, -0.2, 0.0, 3.6, *COLLISION_PART_HEAD as f64,*HIT_HEIGHT_HIGH as f64,*HIT_STATUS_NORMAL as f64],
        disabled_hurtbox,
        disabled_hurtbox,
        disabled_hurtbox,
        disabled_hurtbox,
        [hash40("toer") as f64, 0.7, -1.0, 0.0, -1.0, -1.0, 0.0, 1.0, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64,*HIT_STATUS_NORMAL as f64],
        [hash40("toel") as f64, 0.7, -1.0, 0.0, -1.0, -1.0, 0.0, 1.0, *COLLISION_PART_BODY_LEGS as f64,*HIT_HEIGHT_LOW as f64,*HIT_STATUS_NORMAL as f64],
        disabled_hurtbox
    ];
    for i in 0..custom_hurtboxes.len() {
        let hurtbox = custom_hurtboxes[i];
        let mut vec1 = Vector3f{x: hurtbox[1] as f32, y: hurtbox[2] as f32, z: hurtbox[3] as f32};
        let mut vec2 = Vector3f{x: hurtbox[4] as f32, y: hurtbox[5] as f32, z: hurtbox[6] as f32};
        FighterUtil::set_hit_data(fighter.module_accessor,i as i32,0,&vec1,&vec2,hurtbox[7] as f32,Hash40::new_raw(hurtbox[0] as u64),
        smash::app::CollisionPart(hurtbox[8] as i32),smash::app::HitHeight(hurtbox[9] as i32),smash::app::HitStatus(hurtbox[10] as i32),
        smash::app::CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));
    }
}

unsafe extern "C" fn on_rebirth(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_GOOMBA_INSTANCE_FLAG_SUPERLEAF_VISIBLE);
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    on_rebirth(fighter);
    set_hurtbox(fighter);
    set_kirifuda_pos(fighter);

	fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
	fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(should_use_special_n as *const () as _));
}

unsafe extern "C" fn entry_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    on_rebirth(fighter);
    fighter.status_pre_Entry()
}
unsafe extern "C" fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    on_rebirth(fighter);
    fighter.status_pre_Rebirth()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
    agent.status(Pre, *FIGHTER_STATUS_KIND_ENTRY, entry_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_pre);
}