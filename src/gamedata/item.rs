pub use super::*;

#[repr(C)]
pub struct ItemData {
    pub item_when_broken: i16,
    pub skill_properies: [u8; 2],
    pub weapon_effect: u8,
    pub weapon_type: u8,
    pub rank: u8,
    pub max_range: u8,
    pub weapon_model_id: u8,
    pub crests: u8,
    pub min_range: u8,
    pub uses: u8,
    pub extra_effects: u8,
    pub item_type: u8,
    pub effectiveness: u8,
    unk2: u8,
    pub increase: u8,
    pub stat: u8,
    pub crit: u8,
    pub weight: u8,
    pub flags: [u8; 3],
    padding: u8,
}

impl ItemData {
    pub fn get_weapon_table() -> &'static mut FixedTable<'static, ItemData, 500> {
        return FixedTable::<'static, ItemData, 500>::get_table_mut(0x01b386e8);
    }
    pub fn get_equip_table() -> &'static mut FixedTable<'static, ItemData, 50> {
        return FixedTable::<'static, ItemData, 50>::get_table_mut(0x01b38710);
    } 
    pub fn get_magic_table() -> &'static mut FixedTable<'static, ItemData, 38> {
        return FixedTable::<'static, ItemData, 38>::get_table_mut(0x01b386f0);
    }
    pub fn get_item_table() -> &'static mut FixedTable<'static, ItemData, 200> {
        return FixedTable::<'static, ItemData, 200>::get_table_mut(0x01b38718);
    }
}