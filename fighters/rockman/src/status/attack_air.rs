use super::*;

unsafe extern "C" fn rockman_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [
        hash40("attack_air_hi"), hash40("attack_air_lw")
    ].contains(&mot) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

unsafe extern "C" fn rockman_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackAir()
}

pub fn install() {
    smashline::Agent::new("rockman")
        .status(
            Main,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            rockman_attack_air_main,
        )
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, rockman_attack_air_end)
        .install();
}
