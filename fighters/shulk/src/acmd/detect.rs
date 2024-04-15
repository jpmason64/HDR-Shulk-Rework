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
    
    skyline::install_hook!(
        notify_log_event_collision_hit_replace
    );
}
