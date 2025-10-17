use smash::{
    app::{self, lua_bind::*, *},
    hash40,
    lib::{lua_const::*, L2CAgent, L2CValue},
    lua2cpp::*,
    phx::*,
};

//use arcropolis_api;
use std::sync::RwLock;
use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    collections::HashMap,
    path::Path,
    fs,
    iter::FromIterator,
};

const DEFAULT_COLOR: (f32,f32,f32) = (2.8f32,0.5f32,0.1f32);
const DEFAULT_COLOR_ODD: (f32,f32,f32) = (0.5f32,2.8f32,0.1f32);
pub static mut EFFECT_COLORS: [(f32,f32,f32);256] = [DEFAULT_COLOR;256];

unsafe fn get_slot_from_module_accesor(boma: &mut BattleObjectModuleAccessor) -> usize {
    let entry_id = sv_battle_object::entry_id((*boma).battle_object_id) as u32;
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let slot = app::lua_bind::FighterInformation::fighter_color(info) as usize;
    return slot;
}
pub unsafe fn common_effect_color(agent: &mut L2CAgentBase) {
    use smash_script::{macros::*, *};

    //let r = 2.8; let g = 0.5; let b = 0.1;
    let slot = get_slot_from_module_accesor(&mut *agent.module_accessor);

    let mut effect_color = EFFECT_COLORS[slot];
    #[cfg(feature = "dev")] {
        if slot % 2 != 0 {
            effect_color = DEFAULT_COLOR_ODD;
        }
    }
    macros::LAST_PARTICLE_SET_COLOR(agent, effect_color.0,effect_color.1,effect_color.2);
    macros::LAST_EFFECT_SET_COLOR(agent, effect_color.0,effect_color.1,effect_color.2);
}

const DEFAULT_SLOTS: [usize;8] = [120,121,122,123,124,125,126,127];
lazy_static! {
    pub static ref MOD_SLOTS: RwLock<Vec<usize>> = RwLock::new({
        let mut default = Vec::with_capacity(256);
        for c in DEFAULT_SLOTS {
            default.push(c);
        }
        
        default
    });
}
pub unsafe fn get_kuribo_color(module_accessor: *mut BattleObjectModuleAccessor) -> i32 {
    let slot = get_slot_from_module_accesor(&mut *module_accessor);

    let mut slot_base: usize = 0;
    #[cfg(feature = "dev")]
    let slot_base = DEFAULT_SLOTS[0];
    #[cfg(not(feature = "dev"))]
    let slot_base = (*MOD_SLOTS.read().unwrap())[0];
    
    return (slot-slot_base) as i32;
}

pub unsafe fn is_kuribo(module_accessor: *mut BattleObjectModuleAccessor) -> bool
{
    let entry_id = sv_battle_object::entry_id((*module_accessor).battle_object_id) as u32;
    let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
    let color = app::lua_bind::FighterInformation::fighter_color(info) as usize;

    #[cfg(feature = "dev")]
    return DEFAULT_SLOTS.contains(&color);
    #[cfg(feature = "devhook")]
    return DEFAULT_SLOTS.contains(&color);

    let modded = (*MOD_SLOTS.read().unwrap()).contains(&color);
    return modded;
}

pub fn print_slots() {
    for slot in (&*MOD_SLOTS.read().unwrap()) {
        print!("{}, ",*slot);
    }
    println!("");
}

pub fn install() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    /*
    #[cfg(feature = "devhook")]{
        install_continue();
        return;
    } */
    #[cfg(not(feature = "dev"))]
    install_by_finding_markers();
}

