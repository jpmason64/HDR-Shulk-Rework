use super::*;

unsafe extern "C" fn miigunner_throw_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 361, 90, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 48, 100, 140, 10, 5.5, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(boma, true, false);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, 13.0, 0.0);
		//FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
		//FighterCutInManager::set_throw_finish_offset(boma, 9, -2, 0);
	}
	frame(lua_state, 12.0);
	FT_MOTION_RATE_RANGE(fighter, 12.0, 34.0, 18.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(boma);
	}
}

unsafe extern "C" fn miigunner_throw_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	// if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
	// 	FT_MOTION_RATE_RANGE(fighter, 1.0, 3.0, 8.0);
	// }
	if is_excute(fighter) {
		// if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
        //     VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
		// 	ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 52, 56, 2, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        // }
		// else {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 45, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		//}
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 3.0);
	FT_MOTION_RATE(fighter, 1.0);
	frame(lua_state, 10.0);
	// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 	FT_MOTION_RATE_RANGE(fighter, 10.0, 21.0, 9.0);
	// }
	// else {
	FT_MOTION_RATE_RANGE(fighter, 10.0, 20.0, 7.0);
	// }
	if is_excute(fighter) {
		REVERSE_LR(fighter);
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 20.0);
	// if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		FT_MOTION_RATE_RANGE(fighter, 20.0, 50.0, 18.0);
	// }
	if is_excute(fighter) {
		// if !VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, -1);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_b"), false, -1.0);
		// }
	}
	// frame(lua_state, 21.0);
	// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 	FT_MOTION_RATE_RANGE(fighter, 21.0, 50.0, 24.0);
	// }
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 142, 40, 0, 98, 18.0, 0.0, 22.0, -31.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
	// 	}
	// }
	// frame(lua_state, 24.0);
    // if is_excute(fighter) {
    //     AttackModule::clear_all(boma);
    // }
	frame(lua_state, 50.0);
	FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn miigunner_rapidshot_bullet_flythrowb_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 160, 0, 20, 4.0, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn miigunner_throw_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	// frame(lua_state, 4.0);
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18.0, -27.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
	// 		LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.15, 0.15);
	// 		LAST_EFFECT_SET_RATE(fighter, 1.25);
	// 		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
	// 		LAST_EFFECT_SET_RATE(fighter, 2.0);
	// 	}
	// }
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	// frame(lua_state, 19.0);
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 13, -13, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
	// 	}
	// }
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	EFFECT(fighter, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 70, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
		// 	LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
		// 	EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
		// 	LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
		// 	LAST_EFFECT_SET_RATE(fighter, 1.6);
		// 	EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 19.0, -26.5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		// }
		// else {
		EFFECT(fighter, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 70, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 1.6);
		// }
	}
}

unsafe extern "C" fn miigunner_throw_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack01"));
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
	// wait(lua_state, 9.0);
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
	// 	}
	// }
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
	}
	// frame(lua_state, 22.0);
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s04"));
	// 	}
	// }
}

unsafe extern "C" fn miigunner_throw_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	frame(lua_state, 1.0);
	// if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
	// 	FT_MOTION_RATE_RANGE(fighter, 1.0, 7.0, 12.0);
	// }
	if is_excute(fighter) {
		// if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
        //     VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
		// 	ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 54, 2, 76, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		// }
		// else {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 100, 40, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		// }
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	frame(lua_state, 7.0);
	FT_MOTION_RATE(fighter, 1.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 90, 100, 40, 0, 10.0, 0.0, 42.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		// }
		// else {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi"), false, 0.0);
		// }
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		AttackModule::clear_all(boma);
	}
	// frame(lua_state, 28.0);
	// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 	FT_MOTION_RATE_RANGE(fighter, 28.0, 52.0, 27.0);
	// }
	// else {
	FT_MOTION_RATE_RANGE(fighter, 28.0, 52.0, 17.0);
	// }
	if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 270, 100, 80, 0, 14.0, 0.0, 50.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		// }
		// else {
		ArticleModule::generate_article(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, false, 0);
		ArticleModule::change_motion(boma, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_RAPIDSHOT_BULLET, Hash40::new("fly_throw_hi_2"), false, 0.0);
		// }
	}
	// frame(lua_state, 30.0);
	// if is_excute(fighter) {
	// 	AttackModule::clear_all(boma);
	// }
	frame(lua_state, 52.0);
	FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn miigunner_rapidshot_bullet_flythrowhi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 90, 77, 0, 89, 6.5, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(10.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATK_POWER(fighter, 0, 3);
    }
}

unsafe extern "C" fn miigunner_rapidshot_bullet_flythrowhi2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 0, 0, 0, 5.6, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATK_POWER(fighter, 0, 3);
    }
}

unsafe extern "C" fn miigunner_throw_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	// frame(lua_state, 2.0);
	// if is_excute(fighter) {
	// 	if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
	// 		EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
	// 		LAST_EFFECT_SET_RATE(fighter, 2.0);
	// 	}
	// }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
		// 	LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
		// 	LAST_EFFECT_SET_RATE(fighter, 1.6);
		// 	EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 36.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
		// }
		// else {
		EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 1.6);
		// }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
		// 	LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
		// 	LAST_EFFECT_SET_RATE(fighter, 1.6);
		// 	EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 43.0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		// }
		// else {
		EFFECT(fighter, Hash40::new("miigunner_blaster_shot"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, true);
		LAST_EFFECT_SET_RATE(fighter, 1.6);
		EFFECT(fighter, Hash40::new("miigunner_shot_add"), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.5, 0, 0, 0, 0, 0, 0, true);
		// }
    }
}

unsafe extern "C" fn miigunner_throw_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack01"));
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
		// }
		// else {
		PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
		// }
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
		// if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
		// 	PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s04"));
		// }
		// else {
		PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
		// }
    }
}

unsafe extern "C" fn miigunner_throw_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 92, 38, 0, 95, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 0, 0);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: -3.0, z: 0.0});
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

pub fn install() {
    smashline::Agent::new("miigunner_rapidshot_bullet")
        .acmd("game_flythrowb", miigunner_rapidshot_bullet_flythrowb_game)
        .acmd("game_flythrowhi", miigunner_rapidshot_bullet_flythrowhi_game)
        .acmd("game_flythrowhi2", miigunner_rapidshot_bullet_flythrowhi2_game)
        .install();
    smashline::Agent::new("miigunner")
        .acmd("game_throwf", miigunner_throw_f_game)
        .acmd("game_throwb", miigunner_throw_b_game)
        .acmd("effect_throwb", miigunner_throw_b_effect)
        .acmd("sound_throwb", miigunner_throw_b_sound)
        .acmd("game_throwhi", miigunner_throw_hi_game)
        .acmd("effect_throwhi", miigunner_throw_hi_effect)
        .acmd("sound_throwhi", miigunner_throw_hi_sound)
        .acmd("game_throwlw", miigunner_throw_lw_game)
        .install();
}