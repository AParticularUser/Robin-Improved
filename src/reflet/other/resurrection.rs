use crate::imports::*;
use crate::common::consts::*;
use crate::reflet::funcs::*;


//status
//moved effect to motion
unsafe extern "C" fn resurrection_book_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let magic_kind = specializer::get_magic_kind(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_RECOVER_TABLE);
    WorkModule::set_int(agent.module_accessor, magic_kind, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_TEMP_MAGIC_KIND);
    WorkModule::set_int(agent.module_accessor, magic_kind, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, magic_kind);
    FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, magic_kind, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_RECOVER_TABLE);
    MotionModule::change_motion(agent.module_accessor, Hash40::new("resurrection_book"), 0.0, 1.0, false, 0.0, false, false);
    agent.sub_shift_status_main(L2CValue::Ptr(resurrection_book_status_main_loop as *const () as _))
}
unsafe fn resurrection_book_status_main_loop(agent: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    false.into()
}
//motion
//added different effect for each book
unsafe extern "C" fn resurrection_book_eff(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let magic_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_TEMP_MAGIC_KIND);
        if magic_kind == *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hold"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.4, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }else if magic_kind == *FIGHTER_REFLET_MAGIC_KIND_EL_WIND {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_magic_wind"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.35, true);
        }else if magic_kind == *FIGHTER_REFLET_MAGIC_KIND_RIZAIA {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_rizaia"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.4, true);
        }
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_revival_th"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.4, true);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        let magic_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_TEMP_MAGIC_KIND);
        if magic_kind == *FIGHTER_REFLET_MAGIC_KIND_EL_WIND {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_magic_wind"), false, false);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_BOOK, resurrection_book_status_main);
    //motion
    agent.effect_acmd("effect_resurrectionbook", resurrection_book_eff);
}