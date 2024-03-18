use crate::imports::*;
use crate::common::consts::*;
use crate::reflet::{
    consts::{
        param, 
        vars::*
    },
    funcs::*
};


//status
// moved durability consumption to motion
unsafe extern "C" fn special_lw_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    //update hud
    agent.clear_lua_stack();
    let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    WorkModule::set_int(agent.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    //update book
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    }else {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    }
    //set vars
    WorkModule::set_int64(agent.module_accessor, hash40("special_lw_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_AIR as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    WorkModule::set_int(agent.module_accessor,  0x50000000,*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_OBJECT_ID);
    AttackModule::set_overlap_hit(agent.module_accessor, true);
    //motion and kinetic
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW);
        agent.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
        MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW);
        agent.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
        MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    agent.sub_shift_status_main(L2CValue::Ptr(special_lw_status_main_loop as *const () as _))
}
unsafe fn special_lw_status_main_loop(agent: &mut L2CFighterCommon) -> bool {
    agent.clear_lua_stack();
    lua_args!(agent, MA_MSC_CMD_GRAB_IS_GRAB, 0);
    sv_module_access::grab(agent.lua_state_agent);
    if agent.pop_lua_stack(1).get_bool() {
        if specializer::check_special_lw_pos(agent.module_accessor as *mut FighterModuleAccessor) == false {
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_GRAB_CLEAR, 0);
            sv_module_access::grab(agent.lua_state_agent);
            
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_GRAB_CLEAR, 1);
            sv_module_access::grab(agent.lua_state_agent);
        }
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_WORK_INT_CATCH_STATUS) 
    == *FIGHTER_REFLET_STATUS_SPECIAL_LW_CATCH_STATUS_CATCH_START {
        agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE.into(), false.into());
        return true.into()
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return true.into()
    }
    //air-ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_EFFECT_LANDING_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, false, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_module_access::effect(agent.lua_state_agent);
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
        }
    }
    false.into()
}
// recharges other spells and sword
unsafe extern "C" fn special_lw_capture_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
        WorkModule::set_int(agent.module_accessor, 15, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT);
    }else if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) 
    < WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("thunder_sword_usage_count_max")) {
        let sword_max = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("thunder_sword_usage_count_max"));
        WorkModule::set_int(agent.module_accessor, sword_max, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
    }
    let thunder_max = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("grimoire_thunder_usage_count_max"));
    WorkModule::set_int(agent.module_accessor, thunder_max, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
    let wind_max = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("grimoire_el_wind_usage_count_max"));
    WorkModule::set_int(agent.module_accessor, wind_max, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    let fire_max = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("grimoire_giga_fire_usage_count_max")) as f32;
    WorkModule::set_float(agent.module_accessor, fire_max, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
    VarModule::off_flag(agent.module_accessor, instance::REFLET_FLAG_SPECIAL_HI_2_ENABLE);

    let original = smashline::original_status(Main, agent, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE);
    original(agent)
}
//motion
// comes out faster and lasts longer
// only uses duability after attack comes out
unsafe extern "C" fn special_lw_game(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.33);//0.7
    wait(agent.lua_state_agent, 20.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) == false {
        macros::FT_MOTION_RATE(agent, 20.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_DARK_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            macros::CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 10.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 4.2, 0.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(11.2), *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_A);
        }
    }else {
        macros::FT_MOTION_RATE(agent, 2.0);
    }
}
// added poison effect 
unsafe extern "C" fn special_lw_game_capture(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 60, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    // while WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_WORK_INT_CATCH_STATUS) != *FIGHTER_REFLET_STATUS_SPECIAL_LW_CATCH_STATUS_NONE {
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 1, Hash40::new("throw"), 2.0, 60, 100, 100, 0, 8.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_poison_param(agent.module_accessor, 1, 361, 20, 0.5, false);
            macros::ATTACK_IGNORE_THROW(agent, 2, 1, Hash40::new("throw"), 2.0, 60, 100, 100, 0, 8.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_ATTACK_ID);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
        // wait(agent.lua_state_agent, 1.0);
    // }
}
// no longer enters special-fall if successful
unsafe extern "C" fn special_lw_end_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_LW_END_FLAG_HIT);
    let original = smashline::original_status(Main, agent, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END);
    original(agent)
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_status_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE, special_lw_capture_status_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END, special_lw_end_status_main);
    //motion
    agent.game_acmd("game_speciallwstart", special_lw_game);
    agent.game_acmd("game_specialairlwstart", special_lw_game);
    agent.game_acmd("game_speciallwcapture", special_lw_game_capture);
    agent.game_acmd("game_specialairlwcapture", special_lw_game_capture);
}