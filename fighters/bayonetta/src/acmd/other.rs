use super::*;

unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));
    }
}

unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_bayonetta_rnd_futtobi01"), Hash40::new("seq_bayonetta_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn bayonetta_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 6.6, 0.0, Some(0.0), Some(6.6), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }  
}

unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_bayonetta_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_bayonetta_step_right_l"));
    }
}

unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn bayonetta_wickedweavearm_attack_s4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(15.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 9.0, 0.0, 11.0, 25.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(15.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 9.0, 0.0, 11.0, 25.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 10.0, 0.0, 11.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 105, 0, 34, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 10.0, 0.0, 11.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 361, 100, 0, 35, 9.0, 0.0, 11.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 361, 100, 0, 35, 9.0, 0.0, 11.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }   
}

unsafe extern "C" fn bayonetta_wickedweavearm_attack_s4_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 9.0, 0.0, 7.0, 24.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 9.0, 0.0, 7.0, 24.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 10.0, 0.0, 6.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 10.0, 0.0, 6.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 49, 100, 0, 35, 9.0, 0.0, 6.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 49, 100, 0, 35, 9.0, 0.0, 6.0, 30.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
}

unsafe extern "C" fn bayonetta_wickedweavearm_attack_s4_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 9.0, 0.0, 15.0, 25.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 9.0, 0.0, 15.0, 25.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 10.0, 0.0, 16.0, 30.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 34, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 16.0, 49, 100, 0, 35, 10.0, 0.0, 16.0, 30.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 49, 100, 0, 35, 9.0, 0.0, 16.0, 30.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 15.0, 49, 100, 0, 35, 9.0, 0.0, 16.0, 30.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
}

unsafe extern "C" fn bayonetta_wickedweavearm_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 89, 90, 0, 38, 9.0, 0.0, 9.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 89, 90, 0, 38, 9.0, 0.0, 9.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 89, 90, 0, 36, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 89, 90, 0, 36, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 89, 90, 0, 36, 10.5, 0.0, 15.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 89, 90, 0, 36, 10.5, 0.0, 15.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 25.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 25.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 36.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 89, 90, 0, 32, 10.5, 0.0, 36.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ModelModule::set_scale(boma, 0.96);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 89, 90, 0, 32, 10.0, 0.0, 32.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 89, 90, 0, 32, 10.0, 0.0, 32.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 51.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    } 
}

unsafe extern "C" fn bayonetta_wickedweaveleg_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        // Ground-only
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 273, 10, 0, 150, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only 
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 270, 63, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        // Ground-only
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 273, 10, 0, 150, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // Air-only
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 270, 63, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
    }
}

unsafe extern "C" fn bayonetta_specialn_bullet_move_charge_bullet_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.7, 45, 200, 0, 15, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_02, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn bayonetta_specialn_bullet_move_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.35, 361, 40, 0, 5, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -0.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn escape_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
}

unsafe extern "C" fn escape_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
}

unsafe extern "C" fn escape_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
}

unsafe extern "C" fn down_left_taunt_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 175.0);
    if is_excute(fighter) {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT_RV, false);
    }
}

unsafe extern "C" fn side_taunt_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 144/(90-1));
}

unsafe extern "C" fn up_taunt_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 210/(100-1));
}

unsafe extern "C" fn sound_appealhi  (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
		if is_excute(fighter) {
			fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("vc_bayonetta_appeal01"));
            sv_animcmd::PLAY_DAMAGESTOP(fighter.lua_state_agent);
		}
		frame(lua_state, 4.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_bayonetta_appeal_h01"));
		}
		frame(lua_state, 8.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_bayonetta_appeal_h02"));
		}
		frame(lua_state, 25.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_bayonetta_step_right_s"));
		}
        frame(lua_state, 38.0);
        if is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("vc_bayonetta_appeal01_02"));
            sv_animcmd::PLAY_DAMAGESTOP(fighter.lua_state_agent);
        }
		frame(lua_state, 41.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_bayonetta_appeal_h03"));
		}
		frame(lua_state, 45.0);
		if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_bayonetta_step_right_m"));
		}
	}

    
unsafe extern "C" fn sound_appeallwr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01_02"));
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01_03"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_bayonetta_win08"));
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_step_right_m"));
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01_04"));
    }
    frame(fighter.lua_state_agent, 106.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01_05"));
    }
    frame(fighter.lua_state_agent, 127.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win01_06"));
    }
}

unsafe extern "C" fn sound_appeallwl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win02_02"));
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_win02_03"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_bayonetta_win09"));
    }
}

unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::common::status::PUMMEL_OVERRIDE_GLOBAL_STATS);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 100, 30, 0, 5.0, 0.0, 12.0, 10.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.7, 361, 100, 30, 0, 5.0, 0.0, 12.0, 10.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn bayonetta_landing_heavy_effect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::Agent::new("bayonetta")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_catch", bayonetta_catch_game)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("game_escapen", escape_n_game)
        .acmd("game_escapef", escape_f_game)
        .acmd("game_escapeb", escape_b_game)
        .acmd("game_appeallwl", down_left_taunt_game)
        .acmd("game_appealsr", side_taunt_game)
        .acmd("game_appealsl", side_taunt_game)
        .acmd("game_appealhir", up_taunt_game)
        .acmd("game_appealhil", up_taunt_game)
        .acmd("sound_appealhir", sound_appealhi)
        .acmd("sound_appealhil", sound_appealhi)
        .acmd("sound_appeallwr", sound_appeallwr)
        .acmd("sound_appeallwl", sound_appeallwl)
        .acmd("game_catchattack", game_catchattack)
        .acmd("effect_landingheavy", bayonetta_landing_heavy_effect)
        .install();
    smashline::Agent::new("bayonetta_wickedweavearm")
        .acmd("game_attacks4", bayonetta_wickedweavearm_attack_s4_game)
        .acmd("game_attacks4lw", bayonetta_wickedweavearm_attack_s4_lw_game)
        .acmd("game_attacks4hi",bayonetta_wickedweavearm_attack_s4_hi_game)
        .acmd("game_attackhi4", bayonetta_wickedweavearm_attack_hi4_game)
        .install();
    smashline::Agent::new("bayonetta_wickedweaveleg")
        .acmd("game_attacklw4", bayonetta_wickedweaveleg_attack_lw4_game)
        .install();
    smashline::Agent::new("bayonetta_specialn_bullet")
        .acmd("game_movechargebullet", bayonetta_specialn_bullet_move_charge_bullet_game)
        .acmd("game_move", bayonetta_specialn_bullet_move_game)
        .install();
}
