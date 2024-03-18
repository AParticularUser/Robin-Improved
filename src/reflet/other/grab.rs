use crate::imports::*;
use crate::reflet::consts::param;


// updates hud and book
pub unsafe fn catch_update_hud(agent: &mut L2CFighterCommon) {
    //update hud
    agent.clear_lua_stack();
    let fighter : *mut Fighter = std::mem::transmute(sv_system::battle_object(agent.lua_state_agent));
    FighterSpecializer_Reflet::change_hud_kind(fighter, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    WorkModule::set_int(agent.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    //update book
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    }else {
        FighterSpecializer_Reflet::change_grimoire(agent.module_accessor as *mut FighterModuleAccessor, -1);
    }
}
//status
unsafe extern "C" fn catch_pull_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    catch_update_hud(agent);
    let original = smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_CATCH_PULL);
    original(agent)
}
unsafe extern "C" fn catch_dash_pull_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    catch_update_hud(agent);
    let original = smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL);
    original(agent)
}
unsafe extern "C" fn catch_attack_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    catch_update_hud(agent);
    let original = smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_CATCH_ATTACK);
    original(agent)
}
//motion
// requires dark magic now
unsafe extern "C" fn catch_attack_game(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) > 0 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::sub_int(agent.module_accessor, param::REFLET_INT_DARK_CATCH_ATTACK_CONSUME_POINT, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.6/*1.5*/, 361, 100, 30, 0, 5.5, 0.0, 10.0, 12.0, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}
unsafe extern "C" fn catch_attack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let effect_handle = WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE);
        if EffectModule::is_exist_effect(agent.module_accessor, effect_handle as u32) == false {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_catch"), Hash40::new("top"), 0, 0, 5, 0, 0, 0, 1, false);
            agent.clear_lua_stack();
            lua_args!(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE);
            sv_animcmd::LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
            agent.clear_lua_stack();
            // macros::LAST_EFFECT_SET_WORK_INT(agent, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("reflet_catch_attack"), Hash40::new("top"), 0, 9, 13.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("reflet_book_smoke"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
}
unsafe extern "C" fn catch_attack_snd(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_reflet_mp_empty"));
        }
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_status_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, catch_dash_pull_status_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_status_main);
    //motion
    agent.game_acmd("game_catchattack", catch_attack_game);
    agent.effect_acmd("effect_catchattack", catch_attack_eff);
    agent.sound_acmd("sound_catchattack", catch_attack_snd);
}