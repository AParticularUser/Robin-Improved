use crate::imports::*;
use crate::reflet::consts::{
    vars::*, 
    param
};


pub unsafe fn thunder_sword_on(agent: *mut BattleObjectModuleAccessor) {
    if WorkModule::is_flag(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == false {
        VisibilityModule::set_int64(agent, hash40("sword") as i64, hash40("sword_thunder") as i64);
        WorkModule::on_flag(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
}
pub unsafe fn thunder_sword_off(agent: *mut BattleObjectModuleAccessor) {
    if WorkModule::is_flag(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        VisibilityModule::set_int64(agent, hash40("sword") as i64, hash40("sword_normal") as i64);
        WorkModule::off_flag(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
}
pub unsafe fn sword_passive_recharge(agent: *mut BattleObjectModuleAccessor) {
    let sword_max_charge = WorkModule::get_param_int(agent, hash40("param_private"), 0x1d517ad274);
    if WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) < sword_max_charge
    && WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
        if WorkModule::is_flag(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON)
        && [
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        ].contains(&StatusModule::status_kind(agent)) {
            VarModule::set_int(agent, instance::REFLET_INT_SWORD_PASSIVE_RECHARGE_COUNT, 0);
        }else {
            if VarModule::get_int(agent, instance::REFLET_INT_SWORD_PASSIVE_RECHARGE_COUNT) < param::REFLET_INT_SWORD_PASSIVE_RECHARGE_FRAME {
                VarModule::inc_int(agent, instance::REFLET_INT_SWORD_PASSIVE_RECHARGE_COUNT);
            }else {
                WorkModule::inc_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                VarModule::set_int(agent, instance::REFLET_INT_SWORD_PASSIVE_RECHARGE_COUNT, 0);
            }
        }
    }

}
pub unsafe fn thunder_passive_recharge(agent: *mut BattleObjectModuleAccessor) {
    let thunder_max_charge = WorkModule::get_param_int(agent, hash40("param_private"), 0x20682d7c6b);
    if WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) < thunder_max_charge
    && WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) > 0 {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END
        ].contains(&StatusModule::status_kind(agent)) {
            VarModule::set_int(agent, instance::REFLET_INT_THUNDER_PASSIVE_RECHARGE_COUNT, 0);
        }else {
            if VarModule::get_int(agent, instance::REFLET_INT_THUNDER_PASSIVE_RECHARGE_COUNT) < param::REFLET_INT_THUNDER_PASSIVE_RECHARGE_FRAME {
                VarModule::inc_int(agent, instance::REFLET_INT_THUNDER_PASSIVE_RECHARGE_COUNT);
            }else {
                WorkModule::inc_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
                VarModule::set_int(agent, instance::REFLET_INT_THUNDER_PASSIVE_RECHARGE_COUNT, 0);
            }
        }
    }
}
pub unsafe fn fire_passive_recharge(agent: *mut BattleObjectModuleAccessor) {
    let fire_max_charge = WorkModule::get_param_int(agent, hash40("param_private"), 0x2203952c8f);
    if WorkModule::get_float(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) < fire_max_charge as f32
    && WorkModule::get_float(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) > 0.0 {
        if *FIGHTER_STATUS_KIND_SPECIAL_S == StatusModule::status_kind(agent) 
        || MotionModule::motion_kind(agent) == hash40("attack_13") {
            VarModule::set_int(agent, instance::REFLET_INT_FIRE_PASSIVE_RECHARGE_COUNT, 0);
        }else {
            if VarModule::get_int(agent, instance::REFLET_INT_FIRE_PASSIVE_RECHARGE_COUNT) < param::REFLET_INT_FIRE_PASSIVE_RECHARGE_FRAME {
                VarModule::inc_int(agent, instance::REFLET_INT_FIRE_PASSIVE_RECHARGE_COUNT);
            }else {
                WorkModule::add_float(agent, 0.5, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
                VarModule::set_int(agent, instance::REFLET_INT_FIRE_PASSIVE_RECHARGE_COUNT, 0);
            }
        }
    }

}
pub unsafe fn wind_passive_recharge(agent: *mut BattleObjectModuleAccessor) {
    let wind_max_charge = WorkModule::get_param_int(agent, hash40("param_private"), 0x223d7cc6af);
    if WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) < wind_max_charge
    && WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) > 0 {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2,
            *FIGHTER_STATUS_KIND_ATTACK_100,
        ].contains(&StatusModule::status_kind(agent)) {
            VarModule::set_int(agent, instance::REFLET_INT_WIND_PASSIVE_RECHARGE_COUNT, 0);
        }else {
            if VarModule::get_int(agent, instance::REFLET_INT_WIND_PASSIVE_RECHARGE_COUNT) < param::REFLET_INT_WIND_PASSIVE_RECHARGE_FRAME {
                VarModule::inc_int(agent, instance::REFLET_INT_WIND_PASSIVE_RECHARGE_COUNT);
            }else {
                WorkModule::inc_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
                VarModule::set_int(agent, instance::REFLET_INT_WIND_PASSIVE_RECHARGE_COUNT, 0);
            }
        }
    }
}
pub unsafe fn dark_passive_recharge(agent: *mut BattleObjectModuleAccessor) {
    let dark_max_charge = WorkModule::get_param_int(agent, hash40("param_private"), 0x1f3fcc7e7d);
    if WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) < dark_max_charge
    && WorkModule::get_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE,
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END,
            *FIGHTER_STATUS_KIND_CATCH_ATTACK,
        ].contains(&StatusModule::status_kind(agent)) {
            VarModule::set_int(agent, instance::REFLET_INT_DARK_PASSIVE_RECHARGE_COUNT, 0);
        }else {
            if VarModule::get_int(agent, instance::REFLET_INT_DARK_PASSIVE_RECHARGE_COUNT) < param::REFLET_INT_DARK_PASSIVE_RECHARGE_FRAME {
                VarModule::inc_int(agent, instance::REFLET_INT_DARK_PASSIVE_RECHARGE_COUNT);
            }else {
                WorkModule::inc_int(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
                VarModule::set_int(agent, instance::REFLET_INT_DARK_PASSIVE_RECHARGE_COUNT, 0);
            }
        }
    }
}
pub mod specializer {
    extern "C" {
        #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet13get_magickindERNS_21FighterModuleAccessorEi"]
        pub fn get_magic_kind(
            arg1: *mut smash::app::FighterModuleAccessor,
            arg2: i32,
        ) -> i32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet20check_special_lw_posERNS_21FighterModuleAccessorE"]
        pub fn check_special_lw_pos(
            arg1: *mut smash::app::FighterModuleAccessor
        ) -> bool;
    }
    // extern "C" {
    //     #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet20get_active_thunder_numERNS_21FighterModuleAccessorEi"]
    //     pub fn get_active_thunder_num(
    //         arg1: *mut smash::app::FighterModuleAccessor,
    //         arg2: i32
    //     ) -> i32;
    // }
    // extern "C" {
    //     #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet15change_hud_kindERNS_7FighterEi"]
    //     pub fn change_hud_kind(
    //         arg1: *mut smash::app::Fighter,
    //         arg2: i32,
    //     ) -> u64;
    // }
    // extern "C" {
    //     #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet15change_grimoireERNS_21FighterModuleAccessorEi"]
    //     pub fn change_grimoire(
    //         arg1: *mut smash::app::BattleObjectModuleAccessor,
    //         arg2: i32,
    //     ) -> u64;
    // }
    // extern "C" {
    //     #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet15throwaway_swordERNS_7FighterERN3phx8Vector2fEb"]
    //     pub fn throwaway_sword(
    //         arg1: *mut smash::app::Fighter,
    //         arg2: smash::phx::Vector2f,
    //         arg3: bool,
    //     ) -> u64;
    // }
    // extern "C" {
    //     #[link_name = "\u{1}_ZN3app25FighterSpecializer_Reflet17set_flag_to_tableERNS_21FighterModuleAccessorEibi"]
    //     pub fn set_flag_to_table(
    //         arg1: *mut smash::app::BattleObjectModuleAccessor,
    //         arg2: i32,
    //         arg3: bool,
    //         arg2: i32,
    //     ) -> u64;
    // }
}