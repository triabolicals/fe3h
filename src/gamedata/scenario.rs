pub use super::*;
use crate::save::*;

#[repr(C)]
pub struct ScenarioData {
    pub unk_0x0: i16,
    pub unk_0x2: i16,
    pub unavailable_characters: [i16; 3],
    pub paralogue_characters: [i16; 2],
    pub unk_0xe: u8,
    pub chapter: u8,
    pub aux_battle_avaliability: u8,
    pub generic_battalion_level: [u8; 3],
    pub paralogue_expiration_chapter: [u8; 4],
    pub paralogue_expiration_day_label: [u8; 4],
    pub recommended_level: u8,
    pub bgm: u8,
    pub turn_limit: u8,
    pub victory_condition: u8,
    pub map: u8,
    pub generic_levels: [u8; 3],
    pub flag1: u8,
    pub flag2: u8,
    pub divine_pulse_inc: u8,
    pub divine_pulse_inc_maddening: u8,
    pub defeat_conditions: [u8; 5],
    pub padding: u8,
}

impl FixedDataTable<ScenarioData, 100> for ScenarioData {
    fn get_table() -> &'static mut FixedTable<'static, ScenarioData, 100> {
        return FixedTable::<'static, ScenarioData, 100>::get_table_mut(0x01b38a80);
    }
}

impl ScenarioData {
    pub fn get_current_scenario() -> &'static mut ScenarioData {
        let flow_entry = Save::get_current_flow();
        let scenario_id = flow_entry.scenario_id;
        return Self::get_entry(scenario_id as usize);
    }

}

