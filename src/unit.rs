use crate::{save::*, gamedata::*};

#[skyline::from_offset(0x001db680)]
pub fn is_monastery() -> bool;

#[skyline::from_offset(0x0040c420)]
pub fn get_item_data_from_index(index: i16) -> &'static ItemData;

#[repr(C)]
pub struct BattleSkill {
    pub unit: Option<&'static BattleUnit>,
    pub check_ability: i32,
    pub skill_property: i32,
    pub battle_side: [u64; 2],
    pub battle_detail: u64,
    pub condition_check: i32,
    pub skill_value: i32,
}

#[repr(C)]
pub struct BattleUnit {
    pub inventory: [UnitItem; 6],
    pub battalion: UnitBattalion,
    pub seed: u32,
    pub character: i16,
    pub unk_0x26: i16,
    pub unk_0x28: i16,
    pub unk_0x2a: i16,
    pub exp: i16,
    pub equipped_weapon: i16,
    pub equipped_accessory: i16,
    pub skill_exp: [u16; 12],
    pub level: u8,
    pub class: u8,
    pub hp: u8,
    pub unk_0x4d: u8,
    pub stats: [u8; 9],
    pub combat_art_unlock: [u8; 10],
    pub ability_unlock: [u8; 30],
    pub equipped_abilities: [u8; 5],
    pub equipped_combat_art: [u8; 3],
    pub item_count: u8,
    pub skill_rank_level: [u8; 11],
    pub current_class_level: u8,
    pub magic_durability: [u8; 12],
    pub learned_magic: [u8; 12],
    pub flag: i32,
    pub junk: [u8; 80],
    pub adjutant_id: i8,
}

#[repr(C)]
pub struct UnitItem {
    pub item_id: i16,
    pub durability: i16,
}

#[repr(C)]
pub struct UnitBattalion {
    pub char_id: i16,
    pub exp: i16,
    pub endurance: i16,
    pub battalion_id: u8,
    pub gambit: u8,
}

#[repr(C)]
pub struct Unit {
    pub inventory: [UnitItem; 6],
    pub battalion: UnitBattalion,
    pub seed: u32,
    pub character: i16,
    pub unk_0x26: i16,
    pub unk_0x28: i16,
    pub unk_0x2a: i16,
    pub exp: i16,
    pub equipped_weapon: i16,
    pub equipped_accessory: i16,
    pub skill_exp: [u16; 11],
    pub current_class_exp: u16,
    pub level: u8,
    pub class: u8,
    pub hp: u8,
    pub unk_0x4d: u8,
    pub stats: [u8; 9],
    pub combat_art_unlock: [u8; 10],
    pub ability_unlock: [u8; 30],
    pub equipped_abilities: [u8; 5],
    pub equipped_combat_art: [u8; 3],
    pub item_count: u8,
    pub skill_rank_level: [u8; 11],
    pub current_class_level: u8,
    pub magic_durability: [u8; 12],
    pub learned_magic: [u8; 12],
    pub flag: i32,
    pub unk_0xb0: [u8; 14],
    pub ability_unlock2: [u8; 2],   // Ability 241 - 254
    pub unk_0xc0: [u8; 4],
    pub class_related_stuff: UnitClass,
    pub unk_0xeb: [u8; 5],
    pub unk_0xf3: [u8; 7],
    pub adjutant_id: u16,
    pub skill_exp2: [u16; 11],
    pub class_exp: [u16; 100],
    pub deploy_index: i8,
    pub deploy_slot: i8,
    pub skill_level: [u8; 11],
    pub class_level: [u8; 100],
}

#[repr(C)]
pub struct UnitClass {
    pub movitation: u8,
    pub teaching_related: [u8; 14],
    pub class_unlock_flags: [u8; 8],
    pub goal: u8, 
    pub possible_flags: [u8; 7],
    pub class_unlock_flags2: u8,
    pub unk:[u8; 9],
}

impl Unit {
    pub fn learn_ability(&self, ability: i32) {  unsafe { unit_learn_ability(self, ability) }; }
    pub fn learn_combat_art(&self, combat_art: i32) { unsafe { unit_learn_ability(&self, combat_art)}; }
    pub fn learn_from_ranks(&self, param2: u64) {  unsafe { unit_learn_from_ranks(self, param2)}; }
    pub fn cert_into_class(&self, class: i32) { unsafe { unit_cert_class(&self.class_related_stuff, class, true)}; }
    pub fn has_class(&self, class: i32) -> bool {  unsafe { has_cert(&self.class_related_stuff, class)}  }
    pub fn add_item(&self, item: i32) { unsafe { unit_add_item(self, item)}; }
    pub fn gain_skill_exp(&self, skill: i32, earn: i32) { unsafe { unit_gain_skill_exp(self, skill, earn)}; }
    pub fn get_ability_parameter(&self, ability_parameter: i32) -> i32 {
        unsafe { get_unit_ability_parameter(self, ability_parameter) }
    }

    pub fn grant_battalion(&self, battalion_id: i32, equip: bool) {
        let save = Save::get_instance();
        unsafe { grant_unit_battalion(save, self, battalion_id, equip); }

    }

    pub fn reclass(&self, class: i32, param3: bool, param4: u64, param5: u64) {
        unsafe { unit_reclass(self, class, param3, param4, param5)};
    }
}

#[skyline::from_offset(0x004149f0)]
pub fn has_cert(class: &UnitClass, class_id: i32) -> bool; 

#[skyline::from_offset(0x003d2a10)]
pub fn unit_cert_class(class_stuff: &UnitClass, class_id: i32, equip: bool);

#[skyline::from_offset(0x004124f0)]
pub fn unit_learn_ability(unit: &Unit, ability: i32);

#[skyline::from_offset(0x00412420)]
pub fn unit_learn_combat_art(unit: &Unit, combat_art: i32);

#[skyline::from_offset(0x003d2ac0)]
pub fn unit_reclass(unit: &Unit, class: i32, param3: bool, param4: u64, param5: u64);

#[skyline::from_offset(0x003dc1e0)]
pub fn grant_unit_battalion(save: &Save, unit: &Unit, battalion_id: i32, equip: bool);

#[skyline::from_offset(0x003ddb50)]
pub fn unit_add_item(unit: &Unit, item_id: i32);

#[skyline::from_offset(0x00411710)]
pub fn unit_learn_from_ranks(unit: &Unit, parm2: u64);

#[skyline::from_offset(0x00411590)]
pub fn unit_gain_skill_exp(unit: &Unit, skill_type: i32, earn: i32);

#[skyline::from_offset(0xa7e80)]
pub fn get_unit_ability_parameter(unit: &Unit, ability_index: i32) -> i32;