#[cfg(not(feature = "dev"))]
fn install_by_finding_markers() {
    unsafe {
        let mut found_folder = false;
        (*MOD_SLOTS.write().unwrap()).clear();

        println!("[smashline_kuribo::ssm] Finding marker files...");
        const FIGHTER_NAME: &str = "pichu";
        const MARKER_FILE: &str = "kuribo.marker";
        let mut lowest_color: i32 = -1;
        let mut marked_slots: Vec<i32> = Vec::with_capacity(256);
        for x in 0..256 {
            if let Ok(marker_contents) = std::fs::read_to_string(format!(
                "mods:/fighter/{}/model/body/c{:02}/{}",
                FIGHTER_NAME, x, MARKER_FILE
            )) {
                unsafe {
                    (*MOD_SLOTS.write().unwrap()).push(x as _);
                    if lowest_color == -1 {
                        lowest_color = x as _ ;
                    }
                    if marker_contents.len() <= 0 {continue;}
                    
                    let mut eff_color = (DEFAULT_COLOR.0,DEFAULT_COLOR.1,DEFAULT_COLOR.2);
                    let (r, g) = marker_contents.split_once(char::is_whitespace).unwrap();
                    let (g, b) = g.trim_start().split_once(char::is_whitespace).unwrap();
                    let b = b.trim_start();
                    //let (b, rpg) = b.trim_start().split_once(char::is_whitespace).unwrap();

                    let mut color_changed=false;
                    let r32 = r.parse::<f32>();
                    let g32 = g.parse::<f32>();
                    let b32 = b.parse::<f32>();
                    if !r32.is_err() {
                        color_changed=true;
                        eff_color.0 = r32.unwrap();
                    }
                    if !g32.is_err() {
                        color_changed=true;
                        eff_color.1 = g32.unwrap();
                    }
                    if !b32.is_err() {
                        color_changed=true;
                        eff_color.2 = b32.unwrap();
                    }
                    if color_changed {
                        println!("[smashline_kuribo::ssm] Custom effect color for {x}: {}, {}, {}",eff_color.0,eff_color.1,eff_color.2);
                        EFFECT_COLORS[x] = eff_color;
                    }
                    /*
                    if rpg.len() > 1 {
                        if rpg.contains("goombella=T") {
                            println!("[smashline_kuribo::ssm] Alt {x} is Goombella");
                            (*RPG_SLOTS.write().unwrap()).push(x as _);
                        }
                    } 
                    */
                }
            }
        }

        if lowest_color == -1 {
            crate::SHOULD_INSTALL = false;
            println!("[smashline_kuribo::ssm] Goomba folder missing?");
            return;
        }
        install_continue();
    }
}

#[cfg(not(feature = "dev"))]
fn install_continue() {
    println!("[smashline_kuribo::ssm] Goomba slots: ");
    print_slots();
    params();
    csk();
    crate::install_after_mount();
}

