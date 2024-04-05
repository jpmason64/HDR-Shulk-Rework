
use super::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67A20;
const SHULK_TILTS_HIT : i32 = 0x200000ee;
const UNAVAILABLE_REDUCTION_FACTOR: f32 = 0.2;

//Sword bone might be scaled to 0, do check
unsafe extern "C" fn shulk_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    println!("High Ftilt");
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(11.0-1.0));
        WorkModule::on_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter){
        PLAY_SE(fighter, Hash40::new("se_shulk_monado_open"));
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -85.0, 0.0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter){
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_vision_attack"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -70.0, 0.0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_shulk_sword3"), Hash40::new("tex_shulk_sword4"), 5.0 as u64, Hash40::new("haver"), 0.0, 3.0, 0.9, Hash40::new("haver"), 0.0, 19.3, 0.9, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101.0 as u64, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2_lightning"), Hash40::new("haver"), 0, 2.4, 0, 0, 0, 0, 0.7, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);

    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_vision_attack"), true, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(16.2-12.0));
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), false);
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), true);
        PLAY_SE(fighter, Hash40::new("se_shulk_attackhard_s01"));
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.0, 361, 97, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 97, 0, 35, 2.75, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordr"), 15.0, 361, 97, 0, 35, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordr"), 13.0, 361, 97, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordr"), 11.0, 361, 97, 0, 35, 4.0, 11.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter){
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 16.2);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(17.0-16.2));
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
        PLAY_SEQUENCE(fighter, Hash40::new("seq_shulk_rnd_attack"));
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_circle"), false, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), true);
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), false);

}

unsafe extern "C" fn shulk_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    println!("Side Ftilt");
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(11.0-1.0));
        WorkModule::on_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(16.2-12.0));
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), false);
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), true);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.0, 361, 97, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 97, 0, 35, 2.75, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordr"), 15.0, 361, 97, 0, 35, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordr"), 13.0, 361, 97, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordr"), 11.0, 361, 97, 0, 35, 4.0, 11.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 16.2);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(17.0-16.2));
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 22.0/(42.0-17.0));
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), true);
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), false);
}

unsafe extern "C" fn shulk_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    println!("Low Ftilt");
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 8.0/(11.0-1.0));
        WorkModule::on_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter){
        PLAY_SE(fighter, Hash40::new("se_shulk_monado_open"));
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -85.0, 0.0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter){
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_vision_attack"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -70.0, 0.0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_shulk_sword3"), Hash40::new("tex_shulk_sword4"), 5.0 as u64, Hash40::new("haver"), 0.0, 3.0, 0.9, Hash40::new("haver"), 0.0, 19.3, 0.9, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101.0 as u64, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2_lightning"), Hash40::new("haver"), 0, 2.4, 0, 0, 0, 0, 0.7, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);

    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_vision_attack"), true, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        //bleh
        FT_MOTION_RATE(fighter, 3.0/(16.2-12.0));
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), false);
        // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), true);
        PLAY_SE(fighter, Hash40::new("se_shulk_attackhard_s01"));
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.0, 361, 97, 0, 35, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 13.0, 361, 97, 0, 35, 2.75, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordr"), 15.0, 361, 97, 0, 35, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordr"), 13.0, 361, 97, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordr"), 11.0, 361, 97, 0, 35, 4.0, 11.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter){
        AFTER_IMAGE_OFF(fighter, 1);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter){
        EFFECT_FOLLOW(fighter, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 16.2);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(17.0-16.2));
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
        PLAY_SEQUENCE(fighter, Hash40::new("seq_shulk_rnd_attack"));
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("shulk_monad_circle"), false, false);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 22.0/(42.0-17.0));
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), true);
    // ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), false);

}

unsafe extern "C" fn shulk_attack_s3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

//Sound script from script dump
unsafe extern "C" fn shulk_sound_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_monado_open"));
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_attackhard_s01"));
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_shulk_rnd_attack"));
    }
}

//Effect script from script dump
unsafe extern "C" fn shulk_effect_attacks3(agent: &mut L2CAgentBase) {
    println!("Queres?");
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle"), Hash40::new("swordr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -85.0, 0.0, 0.5, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_vision_attack"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_vision_attack"), Hash40::new("haver"), 0.5, 3.0, 1.0, 0.0, -70.0, 0.0, 0.5, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_shulk_sword3"), Hash40::new("tex_shulk_sword4"), 5.0 as u64, Hash40::new("haver"), 0.0, 3.0, 0.9, Hash40::new("haver"), 0.0, 19.3, 0.9, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101.0 as u64, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_lightning"), Hash40::new("haver"), 0, 2.4, 0, 0, 0, 0, 0.7, true);
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_vision_attack"), true, true);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword2_end"), Hash40::new("haver"), 0.0, 2.4, 0.0, 0.0, 0.0, 0.0, 1.0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword2"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle"), false, false);
    }
}

unsafe extern "C" fn shulk_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.533);
        WorkModule::on_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.0, 85, 80, 0, 70, 3.8, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 10.0, 85, 80, 0, 70, 3.8, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 10.0, 85, 80, 0, 70, 3.8, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 10.0, 85, 80, 0, 70, 3.8, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.0, 85, 80, 0, 70, 3.8, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 107, 75, 0, 65, 5.0, 0.0, 14.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 107, 75, 0, 65, 5.0, 0.0, 10.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
        FT_MOTION_RATE(fighter, 0.800);
    }
    
}

//This is Shulk's down tilt 
unsafe extern "C" fn shulk_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, SHULK_TILTS_HIT);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.5, 70, 90, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.5, 70, 90, 0, 65, 3.0, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.5, 70, 90, 0, 65, 3.5, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.5, 80, 90, 0, 65, 3.5, 0.0, 9.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("haver"), 7.5, 80, 90, 0, 65, 3.0, 0.0, 14.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    }
    
}

pub fn install() {

    smashline::Agent::new("shulk")
        .game_acmd("game_attacks3hi",shulk_attack_s3_hi_game)
        .game_acmd("game_attacks3", shulk_attack_s3_s_game)
        .game_acmd("game_attacks3lw", shulk_attack_s3_lw_game)
        .game_acmd("expression_attacks3", shulk_attack_s3_expression)
        .game_acmd("expression_attacks3hi", shulk_attack_s3_expression)
        .game_acmd("expression_attacks3lw", shulk_attack_s3_expression)
        .game_acmd("sound_attacks3hi", shulk_sound_attacks3)
        .game_acmd("sound_attacks3lw", shulk_sound_attacks3)
        .game_acmd("effect_attacks3hi", shulk_effect_attacks3)
        .game_acmd("effect_attacks3lw", shulk_effect_attacks3)
        .game_acmd("game_attackhi3", shulk_attack_hi3_game)
        .game_acmd("game_attacklw3", shulk_attack_lw3_game)
        .install();
}
