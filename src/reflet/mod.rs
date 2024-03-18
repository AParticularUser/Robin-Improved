use crate::imports::*;
use crate::common::consts::*;
use crate::reflet::{
    specials::{
        special_n::*,
        special_hi::*
    },
    funcs::*
};


unsafe extern "C" fn reflet_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[global_table::CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(special_n_force_attack_air_n_check as *const () as _));
}

unsafe extern "C" fn reflet_frame(agent: &mut L2CFighterCommon) {
    special_hi_2_enable_reset_check(agent);
    sword_passive_recharge(agent.module_accessor);
    thunder_passive_recharge(agent.module_accessor);
    wind_passive_recharge(agent.module_accessor);
    fire_passive_recharge(agent.module_accessor);
    dark_passive_recharge(agent.module_accessor);
}


pub mod consts;
pub mod funcs;
mod normals;
mod specials;
mod other;

pub fn install() {
    let agent = &mut smashline::Agent::new("reflet");
    agent.on_start(reflet_init);
    agent.on_line(Main, reflet_frame);
    normals::install(agent);
    specials::install(agent);
    other::install(agent);
    agent.install();
    // Always start a match with Levin Sword
    skyline::patching::Patch::in_text(0x1005d30).nop();
}