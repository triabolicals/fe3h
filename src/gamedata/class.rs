pub use super::*;

#[repr(C)]
pub struct ClassData {
    frame: [u16; 4],    //0x0
    pub male_aid: i16,  // 0x8
    pub female_aid: i16,    //0xa
    unknown: [ u16; 2], 
    pub hp_mod: u8, //0x10
    pub mount: u8,
    pub adjutant: u8,
    pub enemy_hp_growth: i8,
    pub player_hp_growth: i8,
    pub player_growth: [i8; 9],
    pub enemy_growth: [i8; 9],
    pub stat_mod: [i8; 9],
    pub dismount_mod: [i8; 9],
    pub skill_exp_bonus: [u8; 11],
    pub class_item: i8,
    pub class_tier: u8,
    pub mount_scale: u8,
    pub unit_scale: u8,
    unknown2: u8,
    pub monster_index: i8,
    pub class_exp_requirement: i8,
    pub base_hp: u8,
    pub exp_coefficient: u8,
    pub movement_type: u8,
    pub movement_type2: u8,
    pub default_weapon: u8,
    pub secondary_weapon: u8,
    pub effectiveness: u8,
    pub class_flag: u8,
    pub generic_magic: [u8; 2],
    pub base_stats: [u8; 9],
    pub generic_combat_art: [u8; 4],
    pub cs_generic_normal_abilities: [u8; 5],
    pub cs_generic_hard_abilities: [u8; 5],
    pub generic_hard_abilities: [u8; 5],
    pub generic_maddening_abilities: [u8; 5],
}

impl FixedDataTable<ClassData, 100> for ClassData {
    fn get_table() -> &'static mut FixedTable<'static, ClassData, 100> {
        return FixedTable::<'static, ClassData, 100>::get_table_mut(0x01b38798);
    }
}