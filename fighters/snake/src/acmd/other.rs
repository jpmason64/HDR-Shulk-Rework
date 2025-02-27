
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));
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
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_snake_rnd_futtobi01"), Hash40::new("seq_snake_rnd_futtobi02"));}
    }
}

unsafe extern "C" fn snake_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 8.2, 0.0, Some(0.0), Some(8.2), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_snake_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_snake_step_left_m"));
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_snake_step_right_m"));
    }
}

unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn snake_side_taunt_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_win03"));
    }
}

unsafe extern "C" fn snake_up_taunt_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 2, 360/*angle*/, 10/*size*/, 1, 0, 12, 30, 30, 80);
        // physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealhi"));
    }
}

unsafe extern "C" fn snake_down_taunt_explode_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_DESIRED_RATE(fighter, 80.0, 50.0);
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0 );
        ArticleModule::shoot(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(lua_state, 80.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        // WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        // SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = true;
        ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn snake_down_taunt_explode_exp(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0 );
    }
    frame(lua_state, 30.0);
    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 80.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn snake_down_taunt_explode_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealend"));
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l04"));
        PLAY_SE(fighter, Hash40::new("se_snake_squat"));
    }
    // frame(lua_state, 70.0);
    // if is_excute(fighter) {
    // }
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l05"));
    }
}

unsafe extern "C" fn snake_down_taunt_explode_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 75.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn snake_trenchmortar_bullet_impact_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
        KineticModule::unable_energy(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_KINETIC_ENERGY_ID_GRAVITY);
        VisibilityModule::set_int64(boma, hash40("main") as i64, hash40("impact") as i64);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 77, 80, 0, 45, 12.0, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_STATUS_FLAG_ENABLE_ADVANCE_STATUS);
    }
}

unsafe extern "C" fn snake_c4_target_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if VarModule::is_flag(fighter.object(), vars::snake::instance::SELF_STICK) {
            SEARCH(fighter, 0, 0, Hash40::new("rot"), 0.1, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
        else {
            SEARCH(fighter, 0, 0, Hash40::new("rot"), 5.0, 0.0, -3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
    }
}

unsafe extern "C" fn snake_c4_stick_other_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    for _ in 0..5 {
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        //EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

unsafe extern "C" fn snake_c4_stick_target_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        
    }
}

unsafe extern "C" fn snake_c4_stick_target_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon_ready"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_final_lockon_ready2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    wait(lua_state, 60.0);
    if is_excute(fighter) {
        //EFFECT_OFF_KIND(fighter, Hash40::new("snake_final_lockon_ready"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("snake_final_lockon_ready2"), false, false);
    }
    wait(lua_state, 90.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
    }
    for _ in 0..4 {
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

unsafe extern "C" fn snake_c4_stick_target_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_C3){
            PLAY_SE(fighter, Hash40::new("se_snake_special_l08"));        
        }
        else{
            PLAY_SE(fighter, Hash40::new("se_snake_special_l03"));
        }
        PLAY_SE(fighter, Hash40::new("se_snake_final02"));
    }
}

unsafe extern "C" fn snake_c4_explosion_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        // Hitbox for opponents
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 16.0, 86, 78, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        // Snake-only hitbox
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 16.0, 86, 78, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        VisibilityModule::set_whole(boma, false);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::set_size(boma, 0, 17.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if !WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

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
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//tranq gun

unsafe extern "C" fn snake_tranq_gun_start_snd(fighter : &mut L2CAgentBase) {
}

unsafe extern "C" fn snake_tranq_gun_shoot_snd(fighter : &mut L2CAgentBase) {
}

//tranq dart

unsafe extern "C" fn snake_tranq_dart_fly_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 1, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn snake_tranq_dart_fly_snd(fighter : &mut L2CAgentBase) {
}

unsafe extern "C" fn snake_tranq_dart_fly_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.3, true);
        // LAST_PARTICLE_SET_COLOR(fighter, 0.6, 0.6, 2.8);
        LAST_PARTICLE_SET_COLOR(fighter, 2.5, 2.5, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.001);

        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke"), true, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke2"), true, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke3"), true, true);
        }
        wait(lua_state, 5.0);
    }
}

unsafe extern "C" fn snake_tranq_dart_fall_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
}

unsafe extern "C" fn snake_tranq_dart_explode_game(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 45, 0, 0, 30, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BOMB);
    }
}

unsafe extern "C" fn snake_tranq_dart_fall_explode_game(fighter : &mut L2CAgentBase) {
}

unsafe extern "C" fn snake_tranq_dart_explode_snd(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        let sfx = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_snake_special_l02"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, sfx as i32, 2.0, 0);
    }
}

unsafe extern "C" fn snake_tranq_dart_explode_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_piyo"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn snake_tranq_dart_land_eff(fighter : &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}
////

pub fn install() {
    smashline::Agent::new("snake")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("game_catch", snake_catch_game)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("sound_appealsr", snake_side_taunt_snd)
        .acmd("sound_appealhir", snake_up_taunt_snd)
        .acmd("game_appealendexplode", snake_down_taunt_explode_game)
        .acmd("expression_appealendexplode", snake_down_taunt_explode_exp)
        .acmd("sound_appealendexplode", snake_down_taunt_explode_snd)
        .acmd("effect_appealendexplode", snake_down_taunt_explode_eff)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .install();
    smashline::Agent::new("snake_c4")
        .acmd("game_establishtarget", snake_c4_target_game)
        .acmd("effect_stickother", snake_c4_stick_other_effect)
        .acmd("game_sticktarget", snake_c4_stick_target_game)
        .acmd("effect_sticktarget", snake_c4_stick_target_effect)
        .acmd("sound_sticktarget", snake_c4_stick_target_sound)
        .acmd("game_explosion", snake_c4_explosion_game)
        .install();
    smashline::Agent::new("snake_nikita")
        .acmd("sound_start", snake_tranq_gun_start_snd)
        .acmd("sound_shoot", snake_tranq_gun_shoot_snd)
        .install();
    smashline::Agent::new("snake_nikitamissile")
        .acmd("game_fly", snake_tranq_dart_fly_game)
        .acmd("game_flyattackcommon", snake_tranq_dart_fly_game)
        .acmd("game_stopfall", snake_tranq_dart_fly_game)
        .acmd("sound_fly", snake_tranq_dart_fly_snd)
        .acmd("sound_stopfall", snake_tranq_dart_fly_snd)
        .acmd("effect_fly", snake_tranq_dart_fly_eff)
        .acmd("effect_stopfall", snake_tranq_dart_fall_eff)
        .acmd("game_explosion", snake_tranq_dart_explode_game)
        .acmd("game_fallexplosion", snake_tranq_dart_fall_explode_game)
        .acmd("game_hiexplosion", snake_tranq_dart_fall_explode_game)
        .acmd("sound_explosion", snake_tranq_dart_explode_snd)
        .acmd("sound_fallexplosion", snake_tranq_dart_explode_snd)
        .acmd("sound_hiexplosion", snake_tranq_dart_explode_snd)
        .acmd("effect_explosion", snake_tranq_dart_explode_eff)
        .acmd("effect_hiexplosion", snake_tranq_dart_explode_eff)
        .acmd("effect_fallexplosion", snake_tranq_dart_land_eff)
        .install();
    smashline::Agent::new("snake_trenchmortar")
        .acmd("game_impact", snake_trenchmortar_bullet_impact_game)
        .install();
}
