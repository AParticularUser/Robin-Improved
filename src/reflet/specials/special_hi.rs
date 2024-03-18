use crate::imports::*;
use crate::common::{
    funcs::*,
    consts::*
};
use crate::reflet::consts::{
    vars::*, 
    param
};


//status
// handels enabling up-special-2
pub unsafe fn special_hi_2_enable_reset_check(agent: &mut L2CFighterCommon) {
    if VarModule::is_flag(agent.module_accessor, instance::REFLET_FLAG_SPECIAL_HI_2_ENABLE) {
        if (agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR 
        && StatusModule::status_kind(agent.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI)
        || is_damage_check(agent.module_accessor) {
            VarModule::off_flag(agent.module_accessor, instance::REFLET_FLAG_SPECIAL_HI_2_ENABLE);
        }
    }
}
// if up-special-1 was already used, change to up-special-2
unsafe extern "C" fn special_hi_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    //update hud
    agent.clear_lua_stack();
    let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
    FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
    WorkModule::set_int(agent.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);

    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL.into(), true.into());
        return true.into()
    }
    if VarModule::is_flag(agent.module_accessor, instance::REFLET_FLAG_SPECIAL_HI_2_ENABLE) {
        agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2.into(), true.into());
        return true.into()
    }
    VarModule::on_flag(agent.module_accessor, instance::REFLET_FLAG_SPECIAL_HI_2_ENABLE);
    false.into()
}
// removed special-fall from up-special-1
// moved durability consumption to motion
unsafe extern "C" fn special_hi_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(agent.module_accessor) == false {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
        }else {
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
        }
        let landing_frame = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
        WorkModule::set_float(agent.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        agent.sub_shift_status_main(L2CValue::Ptr(special_hi_status_main_loop as *const () as _));
    }
    0.into()
}
unsafe fn special_hi_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    && agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if CancelModule::is_enable_cancel(agent.module_accessor) {
            agent.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        return true.into()
    }
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND) 
    && (ControlModule::get_command_flag_cat(agent.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 
    {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) > 0 {
            agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
        }else {
            agent.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL.into(), false.into());
        }
        return true.into()
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        GroundModule::set_correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let jump = FighterSpecializer_Reflet::get_special_hi_jump_speed(agent.module_accessor as *mut smash::app::FighterModuleAccessor);
        //gravity
        let gravity = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticEnergy::reset_energy(gravity as *mut smash::app::KineticEnergy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: jump.y}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, agent.module_accessor);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        //stop
        let stop = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        KineticEnergy::unable(stop as *mut smash::app::KineticEnergy);
        //control
        let control = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticEnergy::reset_energy(control as *mut smash::app::KineticEnergy, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, agent.module_accessor);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump.x, 0.0);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let unk = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), 0x2c13759450);
        let mul = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_x_mul"), 0);
        let add = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_x_add"), 0);
        let limit = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
        FighterKineticEnergyController::set_accel_x_mul(control as *mut smash::app::FighterKineticEnergyController, mul*unk);
        FighterKineticEnergyController::set_accel_x_add(control as *mut smash::app::FighterKineticEnergyController, add*unk);
        KineticEnergyNormal::set_limit_speed(control as *mut smash::app::KineticEnergyNormal, &Vector2f{x: limit*unk, y: 0.0});
        //motion
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    false.into()
}
// moved durability consumption to motion
unsafe extern "C" fn special_hi_2_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_hi2"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi2"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_2_status_main_loop as *const () as _))
}
unsafe fn special_hi_2_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND 
    && agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        GroundModule::set_correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let jump = FighterSpecializer_Reflet::get_special_hi_jump_speed(agent.module_accessor as *mut smash::app::FighterModuleAccessor);
        //gravity
        let gravity = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticEnergy::reset_energy(gravity as *mut smash::app::KineticEnergy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: jump.y}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, agent.module_accessor);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        //stop
        let stop = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
        KineticEnergy::unable(stop as *mut smash::app::KineticEnergy);
        //control
        let control = KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticEnergy::reset_energy(control as *mut smash::app::KineticEnergy, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, agent.module_accessor);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, jump.x, 0.0);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let unk = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), 0x2c13759450);
        let mul = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_x_mul"), 0);
        let add = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_x_add"), 0);
        let limit = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
        FighterKineticEnergyController::set_accel_x_mul(control as *mut smash::app::FighterKineticEnergyController, mul*unk);
        FighterKineticEnergyController::set_accel_x_add(control as *mut smash::app::FighterKineticEnergyController, add*unk);
        KineticEnergyNormal::set_limit_speed(control as *mut smash::app::KineticEnergyNormal, &Vector2f{x: limit*unk, y: 0.0});
        //motion
        KineticModule::unable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    false.into()
}
//motion
// only uses duability after attack comes out
unsafe extern "C" fn special_hi_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_WIND_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
// only uses duability after attack comes out
unsafe extern "C" fn special_hi_2_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_WIND_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
// added missing "set_have_item_visibility" to be consistant with other specials
unsafe extern "C" fn special_hi_fail_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_blank"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_main);
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, special_hi_2_status_main);
    //game
    agent.game_acmd("game_specialhi", special_hi_game);
    agent.game_acmd("game_specialairhi", special_hi_game);
    agent.game_acmd("game_specialhi2", special_hi_2_game);
    agent.game_acmd("game_specialairhi2", special_hi_2_game);
    agent.expression_acmd("expression_specialhifail", special_hi_fail_exp);
    agent.expression_acmd("expression_specialairhifail", special_hi_fail_exp);
}