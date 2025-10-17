use crate::imports::imports_agent::*;

pub unsafe extern "C" fn tower_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, TOWER_INSTANCE_FLAG_DIE);
    WorkModule::off_flag(weapon.module_accessor, TOWER_INSTANCE_FLAG_SLIP);
    WorkModule::set_int(weapon.module_accessor, 0,TOWER_INSTANCE_INT_STEP);

    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("wing"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("helmet"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("superleaf_ear"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("superleaf_tail"), false);

    let owner = &mut *sv_battle_object::module_accessor(
        (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32
    );
    let owner_scale = ModelModule::scale(owner);
    ModelModule::set_scale(weapon.module_accessor, owner_scale);
    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_VISIBILITY as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SCALE as u8}, true);
        LinkModule::set_attribute(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_MODEL_SCALE as u8}, true);
    }

    let num_goombas = ArticleModule::get_num(owner, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);
    let is_first = num_goombas==1;
    println!("Current goombas: {num_goombas}");

    let anim = if is_first {Hash40::new("goomba1_win2")} else {Hash40::new("goomba2_win2")};
    let frame = if is_first {0.0} else {0.0};
    MotionModule::change_motion(weapon.module_accessor, anim, frame, 1.0, false, 0.0, false, false);
    
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("hat"), !is_first);
    weapon.fastshift(L2CValue::Ptr(tower_main_loop as *const () as _))
}

unsafe extern "C" fn tower_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32;
    if owner_id == *BATTLE_OBJECT_ID_INVALID as u32 {
        return 0.into();
    }

    let owner = &mut *sv_battle_object::module_accessor(owner_id);
    let step = WorkModule::get_int(weapon.module_accessor, TOWER_INSTANCE_INT_STEP);
    if step == TOWER_STEP_LAND {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("head"), &mut pos,false); 
        let eff = EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_down_smoke"),
            &Vector3f{x:pos.x,y:0.0,z:pos.z},
            &VECTOR_ZERO,
            0.8,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(weapon.module_accessor, eff, -1);

        SoundModule::play_se_no3d(owner, Hash40::new("se_pichu_special_s01"), false, false);
        //SoundModule::play_se(owner, Hash40::new("se_pichu_special_n01_win03"), true, false, false, false, enSEType(0));
    }
    else if step == TOWER_STEP_CLOUD {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("head"), &mut pos,false); 
        let eff = EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_erace_smoke"),
            &Vector3f{x:pos.x,y:0.0,z:pos.z},
            &VECTOR_ZERO,
            1.5,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(weapon.module_accessor, eff, -1);

        SoundModule::play_se_no3d(owner, Hash40::new("se_pichu_special_n01_win03"), false, false);
        //SoundModule::play_se(owner, Hash40::new("se_pichu_special_n01_win03"), true, false, false, false, enSEType(0));
    }

    else if step == TOWER_STEP_COIN {
        let mut pos = VECTOR_ZERO;
        let joint_offset = ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("head"), &mut pos,false); 
        let eff = EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_s_jump"),
            &Vector3f{x:pos.x,y:-0.5,z:pos.z},
            &VECTOR_ZERO,
            0.375,
            0, -1, false, 0
        ) as u32;
        EffectModule::detach(weapon.module_accessor, eff, -1);

        SoundModule::play_se_no3d(owner, Hash40::new("se_common_coin"), false, false);
        //SoundModule::play_se(owner, Hash40::new("se_common_coin"), true, false, false, false, enSEType(0));

        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    
    if MotionModule::is_end(weapon.module_accessor) || 
    WorkModule::is_flag(weapon.module_accessor, TOWER_INSTANCE_FLAG_DIE) {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub unsafe extern "C" fn tower_frame(fighter: &mut L2CFighterCommon) {
    println!("!");
}

pub fn install(agent: &mut smashline::Agent) {
	agent.status(Main, 0, tower_main);
	//agent.on_start(tower_main);
    //agent.on_line(Main, tower_frame);
}