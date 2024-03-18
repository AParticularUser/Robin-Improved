use crate::imports::*;
use crate::common::consts::*;


// a+b smash always on for neutral-air
pub unsafe fn special_n_force_attack_air_n_check(agent: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(agent.module_accessor, 0) == false
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND 
        && ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            return false.into()
        }
        if agent.global_table[global_table::PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
        && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            return false.into()
        }
    }
    true.into()
}
//status
// moved article generation, durability consumption, and empty effect/sound to motion
unsafe extern "C" fn special_n_shoot_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let thunder_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
    //update book
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) > 0 {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_SHOOT_OK);
    }else {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_SHOOT_OK);
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, -1);
    }
    //set vars
    if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_THUNDER {
        WorkModule::set_int64(agent.module_accessor, hash40("special_n_shoot_thunder") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_n_shoot_thunder") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    }else {
        WorkModule::set_int64(agent.module_accessor, hash40("special_n_shoot") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_n_shoot") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    }
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_AIR as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);

    //motion and kinetic
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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

    agent.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_status_main_loop as *const () as _))
}
unsafe fn special_n_shoot_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
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
    false.into()
}
// moved durability consumption to motion 
unsafe extern "C" fn special_n_tron_start_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    //update book
    FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_TRON);
    //set vars
    WorkModule::set_int64(agent.module_accessor, hash40("special_n_tron_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(agent.module_accessor, hash40("special_air_n_tron_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    // WorkModule::set_int64(agent.module_accessor, *GROUND_CORRECT_KIND_AIR as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);

    //motion and kinetic
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
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
    agent.sub_shift_status_main(L2CValue::Ptr(special_n_tron_start_status_main_loop as *const () as _))
}
unsafe fn special_n_tron_start_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD.into(), false.into());
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
    false.into()

}
//motion
//shoot
// only uses duability after attack comes out
unsafe extern "C" fn special_n_shoot_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        // WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_SHOOT_OK) {
            let consume_point : i32;
            let thunder_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_THUNDER {
                consume_point = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("special_n_thunder_consume_point"));
            }else if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER {
                consume_point = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("special_n_el_thunder_consume_point"));
            }else {
                consume_point = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("special_n_giga_thunder_consume_point"));
            }
            WorkModule::sub_int(agent.module_accessor, consume_point, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_THUNDER, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_THUNDER, false, -1);
        }
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}
unsafe extern "C" fn special_n_shoot_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_flash"), Hash40::new("havel"), -1, 1, 0, 0, 0, 0, 0.45, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_SHOOT_OK) == false {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_book_smoke"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
}
unsafe extern "C" fn special_n_shoot_snd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_SHOOT_OK) {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            let thunder_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_THUNDER {
                macros::PLAY_SE(agent, Hash40::new("vc_reflet_special_n01"));
            }else if thunder_kind == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER {
                macros::PLAY_SE(agent, Hash40::new("vc_reflet_special_n02"));
            }else {
                macros::PLAY_SE(agent, Hash40::new("vc_reflet_special_n03"));
            }
        }
    }else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_mp_empty"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_reflet_rnd_special_empty"));
        }
    }
}
//hold
// only uses duability after attack comes out
unsafe extern "C" fn special_n_tron_start_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let consume_point = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("special_n_tron_consume_point"));
        WorkModule::sub_int(agent.module_accessor, consume_point, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_THUNDER, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_THUNDER, false, -1);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_status_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, special_n_tron_start_status_main);
    //motion
    //shoot
    agent.game_acmd("game_specialnshoot", special_n_shoot_game);
    agent.game_acmd("game_specialairnshoot", special_n_shoot_game);
    agent.effect_acmd("effect_specialnshoot", special_n_shoot_eff);
    agent.effect_acmd("effect_specialairnshoot", special_n_shoot_eff);
    agent.sound_acmd("sound_specialnshoot", special_n_shoot_snd);
    agent.sound_acmd("sound_specialairnshoot", special_n_shoot_snd);
    //hold
    agent.game_acmd("game_specialntronhold", special_n_tron_start_game);
    agent.game_acmd("game_specialairntronhold", special_n_tron_start_game);
}