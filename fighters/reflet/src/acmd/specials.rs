
use super::*;

unsafe extern "C" fn reflet_special_n_tron_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn reflet_special_air_n_tron_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 15.0/(19.0-1.0));
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn reflet_special_n_tron_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 0.7); //FAF is frame 61
}

unsafe extern "C" fn reflet_special_air_n_tron_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 0.35); //FAF is frame 63
}

unsafe extern "C" fn reflet_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}

unsafe extern "C" fn reflet_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
            WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        }
        else{
            MotionModule::set_rate(boma, 2.0);
        }
    }
    wait(lua_state, 1.0);
    for _ in 0..30 {
        if is_excute(fighter) {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) {
                WorkModule::on_flag(boma,  *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
        wait(lua_state, 1.0);
    }
    
}

pub fn install() {
    smashline::Agent::new("reflet")
        .acmd("game_specialntronstart", reflet_special_n_tron_start_game)
        .acmd(
            "game_specialairntronstart",
            reflet_special_air_n_tron_start_game,
        )
        .acmd("game_specialntronend", reflet_special_n_tron_end_game)
        .acmd(
            "game_specialairntronend",
            reflet_special_air_n_tron_end_game,
        )
        .acmd("game_specialhi", reflet_special_hi_game)
        .acmd("game_specialairhi", reflet_special_air_hi_game)
        .install();
}
