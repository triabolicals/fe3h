pub use super::*;
use std::{fs, fs::File, io::Write};


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

#[repr(C)]
pub struct PlayablePersonData {
    pub timeskip_autolevels: u8,
    pub unk0x1: u8,
    pub faction_pallete: u8,
    pub pre_cert_classes: [u8; 5],
    pub flag: u8, 
    pub starting_ranks: [u8; 11],
    pub skill_prof: [u8; 11],
    pub ts_classes: [u8; 3],
}

#[repr(C)]
pub struct BuddingTalentData {
    pub character: i16,
    pub instructs: u8,
    pub skill: u8,
    pub combat_art: u8,
    pub ability: u8,
}
impl FixedDataTable<PersonData, 1201> for PersonData {
    fn get_table() -> &'static mut FixedTable<'static, PersonData, 1201> {
        return FixedTable::<'static, PersonData, 1201>::get_table_mut(0x01b387e8);
    }
}

impl FixedDataTable<PlayablePersonData, 45> for PlayablePersonData {
    fn get_table() -> &'static mut FixedTable<'static, PlayablePersonData, 45> {
        return FixedTable::<'static, PlayablePersonData, 45>::get_table_mut(0x01b387f0);
    }
}

impl FixedDataTable<BuddingTalentData, 240> for BuddingTalentData {
    fn get_table() -> &'static mut FixedTable<'static, BuddingTalentData, 240> {
        return FixedTable::<'static, BuddingTalentData, 240>::get_table_mut(0x01b38840);
    }
}

impl BuddingTalentData {
    pub fn get_data(character: i32, skill: u8) -> Option<&'static BuddingTalentData> {
        let table = Self::get_table();
        for x in 0..240 {
            let entry = &table.entries[x as usize];
            if entry.entry.character as i32  == character && skill == entry.entry.skill {
                return Some(entry.entry);
            }
        }
        return None;

    }
}

pub struct SkillGoals {
    pub value: i16,
}

impl FixedDataTable<SkillGoals, 200> for SkillGoals {
    fn get_table() -> &'static mut FixedTable<'static, SkillGoals, 200> {
        return FixedTable::<'static, SkillGoals, 200>::get_table_mut(0x01b38a50);
    }
}

impl SkillGoals {
    pub fn get_goal_skills(index: usize) -> (i32, i32) {
        let entry = SkillGoals::get_entry(index);
        let mut out: [i32; 2] = [11; 2];
        for x in 0..11 {
            if entry.value & (1 << x) != 0 {
                if out[0] == 11 {
                    out[0] = x as i32;
                }
                else if out[1] == 11 {
                    out[1] = x as i32;
                    break;
                }
            }
        }
        return (out[0], out[1]);
    }
}