#[cfg(not(feature = "dev"))]
fn params() {
    println!("[smashline_kuribo::singleslot]: Installing Goomba Params...");
    param_config::set_article_use_type(-(*WEAPON_KIND_PICHU_MONSTERBALL), *ARTICLE_USETYPE_FINAL);
    //return;

    let slot = (*MOD_SLOTS.read().unwrap()).to_vec();
    let mut slots: Vec<i32> = Vec::with_capacity(slot.len());
    for s in slot {
        slots.push(s as i32);
    }
    
    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    let mut param_floats: Vec<(u64,u64,f32)> = Vec::new();
    let mut param_attributes: Vec<(u64,u64,f32)> = Vec::new();

    param_config::disable_kirby_copy(*FIGHTER_KIND_PICHU, slots.clone());
    param_config::disable_villager_pocket(*smash::lib::lua_const::FIGHTER_KIND_PICHU, slots.clone(), *crate::vars::ACCESSORIES_KIND);
    param_config::disable_villager_pocket(*smash::lib::lua_const::FIGHTER_KIND_PICHU, slots.clone(), *crate::vars::REDSHELL_KIND);

    param_ints.push((hash40("s4_combo_max"),0 as u64, 2));

    param_attributes.push((hash40("walk_accel_mul"),0 as u64, 0.05 / 0.1575));
    param_floats.push((hash40("walk_accel_add"),0 as u64, 0.05 / 0.105));
    
    param_attributes.push((hash40("walk_speed_max"),0 as u64, 0.65 / 1.302));
    param_attributes.push((hash40("walk_slow_speed_mul"),0 as u64, 0.01 / 0.2));
    param_attributes.push((hash40("walk_middle_ratio"),0 as u64, 0.38 / 0.38));
    param_attributes.push((hash40("walk_fast_ratio"),0 as u64, 0.875 / 0.75));

    param_attributes.push((hash40("ground_brake"),0 as u64, 0.1 / 0.11));
    param_attributes.push((hash40("dash_speed"),0 as u64, 1.625 / 1.98));
    param_attributes.push((hash40("run_accel_mul"),0 as u64, 0.08 / 0.14)); //0.07-0.12
    param_attributes.push((hash40("run_accel_add"),0 as u64, 0.04 / 0.06)); //0.08-0.04
    param_attributes.push((hash40("run_speed_max"),0 as u64, 1.735 / 1.892));

    param_attributes.push((hash40("jump_speed_x"),0 as u64, 0.9 / 0.8)); 
    param_attributes.push((hash40("jump_speed_x_mul"),0 as u64, 0.8 / 0.8)); 
    param_attributes.push((hash40("jump_speed_x_max"),0 as u64, 1.3 / 1.8)); 
    param_attributes.push((hash40("jump_aerial_speed_x_mul"),0 as u64, 0.9 / 0.9)); 
    
    param_attributes.push((hash40("jump_initial_y"),0 as u64, 16.0 / 20.2125)); 
    param_attributes.push((hash40("jump_y"),0 as u64, 30.0 / 36.75)); 
    param_attributes.push((hash40("mini_jump_y"),0 as u64, 16.0 / 17.43)); 
    param_attributes.push((hash40("jump_aerial_y"),0 as u64, 28.0 / 36.02)); 

    param_attributes.push((hash40("air_accel_x_mul"),0 as u64, 0.08 / 0.09)); 
    //param_floats.push((hash40("air_accel_x_add"),0 as u64, 0.01 / 0.01));
    param_attributes.push((hash40("air_speed_x_stable"),0 as u64, 1.17 / 1.029)); 
    param_attributes.push((hash40("air_brake_x"),0 as u64, 0.008 / 0.0075)); 

    param_attributes.push((hash40("air_accel_y"),0 as u64, 0.095 / 0.14));
    param_attributes.push((hash40("air_speed_y_stable"),0 as u64, 1.75 / 1.9));
    param_attributes.push((hash40("air_brake_y"),0 as u64, 0.008 / 0.008));
    param_attributes.push((hash40("dive_speed_y"),0 as u64, 2.8 / 2.5));

    param_attributes.push((hash40("weight"),0 as u64, 81.0 / 62.0));

    //param_floats.push((hash40("landing_attack_air_frame_n"),0 as u64, 14.0));
    //param_floats.push((hash40("landing_attack_air_frame_f"),0 as u64, 10.0));
    //param_floats.push((hash40("landing_attack_air_frame_b"),0 as u64, 16.0));
    //param_floats.push((hash40("landing_attack_air_frame_hi"),0 as u64, 11.0));
    //param_floats.push((hash40("landing_attack_air_frame_lw"),0 as u64, 16.0));

    //param_floats.push((hash40("shield_radius"),0 as u64, 9.0));
    //param_floats.push((hash40("shield_break_y"),0 as u64, 47.0));

    //param_attributes.push((hash40("cliff_jump_x_speed"),0 as u64, 0.6 / 0.6));

    param_ints.push((hash40("wall_jump_type"),0 as u64, 0));
    param_ints.push((hash40("squat_walk_type"),0 as u64, 0));
    //param_attributes.push((hash40("height"),0 as u64, 8.5 / 10.0));
    //param_attributes.push((hash40("expand_height"),0 as u64, 12.5 / 14.0));
    param_attributes.push((hash40("passive_wall_jump_y_speed"),0 as u64, 2.3 / 2.6));

    param_floats.push((hash40("superleaftail_scale"),0 as u64, 1.0));
    //param_ints.push((hash40("size"),0 as u64, *FIGHTER_SIZE_S as i32));
    //param_floats.push((hash40("star_scale"),0 as u64, 2.4));
    //param_ints.push((hash40("star_attack_power"),0 as u64, 14));
    for p in param_ints {
        param_config::update_int_2(*FIGHTER_KIND_PICHU, slots.clone(), p);
    }
    for p in param_floats {
        param_config::update_float_2(*FIGHTER_KIND_PICHU, slots.clone(), p);
    }
    for p in param_attributes {
        param_config::update_attribute_mul_2(*FIGHTER_KIND_PICHU, slots.clone(), p);
    }
    println!("[smashline_kuribo::ssm]: Installed Params");
}

