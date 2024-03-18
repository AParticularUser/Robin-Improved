use crate::imports::*;
use crate::reflet::{
    consts::param,
    funcs::*
};

// moved durability consumption to motion
unsafe fn attack_4_start_status_main(agent: &mut L2CFighterCommon) {
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_ATTACK_FLAG_SUB_POINT);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
        thunder_sword_on(agent.module_accessor);
    }else {
        thunder_sword_off(agent.module_accessor);
    }
}
unsafe fn attack_4_start_status_main_loop(agent: &mut L2CFighterCommon) {
    //tap-jump short-hop-aerial macro check
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) <= 0 
    || WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_ATTACK_FLAG_SUB_POINT) {
        return
    }
    //update hud
    let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
    //sword recharge bonus
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
        let revival_param = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("thunder_sword_smash_attack_revival_time"));
        WorkModule::sub_int(agent.module_accessor, revival_param, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT);
    }
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_ATTACK_FLAG_SUB_POINT);
}
////status
//side
unsafe extern "C" fn attack_s4_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    attack_4_start_status_main(agent);               
    L2CFighterCommon::status_AttackS4Start_Common(agent);
    agent.sub_shift_status_main(L2CValue::Ptr(attack_s4_start_status_main_loop as *const () as _))
}
pub unsafe fn attack_s4_start_status_main_loop(agent: &mut L2CFighterCommon) {
    attack_4_start_status_main_loop(agent);
    agent.status_AttackS4Start_Main();
}
//hi
unsafe extern "C" fn attack_hi4_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    attack_4_start_status_main(agent);
    let hash:Hash40;
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        hash = Hash40::new("attack_hi4_2");
    }else {
        hash = Hash40::new("attack_hi4");
    }
    agent.status_AttackHi4Start_common(hash.into());
    agent.sub_shift_status_main(L2CValue::Ptr(attack_hi4_start_status_main_loop as *const () as _))
}
pub unsafe fn attack_hi4_start_status_main_loop(agent: &mut L2CFighterCommon) {
    attack_4_start_status_main_loop(agent);
    agent.status_AttackHi4Start_Main();
}
//lw
unsafe extern "C" fn attack_lw4_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    attack_4_start_status_main(agent);  
    agent.status_AttackLw4Start_common();
    agent.sub_shift_status_main(L2CValue::Ptr(attack_lw4_start_status_main_loop as *const () as _))
}
pub unsafe fn attack_lw4_start_status_main_loop(agent: &mut L2CFighterCommon) {
    attack_4_start_status_main_loop(agent);
    agent.status_AttackLw4Start_Main();
}
////motion
//game
//only uses duability after attack comes out
//no longer tosses sword durring endlag
// moved hit-box slightly forward on Forward-Smash to better match animation
unsafe extern "C" fn attack_s4_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 90, 0, 53, 4.0, 0.0, 10.0, 17.0, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 361, 80, 0, 60, 1.5, 0.0, 9.0/*8.0*/, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 9.0);
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
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.6, 361, 87, 0, 50, 3.0, 0.0, 10.0, 16.0/*14.0*/, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}
unsafe extern "C" fn attack_hi42_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 92, 86, 0, 55, 4.3, 0.0, 7.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 79, 0, 70, 3.5, 0.0, 9.0, 7.5, Some(0.0), Some(9.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::set_size(agent.module_accessor, 0, 5.8);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 70, 80, 0, 60, 2.5, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 55, 84, 0, 45, 4.2, 0.0, 5.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::set_size(agent.module_accessor, 0, 5.8);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}
unsafe extern "C" fn attack_lw4_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_SWORD_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 55, 95, 0, 50, 4.5, 0.0, 13.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 55, 95, 0, 50, 4.5, 0.0, 16.0, 12.0, Some(0.0), Some(2.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 14.5, Some(0.0), Some(3.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -6.5, Some(0.0), Some(3.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, Some(0.0), Some(3.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, Some(0.0), Some(3.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, Some(0.0), Some(3.5), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, Some(0.0), Some(3.5), Some(-6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 45, 100, 0, 50, 4.6, 0.0, 3.5, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.8);
        frame(agent.lua_state_agent, 35.0);
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                thunder_sword_off(agent.module_accessor);
            }
        }
    }else {
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 28, 95, 0, 50, 3.0, 0.0, 13.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 28, 95, 0, 50, 3.0, 0.0, 16.0, 10.0, Some(0.0), Some(2.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 29.0);
        macros::FT_MOTION_RATE(agent, 0.8);
    }
}
//effect
// adjusted effects to match hitboxes
unsafe extern "C" fn attack_s4_eff(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, -0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 6);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 6);
        }
    }
}
unsafe extern "C" fn attack_hi42_eff(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 10, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_attackhi4_spark"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 20, 2, -90, 0, 0, 0.4, true);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 20, 2, -90, 0, 0, 0.4, true);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 20, 2, -90, 0, 0, 0.4, true);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 3);
        }
    }
}
unsafe extern "C" fn attack_lw4_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 4, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("reflet_attacklw4_spark"), Hash40::new("top"), -0.0, 3, 4, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.8, 0.0, true, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_status_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, attack_hi4_status_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, attack_lw4_status_main);
    //game
    agent.game_acmd("game_attacks4", attack_s4_game);
    agent.game_acmd("game_attackhi42", attack_hi42_game);
    agent.game_acmd("game_attacklw4", attack_lw4_game);
    //effect
    agent.effect_acmd("effect_attacks4", attack_s4_eff);
    agent.effect_acmd("effect_attackhi42", attack_hi42_eff);
    agent.effect_acmd("effect_attacklw4", attack_lw4_eff);
}