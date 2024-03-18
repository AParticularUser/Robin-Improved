use crate::imports::*;
use crate::reflet::funcs::*;


// use up current book and give as item, if holding item then throw away book
unsafe fn consume_book(agent: &mut L2CAgentBase) {
    let mut current_magic = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    let mut has_magic = false;
    //thunder
    if current_magic == *FIGHTER_REFLET_MAGIC_KIND_THUNDER
    || current_magic == *FIGHTER_REFLET_MAGIC_KIND_EL_THUNDER
    || current_magic == *FIGHTER_REFLET_MAGIC_KIND_GIGA_THUNDER
    || current_magic == *FIGHTER_REFLET_MAGIC_KIND_TRON {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) > 0 {
            WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
            WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
            //WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_THUNDER, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunder_max"), true, true);
            EffectModule::remove_common(agent.module_accessor, Hash40::new("charge_max"));
            current_magic = *FIGHTER_REFLET_MAGIC_KIND_THUNDER;
            has_magic = true;
        }
    }
    //fire
    if current_magic == *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE
    && WorkModule::get_float(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) > 0.0 {
        WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
        has_magic = true;
    }
    //wind
    if current_magic == *FIGHTER_REFLET_MAGIC_KIND_EL_WIND
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) > 0 {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        has_magic = true;
    }
    //dark
    if current_magic == *FIGHTER_REFLET_MAGIC_KIND_RIZAIA
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
        has_magic = true;
    }
    if has_magic {
        if ItemModule::get_have_item_kind(agent.module_accessor, 0) == *ITEM_KIND_NONE {
            ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BOOK), 0, 0, false, false);
            let item_id = ItemModule::get_have_item_id(agent.module_accessor, 0);
            let item_boma = sv_battle_object::module_accessor(item_id as u32);
            MotionModule::set_rate_material(item_boma, 0.0, MaterialAnimeKind{_address: 0});
            MotionModule::set_frame_material(item_boma, current_magic as f32, MaterialAnimeKind{_address: 0});
        }else {
            FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, current_magic, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
        macros::PLAY_SE(agent, Hash40::new("se_reflet_mp_empty"));
    }
}
//status
unsafe extern "C" fn appeal_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_APPEAL);
    let ret = original(agent);
    //update hud
    let motion = MotionModule::motion_kind(agent.module_accessor);
    if motion == smash::hash40("appeal_s_l")
    || motion == smash::hash40("appeal_s_r") {
        let magic_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
        let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
        FighterSpecializer_Reflet::change_hud_kind(fighter, magic_kind);
    }else if motion == smash::hash40("appeal_hi_l")
    || motion == smash::hash40("appeal_hi_r") {
        let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
        FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
    }
    ret
}
//motion
// holding side-taunt consumes current book
unsafe extern "C" fn appeal_s_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        || ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            consume_book(agent);
        }
    }
}
// swaps current sword
unsafe extern "C" fn appeal_hi_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            thunder_sword_off(agent.module_accessor);
        }else if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_revival"), Hash40::new("sword"),  0, 0, 0, 0, 0, 0, 0.4, false);
            thunder_sword_on(agent.module_accessor);
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_status_main);
    //game
    agent.game_acmd("game_appealsl", appeal_s_game);
    agent.game_acmd("game_appealsr", appeal_s_game);
    agent.game_acmd("game_appealhil", appeal_hi_game);
    agent.game_acmd("game_appealhir", appeal_hi_game);
}