#[cfg(not(feature = "dev"))]
fn csk() {
    #[cfg(feature = "devhook")] {
        let has_csk = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libthe_csk_collection.nro").is_ok();
        if !has_csk {
            println!("[smashline_kuribo::ssm] No CSK?");
            //return;
        }
    }

    let chara_hash = smash::hash40("ui_chara_kuribo");
    csk_database(chara_hash);
    csk_css(chara_hash);
    csk_bgm(chara_hash);
}
#[cfg(not(feature = "dev"))]
fn csk_bgm(chara_hash: u64) {
    use smash::hash40;
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_z92_f_kuribo"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z92_f_packun")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_z92_f_kuribo")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_z92_f_kuribo"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_z92_f_kuribo")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_z92_f_kuribo"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_z92_f_kuribo")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });
    
    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_z92_f_kuribo"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("z92_f_kuribo")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("z92_f_kuribo"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7488,
        duration_sample: 359424 
    });
    
    the_csk_collection_api::set_fighter_jingle(chara_hash, "z92_f_kuribo");
    println!("[smashline_kuribo::ssm]: Replaced BGM");
}

#[cfg(not(feature = "dev"))]
fn csk_database(chara_hash: u64) {
    let slots = &*MOD_SLOTS.read().unwrap();
    let LOWEST_SLOT = *slots.iter().min().unwrap();
    let MAX_SLOT = *slots.iter().max().unwrap();
    let colors = (MAX_SLOT-LOWEST_SLOT+1) as u8;
    println!("[smashline_kuribo::ssm]: Creating CSK config {LOWEST_SLOT}-{MAX_SLOT}({colors})");
    //DATABASE ENTRY//
    let disp = 71; //Plant
    let skill_disp = 76; //Plant
    let save_no = 20; //Save no for Pichu
    let kind_hash = smash::hash40("fighter_kind_pichu");
    let narration = "vc_narration_characall_kuribo";
    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: chara_hash, 
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(kind_hash), 
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(kind_hash), 
            ui_series_id: the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40("ui_series_mario"))), 
            fighter_type: the_csk_collection_api::Hash40Type::Optional(Some(0x1353795179 /* Hash40 of fighter_type_normal */)), 
            alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */)), 
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("kuribo")), 
            exhibit_year: the_csk_collection_api::ShortType::Overwrite(1985), 
            exhibit_day_order: the_csk_collection_api::IntType::Overwrite(91301), 
            extra_flags: the_csk_collection_api::IntType::Optional(Some(0)), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Optional(Some(0)),
            skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(skill_disp)), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(disp)), 
            save_no: the_csk_collection_api::SignedByteType::Optional(Some(save_no)), 
            chara_count: the_csk_collection_api::SignedByteType::Optional(Some(1)), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Optional(Some(false)),
            is_img_ext_skill_page1: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_img_ext_skill_page2: the_csk_collection_api::BoolType::Optional(Some(false)), 
            can_select: the_csk_collection_api::BoolType::Optional(Some(true)), 
            is_usable_soundtest: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_called_pokemon: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_mii: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_hidden_boss: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_dlc: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_patch: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_plural_message: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_plural_narration: the_csk_collection_api::BoolType::Optional(Some(false)), 
            is_article: the_csk_collection_api::BoolType::Optional(Some(false)), 
            has_multiple_face: the_csk_collection_api::BoolType::Optional(Some(false)),
            result_pf0: the_csk_collection_api::BoolType::Optional(Some(true)), 
            result_pf1: the_csk_collection_api::BoolType::Optional(Some(true)), 
            result_pf2: the_csk_collection_api::BoolType::Optional(Some(true)), 
            color_num: the_csk_collection_api::UnsignedByteType::Optional(Some(colors)), 
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(hash40(narration)))), 
            
                (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Optional(Some(0x2302D482A /* Hash40 of -1 */))), 
                (0x160ab9eb98, the_csk_collection_api::Hash40Type::Optional(Some(smash::hash40("ui_chara_pichu"))) /* Hash40 of ui_chara_pichu */),
                            ])), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9F873561A /* Hash40 of c00_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x95E045DAE /* Hash40 of c01_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x96FEC4733 /* Hash40 of c02_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9C99B4C87 /* Hash40 of c03_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x90C3C7209 /* Hash40 of c04_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x9AA4B79BD /* Hash40 of c05_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x99BA36320 /* Hash40 of c06_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x93DD46894 /* Hash40 of c07_group */, the_csk_collection_api::UnsignedByteType::Optional(Some(0))), 
                (0x11895f00fc, the_csk_collection_api::UnsignedByteType::Overwrite(LOWEST_SLOT as u8)),
                            ])), 
            ..Default::default()
        }
    );
    //ONLINE,NARRATION//
    the_csk_collection_api::allow_ui_chara_hash_online(chara_hash);
    the_csk_collection_api::add_narration_characall_entry(narration);

    //TIPS//
    let skill_kinds = ["down_1","neutral_1","neutral_1","side_1","up_1","final_1"];
    let id_base = 2800-1 as u32;
    let id_specials = 2803;
    let id_normals = 2809;
    let id_last = 2813;

    for id in id_base..(id_last+1) {
        //let id = (base_id + i as u32) as u32;
        let is_misc = [2799,2802].contains(&id);
        let is_story = id < id_specials;
        let is_special = id >= id_specials && id < id_normals && !is_misc;
        let is_normal = id >= id_normals && !is_misc;

        let level = if is_normal {"level_middle"} else {"level_beginner"};
        let topic = if is_story {"topic_story"} else if is_misc {""} else {"topic_technic"};
        let skill_kind = if is_special {skill_kinds[(id-id_specials) as usize]} else {""};

        //println!("ID: {id} L: {} T: {} S: {}",level,topic,skill_kind);

        let order = id + 5050;
        the_csk_collection_api::add_tips_db_entry_info(
            &the_csk_collection_api::TipsDatabaseEntry {
                ui_tips_id: id as u64,
                clone_from_ui_tips_id: None,
                save_no: the_csk_collection_api::UnsignedIntType::Optional(Some(id)),
                level: the_csk_collection_api::Hash40Type::Overwrite(hash40(level)),
                topic: the_csk_collection_api::Hash40Type::Overwrite(hash40(topic)),
                skill_kind: the_csk_collection_api::Hash40Type::Overwrite(hash40(skill_kind)),
                ui_tips_unlock_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                disp_order: the_csk_collection_api::UnsignedIntType::Optional(Some(order)),
                type_0: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("relation_chara")),
                key_0: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_kuribo")), //chara_hash?
                type_1: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("relation_series")), //relation_series
                key_1: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_mario")), //ui_series_mario
                type_2: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_2: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_3: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_3: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_4: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_4: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_5: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_5: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_6: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_6: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_7: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_7: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                type_8: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
                key_8: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
            },
        );
    } 
}

