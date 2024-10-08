#[repr(C)]
pub struct BaiUnit {
    pub character: i32,
    pub class: i16,
    pub inventory: [i16; 7],
    pub abilities: [u8; 5],
    pub x: u8,
    pub y: u8,
    pub rotation: u8,
    pub set_level: u8,
    pub force: u8,
    pub script_spawn: u8,
    pub movement_type: u8,
    pub battalion_enabled: u8,
    pub deployment_type: u8,
    pub assigned_battalion: u8,
    pub battalion_level: u8,
    pub assigned_magic: [u8; 2],
    pub assigned_combat_art: u8,
    pub assigned_gender: u8,
    pub weapon_weakness: u8,
    pub assigned_faction: u8,
    pub padding: u16,
}

