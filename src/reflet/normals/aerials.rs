use crate::imports::*;
use crate::reflet::{
    consts::{
        param,
        vars::*
    },
    funcs::*
};


////status
// a+b smash always on for neutral-air
// moved durability consumption to motion
unsafe extern "C" fn attack_air_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        DamageModule::add_damage(agent.module_accessor, 0.1, 0);
    }
    agent.sub_attack_air();
    let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [
        smash::hash40("attack_air_n"),
        smash::hash40("attack_air_f"),
        smash::hash40("attack_air_b"),
        smash::hash40("attack_air_hi"),
        smash::hash40("attack_air_lw")
    ].contains(&motion) {
        //update hud
        agent.clear_lua_stack();
        let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
        FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
            //turn sword on/off
            if motion == smash::hash40("attack_air_hi")
            && WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_AIR_SMASH)
            && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) == false 
            && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SMASH) == false {
                let flick_level = FighterControlModuleImpl::get_param_attack_hi4_flick_y(agent.module_accessor) as i32;//0,1,2
                let flick_normal = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("attack_hi4_flick_y"));
                let flick_easy = WorkModule::get_param_int(agent.module_accessor, hash40("common"),  hash40("attack_hi4_flick_y_easy"));
                let flick_hard = WorkModule::get_param_int(agent.module_accessor, hash40("common"),  hash40("attack_hi4_flick_y_hard"));
                let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
                let flick_frame : i32;
                if flick_level == 0 {
                    flick_frame = flick_hard;
                }else if flick_level == 1 {
                    flick_frame = flick_normal;
                }else {
                    flick_frame = flick_easy;
                }
                if flick_frame <= flick_count {
                    WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_AIR_SMASH);
                }
            }else if motion == smash::hash40("attack_air_lw")
            && WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_AIR_SMASH) == false
            && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) == false {
                let flick_level = FighterControlModuleImpl::get_param_attack_lw4_flick_y(agent.module_accessor) as i32;//0,1,2
                let flick_normal = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("attack_lw4_flick_y"));
                let flick_easy = WorkModule::get_param_int(agent.module_accessor, hash40("common"),  hash40("attack_lw4_flick_y_easy"));
                let flick_hard = WorkModule::get_param_int(agent.module_accessor, hash40("common"),  hash40("attack_lw4_flick_y_hard"));
                let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
                let flick_frame : i32;
                if flick_level == 0 {
                    flick_frame = flick_hard;
                }else if flick_level == 1 {
                    flick_frame = flick_normal;
                }else {
                    flick_frame = flick_easy;
                }
                if flick_frame > flick_count {
                    WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_AIR_SMASH);
                }
            }
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_AIR_SMASH) {
                WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_REFLET_STATUS_ATTACK_AIR_INT_THUNDER_SWORD);
                thunder_sword_on(agent.module_accessor);
            }else {
                VarModule::set_int(agent.module_accessor, status::REFLET_INT_ATTACK_AIR_N_AB_SMASH_COUNT, 0);
                thunder_sword_off(agent.module_accessor);
            }
        }else {
            //sword recharge bonus
            let revival_param = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("thunder_sword_smash_attack_revival_time"));
            WorkModule::sub_int(agent.module_accessor, revival_param, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT);
        }
        // let attack_air_kind = ControlModule::get_attack_air_kind(agent.module_accessor);
        let mut additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01;
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            if motion == smash::hash40("attack_air_n") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08;
            }else 
            if motion == smash::hash40("attack_air_f") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10;
            }else 
            if motion == smash::hash40("attack_air_b") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_12;
            }else 
            if motion == smash::hash40("attack_air_hi") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_14;
            }else 
            if motion == smash::hash40("attack_air_lw") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_16;
            }
        }else {
            if motion == smash::hash40("attack_air_n") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07;
            }else 
            if motion == smash::hash40("attack_air_f") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_09;
            }else 
            if motion == smash::hash40("attack_air_b") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_11;
            }else 
            if motion == smash::hash40("attack_air_hi") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_13;
            }else 
            if motion == smash::hash40("attack_air_lw") {
                additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_15;
            }
        }
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, additions - 1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x3a40337e2c), additions - 1);
    }
    agent.sub_shift_status_main(L2CValue::Ptr(attack_air_status_main_loop as *const () as _))
}
pub unsafe fn attack_air_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion == smash::hash40("attack_air_n")
    && WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == false
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0
    && VarModule::get_int(agent.module_accessor, status::REFLET_INT_ATTACK_AIR_N_AB_SMASH_COUNT) <= param::REFLET_INT_ATTACK_AIR_N_AB_SMASH_FRAME {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
            thunder_sword_on(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08 - 1);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08 - 1);
        }else {
            VarModule::inc_int(agent.module_accessor, status::REFLET_INT_ATTACK_AIR_N_AB_SMASH_COUNT);
        }
    }
    agent.status_AttackAir_Main()
}
////motion
//game
// only uses duability after attack comes out
// no longer tosses sword durring endlag
unsafe extern "C" fn attack_air_n_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 6.0);//4
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT/2, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 8.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 11.5, 30, 64, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT/2, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 11.5, 30, 64, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 1, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 8.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 5.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 6.9, 30, 55, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 6.9, 30, 55, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_f_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);//4
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 12.5, 55, 100, 0, 50, 5.0, 0.0, 7.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 12.5, 55, 100, 0, 50, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.5, 55, 100, 0, 50, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 1, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 7.5, 361, 100, 0, 50, 2.5, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 7.5, 361, 100, 0, 50, 2.5, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 7.5, 361, 100, 0, 50, 2.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_b_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);//0
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 361, 93, 0, 40, 5.0, 0.0, 7.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 15.0, 361, 93, 0, 40, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 361, 93, 0, 40, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, -1.0, 8.0, -1.0, Some(-1.0), Some(2.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 1, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 361, 93, 0, 40, 2.5, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 361, 93, 0, 40, 2.5, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 361, 93, 0, 40, 2.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
// active frames lasts one frame longer to better match the animation
unsafe extern "C" fn attack_air_hi_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);//5
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 13.0, 95, 90, 0, 48, 5.0, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 13.0, 95, 90, 0, 48, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 95, 90, 0, 48, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 1, false);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 7.8, 95, 90, 0, 48, 3.0, 0.0, 6.0, -1.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 7.8, 95, 90, 0, 48, 3.0, 0.0, 2.0, -1.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 7.8, 95, 90, 0, 48, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 5.0);//4
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_lw_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);//2
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 70, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 12.0, 40, 80, 0, 55, 4.2, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 8.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 7.2, 361, 80, 0, 55, 3.7, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 6.0, 361, 80, 0, 45, 2.5, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
//effect
// adjusted effects to match hitboxes
unsafe extern "C" fn attack_air_n_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 5, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        }
    }
}
unsafe extern "C" fn attack_air_f_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 7, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
    }
}
unsafe extern "C" fn attack_air_b_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.2, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 1.3, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            // macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 0.5, true);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.3, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
}
unsafe extern "C" fn attack_air_hi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
}
unsafe extern "C" fn attack_air_lw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 1.8, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.3, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 2);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_status_main);
    //game
    agent.game_acmd("game_attackairn", attack_air_n_game);
    agent.game_acmd("game_attackairf", attack_air_f_game);
    agent.game_acmd("game_attackairb", attack_air_b_game);
    agent.game_acmd("game_attackairhi", attack_air_hi_game);
    agent.game_acmd("game_attackairlw", attack_air_lw_game);
    //effect
    agent.effect_acmd("effect_attackairn", attack_air_n_eff);
    agent.effect_acmd("effect_attackairf", attack_air_f_eff);
    agent.effect_acmd("effect_attackairb", attack_air_b_eff);
    agent.effect_acmd("effect_attackairhi", attack_air_hi_eff);
    agent.effect_acmd("effect_attackairlw", attack_air_lw_eff);
}