#[cfg(not(feature = "dev"))]
fn csk_css(chara_hash: u64) {
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry{
            ui_layout_id: smash::hash40("ui_chara_kuribo_00"), 
            ui_chara_id: the_csk_collection_api::Hash40Type::Optional(Some(chara_hash)), 
            chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)), 
            eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
            eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-117.0)), 
            eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(-15.0)), 
            eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(180.0)), 
            eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(146.0)), 
            eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
            eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-108.0)), 
            eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(74.0)), 
            eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(110.0)), 
            eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(192.0)), 
            eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
            eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-50.0)), 
            eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(-12.0)), 
            eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(80.0)), 
            eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(59.0)), 
            eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(39.0)), 
            eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(9.0)), 
            spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)), 
            chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-16.0)), 
            chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-19.0)), 
            chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.84)), 
            chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-11.0)), 
            chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-5.0)), 
            chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.99)), 
            chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(-7.0)), 
            chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-7.0)), 
            chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
            chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(-8.0)), 
            chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-3.0)), 
            chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(0.77)), 
            chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(-31.0)), 
            chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(111.0)), 
            chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-45.0)), 
            chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(153.0)), 
            chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.74)), 
            chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
            chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.02)), 
            chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(50.0)), 
            chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(200.0)), 
            chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(-31.0)), 
            chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(111.0)), 
            chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(-105.0)), 
            chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(157.0)), 
            chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(0.8)), 
            chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(-31.0)), 
            chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(111.0)), 
            chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
            chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(3.0)), 
            chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(0.88)), 
            chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(5.0)), 
            chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.84)), 
            chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
            chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)),  
            ..Default::default()
        }
    );
}