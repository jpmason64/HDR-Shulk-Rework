// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

const SHULK_AERIAL_HIT : i32 = 0x200000ea;
const SHULK_GROUND_HIT : i32 = 0x200000eb;
const SHULK_THROW_HIT : i32 = 0x200000ec;
const SHULK_SMASHES_HIT : i32 = 0x200000ed;
const SHULK_TILTS_HIT : i32 = 0x200000ee;

static mut TILT: bool = false;
 
unsafe fn air_slash_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if frame > 23.0 {
                if boma.is_cat_flag(Cat1::AirEscape) {
                    VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                    ControlModule::reset_trigger(boma);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }
    }
}

// Fixes weird vanilla behavior where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_FALL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_FALL);
    }
    else if fighter.is_status(*FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD)
    && fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N,
        *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_ATTACK
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

//Allows you to cancel out of Shulk's down tilt
unsafe fn down_tilt_cancel(fighter: &mut L2CFighterCommon){
    //Checks if the attack is a down tilt
    if(MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3")){
        //Checks if the frame is between the jump-canceleable window
        if(MotionModule::frame(fighter.module_accessor) >= 15.0 && MotionModule::frame(fighter.module_accessor) < 31.0){
            //Checks for whether the player jumped
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP){
                //Jump button!
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
            }
        }
    }
}

unsafe fn arts_cancelling(fighter: &mut L2CFighterCommon, status_kind: i32) {
    if fighter.is_motion_one_of(&[
        Hash40::new("attack_13"),
        Hash40::new("attack_s3_s"),
        Hash40::new("attack_s3_hi"),
        Hash40::new("attack_s3_lw"),
        Hash40::new("attack_hi3"),
        Hash40::new("attack_lw3"),
        Hash40::new("attack_air_n"),
        Hash40::new("attack_air_f"),
        Hash40::new("attack_air_b"),
        Hash40::new("attack_air_hi"),
        Hash40::new("attack_air_lw") ]) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            // println!("beat");
            VarModule::on_flag(fighter.object(), vars::shulk::status::MONADO_BEAT);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    air_slash_cancels(boma, id, status_kind, cat[0], frame);
    up_special_proper_landing(fighter);
    fastfall_specials(fighter);
    down_tilt_cancel(fighter);
    arts_cancelling(fighter, status_kind);
}

pub extern "C" fn shulk_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		shulk_frame(fighter)
    }
}

pub unsafe fn shulk_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }

    if fighter.is_motion_one_of(&[
        Hash40::new("attack_s3_s"),
        Hash40::new("attack_s3_hi"),
        Hash40::new("attack_s3_lw"),]){
            TILT = true;
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), true);
    }

    if fighter.is_motion_one_of(&[
        Hash40::new("jump_squat"),
        Hash40::new("run"),
        Hash40::new("run_start"),
        Hash40::new("turn"),
        Hash40::new("turn_dash"),
        Hash40::new("dash"),
        Hash40::new("dash_b"),
        Hash40::new("dash_b_run"),
        Hash40::new("wait"),
        Hash40::new("wait_2"),
        Hash40::new("wait_3"),
        Hash40::new("wait_4"),
        Hash40::new("wait_5"),
        Hash40::new("walk_slow"),
        Hash40::new("walk_slow_b"),
        Hash40::new("walk_middle"),
        Hash40::new("walk_middle_b"),
        Hash40::new("walk_fast"),
        Hash40::new("walk_fast_b")]){
            TILT = false;
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadbehind"), true);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("monadhand"), false);
        }

    //Aerials
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_f") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_b") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_hi") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_lw") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_n") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_AERIAL_HIT);
    // }
    // //Ground Attacks
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_13") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_GROUND_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_dash") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_GROUND_HIT);
    // }
    // //Smash Attacks
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s4") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_SMASHES_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s4_hi") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_SMASHES_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s4_lw") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_SMASHES_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_hi_4") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_SMASHES_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_lw_4") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_SMASHES_HIT);
    // }
    // //Throws
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("throw_f") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_THROW_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("throw_b") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_THROW_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("throw_hi") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_THROW_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("throw_lw") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_THROW_HIT);
    // }
    // //Tilt Attacks
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s3") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s3_hi") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_s3_lw") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_lw_3") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    // }
    // if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_hi_3") {
    //     WorkModule::off_flag(fighter.module_accessor, SHULK_TILTS_HIT);
    // }

}
pub fn install() {
    smashline::Agent::new("shulk")
        .on_line(Main, shulk_frame_wrapper)
        .install();
}
