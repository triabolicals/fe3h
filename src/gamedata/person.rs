pub use super::*;

#[repr(C)]
pub struct PersonData {
    pub model_scale_part1: f32,
    pub female_chest_scale_part1 : f32,
    pub model_scale_part2: f32,
    pub female_chest_scale_part2: f32,
    pub head: i16,
    pub name: i16,
    pub unk_0x14: i16,
    pub voice_id: i16,
    pub asset_id: i16,
    pub default_class: i8,
    pub age: i8,
    pub birthmonth: i8,
    pub starting_level: i8,
    pub unk_0x1e: i8,
    pub birthday: i8,
    pub save_data: i8,
    pub unk_0x21: i8,
    pub hp_cap: u8,
    pub unk_0x23: u8,
    pub faction: u8,
    pub unk_0x25: u8,
    pub gender: u8,
    pub body_type: u8,
    pub battalion: u8,
    pub hp_growth: u8,
    pub act_set: u8,
    pub base_hp: u8,
    pub crest: u8,
    pub secondary_crest: u8,
    pub flags: u8,
    pub height_part1: u8,
    pub height_part2: u8,
    pub unk_0x31: u8,
    pub unk_0x32: u8,
    pub base_stat: [u8; 9],
    pub growth: [u8; 9],
    pub cap: [u8; 9],
    pub padding: [u8; 2],
}

impl FixedDataTable<PersonData, 1201> for PersonData {
    fn get_table() -> &'static mut FixedTable<'static, PersonData, 1201> {
        return FixedTable::<'static, PersonData, 1201>::get_table_mut(0x01b387e8);
    }
}
