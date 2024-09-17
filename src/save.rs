use crate::{gamedata::*, unit::*};

#[repr(C)]
pub struct Save {
    pub items: [UnitItem; 400],
    pub item_count: u32,
    pub unit_list: [Unit; 60],
    junk: [u8; 106988],
    pub player: PlayerData,
    pub activities: ActivityData,
}

#[repr(C)]
pub struct PlayerData {
    junk: [u8; 4208],
    pub playtime: u32,
    pub money: u32,
    pub unk: u32,
    pub chapter: i32,
    pub support_exp: [u16; 270],
    pub difficulty: u8,
    pub game_mode: u8,
    pub route: u8,
    pub timeskip: bool,
    pub map_id: u8,
    pub monastery: i8,
    pub flags_ : [u8; 36],
    pub the_timeskip: bool, 
    junk2: [u8; 3073],
}

#[repr(C)]
pub struct ActivityData {}

impl Save {
    pub fn get_instance() -> &'static mut Save {
        let save = crate::gamedata::offset_to_addr::<&mut Save>(0x01b12190);
        unsafe { return *save; }
    }
    pub fn get_activity_data() -> &'static ActivityData {
        let save = Self::get_instance();
        return &save.activities;
    }

    pub fn get_player_data() -> &'static PlayerData {
        let save = Self::get_instance();
        return &save.player;
    }

    pub fn setup_unit_recruitment(&self, character_id: i32, map_id: i32, battalion: bool) {
        let save = Self::get_instance();
        unsafe { setup_for_recruitment(save, character_id, map_id, battalion);  }
    }

    pub fn recruit_character(&self, character_id: i32) {
        let save = Self::get_instance();
        unsafe { save_data_recruitment(save, character_id);  }
    }

    pub fn get_character(character_id: i32) -> Option<&'static mut Unit> {
        let save = Self::get_instance();
        unsafe { get_unit_from_save(save, character_id as u32)  }
    }

    pub fn scenario_recruitment(&self, character_id: i32, map_id: i32, mission_assistance: bool) {
        let save = Self::get_instance();
        unsafe { scenario_recruitment(save, character_id, map_id, mission_assistance)  };
    }

    pub fn get_current_flow() -> &'static crate::gamedata::FlowEntry {
        let save = Self::get_instance();
        let route = save.player.route as i32;
        let chapter = save.player.chapter;
        unsafe { get_fixed_flow_entry(route, chapter) }
    }

    pub fn set_route(&self, route: i32) {
        unsafe { save_set_route(self, route) };
    }



}

impl PlayerData {
    pub fn set_flag(&self, flag: i32, status: bool) {
        unsafe { set_flag(self, flag, status) };
    }
    pub fn get_flag(&self, flag: i32) -> bool {
        unsafe { get_flag(self, flag) }
    }
}

impl ActivityData {
    pub fn get_instance() -> &'static Self {
        let save = Save::get_instance();
        &save.activities
    }
    pub fn get_quest_status(&self, quest: i32) -> i32 {
        unsafe { activity_get_quest_status(self, quest) }
    }

    pub fn set_quest_status(&self, quest: i32, status: i32) {
        unsafe {set_quest_status(self, quest, status) };
    }
    pub fn set_paralogue_status(&self, map_id: i32, status: bool) {
        unsafe { set_paralogue(self, map_id, status) };
    }
}



#[skyline::from_offset(0x003da660)]
fn save_set_route(save: &Save, route: i32);

#[skyline::from_offset(0x003d0dc0)]
fn setup_for_recruitment(savedata: &Save,char_id: i32, map_id: i32, param: bool);

#[skyline::from_offset(0x0003d0b80)]
fn save_data_recruitment(savedata: &Save, char_ide: i32);

#[skyline::from_offset(0x03caf30)]
fn get_unit_from_save(save: &Save, char_id: u32) -> Option<&'static mut Unit>;

#[skyline::from_offset(0x003cafc0)]
fn scenario_recruitment(save: &Save, char_id: i32, map_id: i32, mission_assistance: bool);

#[skyline::from_offset(0x003d8620)]
fn get_fixed_flow_entry(route: i32, chapter: i32) -> &'static FlowEntry;

// PlayerData
#[skyline::from_offset(0x003d8740)]
fn set_flag(player: &PlayerData, flag: i32, status: bool);

#[skyline::from_offset(0x003de440)]
fn get_flag(player: &PlayerData, flag: i32) -> bool;
// ActivityData
#[skyline::from_offset(0x003ddef0)]
fn set_quest_status(activities: &ActivityData, quest: i32, status: i32);

#[skyline::from_offset(0x003f17a0)]
fn set_paralogue(activities: &ActivityData, map_id: i32, set: bool);

#[skyline::from_offset(0x003eeb30)]
fn activity_get_quest_status(activities: &ActivityData, quest: i32) -> i32;
