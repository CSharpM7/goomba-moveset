use crate::imports::imports_agent::*;

const RETURN_0: u64 = 0;
const RETURN_1: u64 = 1;
const RETURN_ORIGINAL: u64 = 2;
const PICHU_VTABLE_ON_SEARCH_OFFSET : usize = 0x68d880+0x20;
const PICHU_VTABLE_ON_REFLECT_OFFSET : usize = 0x68d8d0+0x20;

/*
All of this is just to make Goomball limtless, and to make side special
work a little bit more like Mario's. Nothing here is necessary
*/

#[skyline::from_offset(get_agent_virtual_function(*FIGHTER_KIND_PICHU,53,false,true))]
fn pichu_on_reflect(vtable: u64, fighter: &mut Fighter, log: u64);

#[skyline::from_offset(get_agent_virtual_function(*FIGHTER_KIND_PICHU,48,false,true))]
fn pichu_on_search(vtable: u64, fighter: &mut Fighter, log: u64);

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

unsafe fn on_search_goomba(vtable: u64,fighter: &mut Fighter, log: u64) -> usize {
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

        return 1;
    }
    get_original_fighter_vtable(*FIGHTER_KIND_PICHU,FighterVtableSlot::OnSearch)
    //pichu_on_search(vtable,fighter,log)
}


unsafe fn on_reflect_goomba(vtable: u64, fighter: &mut Fighter, log: u64) -> usize {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S
    {
        WorkModule::on_flag(module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_REFLECT_SFX);
        if WorkModule::is_flag(module_accessor, FIGHTER_GOOMBA_SPECIAL_S_FLAG_REFLECT_GOOMBALL) {
            WorkModule::set_int(module_accessor, 1, FIGHTER_GOOMBA_SPECIAL_S_INT_GOOMBALL_COUNTER);
        }
    }
    get_original_fighter_vtable(*FIGHTER_KIND_PICHU,FighterVtableSlot::Unk53)
    //pichu_on_reflect(vtable,fighter,log)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_PICHU,
        FighterVtableSlot::OnSearch,
        on_search_goomba as *const () as usize 
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_PICHU,
        FighterVtableSlot::Unk53,
        on_reflect_goomba as *const () as usize
    );
}