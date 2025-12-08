use crate::imports::imports_agent::*;

const RETURN_0: u64 = 0;
const RETURN_1: u64 = 1;
const RETURN_ORIGINAL: u64 = 2;
const PICHU_VTABLE_ON_SEARCH_OFFSET : usize = 0x68d880+0x20;
const PICHU_VTABLE_ON_REFLECT_OFFSET : usize = 0x68d8d0+0x20;


#[skyline::hook(offset = PICHU_VTABLE_ON_SEARCH_OFFSET)]
unsafe extern "C" fn pichu_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let object = &mut fighter.battle_object;
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind as i32;
    if kind == *FIGHTER_KIND_PICHU
    && is_kuribo(module_accessor) 
    {
        let custom_result = on_search_goomba(fighter, log);
        if custom_result != RETURN_ORIGINAL {return custom_result};
    }
    original!()(vtable, fighter, log)
}

unsafe fn search_goomball_check(fighter: &mut Fighter, log: u64) -> bool {
    let collision_log = *(log as *const u64).add(0x10 / 0x8);
    let collision_log = collision_log as *mut CollisionLogScuffed;
    let collider_id = (*collision_log).opponent_object_id;
    if collider_id == *BATTLE_OBJECT_ID_INVALID as u32 {return false;}

    let collision_kind = (*collision_log).collision_kind;
    let collider_boma = &mut *(sv_battle_object::module_accessor(collider_id));
    let collider_category = smash::app::utility::get_category(collider_boma);
    if collider_category != *BATTLE_OBJECT_CATEGORY_WEAPON {return false;}

    let collider_kind = smash::app::utility::get_kind(collider_boma);
    let founder_id = WorkModule::get_int(collider_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let founder_boma = &mut *(sv_battle_object::module_accessor(founder_id));
    let founder_kind = smash::app::utility::get_kind(founder_boma);
    if collider_kind == REDSHELL_KIND && founder_kind == *FIGHTER_KIND_PICHU {
        return true;
    }
    return false;
}

unsafe fn on_search_goomba(fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S
    {
        if search_goomball_check(fighter,log) {
            WorkModule::on_flag(module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_REFLECT_GOOMBALL);
        }

        let motion_frame = MotionModule::frame(module_accessor);
        if motion_frame < (GOOMBALL_REFLECT_FRAME as f32) {
            MotionModule::set_frame_sync_anim_cmd(module_accessor, GOOMBALL_REFLECT_FRAME as f32, true, true, false);
        }

        return RETURN_1;
    }
    return RETURN_ORIGINAL;
}
#[skyline::hook(offset = PICHU_VTABLE_ON_REFLECT_OFFSET)]
unsafe extern "C" fn pichu_on_reflect(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let object = &mut fighter.battle_object;
    let module_accessor = fighter.battle_object.module_accessor;
    let kind = fighter.battle_object.kind as i32;
    if kind == *FIGHTER_KIND_PICHU
    && is_kuribo(module_accessor) 
    {
        let custom_result = on_reflect_goomba(fighter, log);
        if custom_result != RETURN_ORIGINAL {return custom_result};
    }
    original!()(vtable, fighter, log)
}

unsafe fn on_reflect_goomba(fighter: &mut Fighter, log: u64) -> u64 {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S
    {
        WorkModule::on_flag(module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_REFLECT_SFX);
        if WorkModule::is_flag(module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_REFLECT_GOOMBALL) {
            WorkModule::set_int(module_accessor, 1, FIGHTER_GOOMBA_SPECIAL_S_INT_GOOMBALL_COUNTER);
        }
    }
    return RETURN_ORIGINAL;
}

pub fn install() {
    skyline::install_hooks!(
        pichu_on_search,
        pichu_on_reflect,
    );
}