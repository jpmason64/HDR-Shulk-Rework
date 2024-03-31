
use super::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67A20;
const SHULK_AERIAL_HIT : i32 = 0x200000ea;
const SHULK_GROUND_HIT : i32 = 0x200000eb;
const SHULK_THROW_HIT : i32 = 0x200000ec;
const SHULK_SMASHES_HIT : i32 = 0x200000ed;
const SHULK_TILTS_HIT : i32 = 0x200000ee;
const UNAVAILABLE_REDUCTION_FACTOR: f32 = 0.2;

unsafe extern "C" fn shulk_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.155);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("swordr"), 8.5, 68, 100, 0, 55, 3.6, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordr"), 7.5, 58, 100, 0, 45, 4.0, 7.75, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordr"), 7.5, 58, 100, 0, 45, 4.0, 13.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 54.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn shulk_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("swordr"), 8.0, 361, 104, 0, 46, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordr"), 8.0, 361, 104, 0, 46, 4.0, 5.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordr"), 6.5, 58, 70, 0, 70, 4.0, 12.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 65.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

unsafe extern "C" fn shulk_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 9.5, 361, 95, 0, 35, 2.75, -1.0, 0.7, 0.0, Some(-1.0), Some(0.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("bust"), 9.5, 361, 95, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 361, 100, 0, 35, 3.6, 0.0, 8.0, -5.5, Some(0.0), Some(8.0), Some(-0.25), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("bust"), 9.5, 361, 90, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 361, 100, 0, 35, 3.6, 0.0, 8.0, -7.5, Some(0.0), Some(8.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 361, 100, 0, 35, 3.6, 0.0, 9.5, -18.5, Some(0.0), Some(9.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("top"), 9.5, 361, 85, 0, 35, 2.6, 0.0, 9.5, -32.0, Some(0.0), Some(10.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 361, 70, 0, 40, 3.6, 0.0, 9.5, -17.0, Some(0.0), Some(9.5), Some(-9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 9.5, 361, 70, 0, 40, 2.6, 0.0, 9.5, -32.0, Some(0.0), Some(9.5), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 43.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

unsafe extern "C" fn shulk_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 3.5);
        FT_MOTION_RATE(fighter, 0.581);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 367, 90, 60, 0, 2.0, 0.0, 24.799999, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 367, 90, 60, 0, 5.0, 0.0, 17.5, 0.0, Some(0.0), Some(17.5), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 367, 90, 40, 0, 5.0, 0.0, 20.5, 0.0, Some(0.0), Some(17.299999), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.5, 90, 90, 60, 0, 2.0, 0.0, 24.799999, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 4.5, 90, 90, 60, 0, 5.0, 0.0, 17.5, 0.0, Some(0.0), Some(17.5), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 4.5, 90, 90, 45, 0, 5.0, 0.0, 20.5, 0.0, Some(0.0), Some(17.299999), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 88, 90, 0, 40, 3.5, 0.0, 20.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 88, 90, 0, 40, 2.0, 0.0, 39.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 88, 80, 0, 35, 3.5, 0.0, 20.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 88, 80, 0, 35, 2.0, 0.0, 39.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 78.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

unsafe extern "C" fn shulk_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 2.5);
        FT_MOTION_RATE(fighter, 0.846);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 367, 100, 45, 0, 5.3, 0.0, 4.0, 0.8, Some(0.0), Some(1.0), Some(0.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 160, 100, 20, 0, 5.3, 0.0, 4.0, 0.8, Some(0.0), Some(1.0), Some(0.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
        FT_MOTION_RATE(fighter, 0.667);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 3.2);
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 270, 90, 0, 15, 6.2, 0.0, -1.0, 0.8, Some(0.0), Some(-1.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        /* Ground-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 270, 85, 0, 50, 6.2, 0.0, 5.0, 0.8, Some(0.0), Some(5.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 270, 85, 0, 55, 3.7, 0.0, -14.0, 0.8, Some(0.0), Some(4.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        /* Air-only */
        ATTACK(fighter, 3, 0, Hash40::new("top"), 11.5, 270, 53, 0, 30, 6.2, 0.0, 5.0, 0.8, Some(0.0), Some(5.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 10.5, 270, 54, 0, 30, 3.7, 0.0, -14.0, 0.8, Some(0.0), Some(4.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 361, 75, 0, 50, 5.4, 0.0, -1.0, 0.8, Some(0.0), Some(-1.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 75, 0, 50, 5.4, 0.0, 5.0, 0.8, Some(0.0), Some(5.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 361, 75, 0, 55, 3.2, 0.0, -14.8, 0.8, Some(0.0), Some(4.0), Some(0.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
    }
    wait(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}


//Hit detection stuff
#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool, fighter: &mut L2CAgentBase) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    let defender_kind = sv_battle_object::kind(defender_id);
    let attacker = utils::util::get_battle_object_from_accessor(attacker_boma);
    // if search_hit flag is on
    if WorkModule::is_flag(attacker_boma, SHULK_AERIAL_HIT) {
        let jump_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        let speed_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        let shield_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        let buster_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        let smash_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        if(jump_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(jump_unavailable),*FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        }
        if(speed_unavailable != 0){
            WorkModule::set_int(attacker_boma,  get_post_collision_unavailable_frames(speed_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        }
        if(shield_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(shield_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        }
        if(buster_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(buster_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        }
        if(smash_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(smash_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        }
        // disable flag
        WorkModule::off_flag(attacker_boma, SHULK_AERIAL_HIT);
    }
    if WorkModule::is_flag(attacker_boma, SHULK_GROUND_HIT) {
        let jump_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        let speed_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        let shield_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        let buster_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        let smash_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        if(jump_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(jump_unavailable),*FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        }
        if(speed_unavailable != 0){
            WorkModule::set_int(attacker_boma,  get_post_collision_unavailable_frames(speed_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        }
        if(shield_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(shield_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        }
        if(buster_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(buster_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        }
        if(smash_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(smash_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        }
        // disable flag
        WorkModule::off_flag(attacker_boma, SHULK_GROUND_HIT);
    }
    if WorkModule::is_flag(attacker_boma, SHULK_SMASHES_HIT) {
        let jump_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        let speed_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        let shield_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        let buster_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        let smash_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        if(jump_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(jump_unavailable),*FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        }
        if(speed_unavailable != 0){
            WorkModule::set_int(attacker_boma,  get_post_collision_unavailable_frames(speed_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        }
        if(shield_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(shield_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        }
        if(buster_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(buster_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        }
        if(smash_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(smash_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        }
        // disable flag
        WorkModule::off_flag(attacker_boma, SHULK_SMASHES_HIT);
    }
    if WorkModule::is_flag(attacker_boma, SHULK_THROW_HIT) {
        let jump_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        let speed_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        let shield_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        let buster_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        let smash_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        if(jump_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(jump_unavailable),*FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        }
        if(speed_unavailable != 0){
            WorkModule::set_int(attacker_boma,  get_post_collision_unavailable_frames(speed_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        }
        if(shield_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(shield_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        }
        if(buster_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(buster_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        }
        if(smash_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(smash_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        }
        // disable flag
        WorkModule::off_flag(attacker_boma, SHULK_THROW_HIT);
    }
    if WorkModule::is_flag(attacker_boma, SHULK_TILTS_HIT) {
        let jump_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        let speed_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        let shield_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        let buster_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        let smash_unavailable = WorkModule::get_int(attacker_boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        if(jump_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(jump_unavailable),*FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_JUMP);
        }
        if(speed_unavailable != 0){
            WorkModule::set_int(attacker_boma,  get_post_collision_unavailable_frames(speed_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SPEED);
        }
        if(shield_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(shield_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SHIELD);
        }
        if(buster_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(buster_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_BUSTER);
        }
        if(smash_unavailable != 0){
            WorkModule::set_int(attacker_boma, get_post_collision_unavailable_frames(smash_unavailable), *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_UNAVAILABLE_FRAME_SMASH);
        }
        // disable flag
        WorkModule::off_flag(attacker_boma, SHULK_TILTS_HIT);
    }
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again, fighter)
}

//
fn get_post_collision_unavailable_frames(current_frames: i32) -> i32{
    let current_float = current_frames as f32;
    let res = current_float - current_float * UNAVAILABLE_REDUCTION_FACTOR;
    // println!("New Frame: {}", res as i32);
    return res as i32;
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];


pub fn install() {
    //Hit detection stuff
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    
    smashline::Agent::new("shulk")
        .acmd("game_attackairn", shulk_attack_air_n_game)
        .acmd("game_attackairf", shulk_attack_air_f_game)
        .acmd("game_attackairb", shulk_attack_air_b_game)
        .acmd("game_attackairhi", shulk_attack_air_hi_game)
        .acmd("game_attackairlw", shulk_attack_air_lw_game)
        .install();
    skyline::install_hook!(
        notify_log_event_collision_hit_replace
    );
}
