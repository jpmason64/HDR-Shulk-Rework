#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![feature(repr_simd)]
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use smash::app::*;
use smash::app;
use smash::hash40;

use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

#[macro_use] extern crate smash_script;

pub mod djc;
pub mod opff;
pub mod misc;
pub mod general_statuses;
pub mod function_hooks;
pub mod shoto_status;
// pub mod tag;

extern "C" fn common_init(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::common::instance::LEDGE_ID, -1);
}

pub fn install() {
    djc::install();
    misc::install();
    // tag::install();
    general_statuses::install();
    function_hooks::install();
    opff::install();

    Agent::new("fighter")
        .on_start(common_init)
        .install();
}