use crate::imports::imports_acmd::*;

const ANGLE: f32 = 65.0;
const SPEED: f32 = 3.2;

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let speed_x = ANGLE.cos()*-SPEED;
        let speed_y = ANGLE.sin()*-SPEED;

        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //0.9, -4. Speed = 4.1
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.5, 285, 90, 0, 20, 4.0, 0.0, 1.0, 0.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.5, 65, 75, 0, 67, 5.2, 0.0, 2.4, 0.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 65, 75, 0, 67, 4.2, 0.0, 2.4, 0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_DIVE_DECIDE);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_SWOOP);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        agent.set_cliff_hangdata(13.0,13.0,-7.6,7.0);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        let speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0,
            speed_y
        );
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
}

unsafe extern "C" fn game_attackairlw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_RESUME_CONTROL);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_speed_y_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_y_stable"), 0);
        let air_brake_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_brake_y"), 0);
        //println!("Speed: {speed_x} , {speed_y}");
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_wing_flying"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent,1.25);
        //air lw
        macros::EFFECT_FOLLOW(agent, Hash40::new("goomba_air_lw"), Hash40::new("top"), 0, -8, 6, -180.0+ANGLE, 0, 0, 1.1, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.7, true, 0.9);
    }    
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING) {
            EFFECT_OFF_KIND(agent,Hash40::new("goomba_air_lw"),false,false);
        }
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("goomba_air_lw"),false,false);
    }
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_wing_scatter"), Hash40::new("top"), 0, 5, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent,1.25);
    }
}

unsafe extern "C" fn effect_attackairlw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        EFFECT_OFF_KIND(agent,Hash40::new("goomba_air_lw"),false,false);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("goomba_wing_scatter"), Hash40::new("top"), 0, 5, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent,1.25);
    }
}

unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_special_s02_s"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_05"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pichu_rnd_attack"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pichu_attackair_l01"));
    }
}

unsafe extern "C" fn expression_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}
unsafe extern "C" fn expression_attackairlw2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_GOOMBA_ATTACK_AIR_FLAG_IS_DIVING) {
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("wing"), false);
        }
    }
}

unsafe extern "C" fn effect_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        //macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        
        macros::EFFECT(agent, Hash40::new("goomba_wing_scatter"), Hash40::new("top"), 0, 5, -5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_landingairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_pichu_attackair_l01"));
        //macros::PLAY_SE(agent, Hash40::new("se_pichu_attackair_l02"));
        macros::PLAY_SE(agent, Hash40::new("se_pichu_landing04"));
    }
}

unsafe extern "C" fn expression_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_attackairlw", game_attackairlw, Priority::Default);
	agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Default);
	agent.acmd("sound_attackairlw", sound_attackairlw, Priority::Default);
	agent.acmd("expression_attackairlw", expression_attackairlw, Priority::Default);

	agent.acmd("game_attackairlw2", game_attackairlw2, Priority::Default);
	agent.acmd("effect_attackairlw2", effect_attackairlw2, Priority::Default);
	agent.acmd("sound_attackairlw2", empty_acmd, Priority::Default);
	agent.acmd("expression_attackairlw2", expression_attackairlw2, Priority::Default);

	agent.acmd("game_landingairlw", empty_acmd, Priority::Default);
	agent.acmd("effect_landingairlw", effect_landingairlw, Priority::Default);
	agent.acmd("sound_landingairlw", sound_landingairlw, Priority::Default);
	agent.acmd("expression_landingairlw", expression_landingairlw, Priority::Default);

}