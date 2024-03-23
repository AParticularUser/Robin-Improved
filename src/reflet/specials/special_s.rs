use crate::imports::*;
use crate::common::consts::*;
use crate::reflet::consts::param;


//status
// moved article generation, durability consumption, and empty effect to motion
unsafe extern "C" fn special_s_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    //update hud
    agent.clear_lua_stack();
    let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
    WorkModule::set_int(agent.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);

    //update book
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) > 0.0 {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }else {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, -1);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }

    //set vars
    WorkModule::set_int64(agent.module_accessor, hash40("special_s") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(agent.module_accessor, hash40("special_air_s") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_AIR as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    // WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_FLAG_MAGIC_EMPTY_EFFECT_DONE);
    // WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);

    //motion and kinetic
    if agent.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        agent.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
        MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        agent.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
        MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    agent.sub_shift_status_main(L2CValue::Ptr(special_s_status_main_loop as *const () as _))
}
unsafe fn special_s_status_main_loop(agent: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(agent.module_accessor) {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    //air-ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
            agent.clear_lua_stack();
            lua_args!(agent, MA_MSC_CMD_EFFECT_LANDING_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, false, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_module_access::effect(agent.lua_state_agent);
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
        }
    }
    //shoot
    // if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) 
    // && WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY) {
    //     WorkModule::sub_float(agent.module_accessor, param::REFLET_FLOAT_FIRE_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
    //     if WorkModule::get_float(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) <= 0.0 {
    //         FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
    //     }
    //     ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_GIGAFIRE, false, -1);
    //     WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);
    // }else if MotionModule::frame(agent.module_accessor) > 12.0 
    // && WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_FLAG_MAGIC_EMPTY_EFFECT_DONE) == false {
    //     WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_FLAG_MAGIC_EMPTY_EFFECT_DONE);
    //     agent.clear_lua_stack();
    //     lua_args!(agent, MA_MSC_CMD_EFFECT_EFFECT_FOLLOW, hash40("reflet_book_smoke"), hash40("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, true);
    //     sv_module_access::effect(agent.lua_state_agent);
    // }
    false.into()
}
//motion
// only uses duability after attack comes out
unsafe extern "C" fn special_s_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
            WorkModule::sub_float(agent.module_accessor, param::REFLET_FLOAT_FIRE_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
            if WorkModule::get_float(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) <= 0.0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_GIGAFIRE, false, -1);
        }
        // WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) == false {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }

}
unsafe extern "C" fn special_s_eff(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hold"), Hash40::new("top"), -1, 22, 1.5, 0, 0, 0, 0.8, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hand"), Hash40::new("handr"), 1, 1, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hand"), Hash40::new("handl"), 1, 1, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }else {
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_book_smoke"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.7, true);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_status_main);
    //motion
    agent.game_acmd("game_specials", special_s_game);
    agent.game_acmd("game_specialairs", special_s_game);
    agent.effect_acmd("effect_specials", special_s_eff);
    agent.effect_acmd("effect_specialairs", special_s_eff);
}