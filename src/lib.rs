#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    ambiguous_glob_reexports,
    unused_must_use
)]


pub mod imports {
    pub use {
        std::f32::consts::PI,
        smash::{
            lua2cpp::*,
            hash40,
            phx::{
                Hash40,
                Vector2f,
                Vector3f
            },
            app::{
                sv_animcmd::{
                    frame,
                    wait,
                    get_value_float
                },
                lua_bind::{
                    Article,
                    KineticEnergy,
                    KineticEnergyNormal,
                    FighterKineticEnergyController,
                    *
                },
                utility,
                *
            },
            lib::{
                lua_const::*,
                *
            }
        },
        smashline::*,
        smash_script::{
            macros::is_excute,
            macros::*,
            *
        },
        custom_var::*
    };
}

mod custom_var_hooks;
mod common;
mod reflet;

#[skyline::main(name = "robin_improved")]
pub fn main() {
    custom_var_hooks::install();
    reflet::install();
}