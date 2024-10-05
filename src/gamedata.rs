pub mod person;
pub mod class;
pub mod item;
pub mod scenario;


pub fn offset_to_addr<T>(offset: usize) -> *mut T {
    unsafe { (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize + offset) as *mut T }
}


#[repr(C)]
pub struct FixedTableEntry<'a, T> {
    some_ptr: *const (),
    pub entry: &'a mut T,
    pub index: u64,
}

#[repr(C)]
pub struct FixedSection<T, const N: usize> {
    pub header: FixedSectionHeader,
    pub entries: [T; N],
}

#[repr(C)]
pub struct FixedSectionHeader {
    pub magic: [u8;4],
    pub block_count: u32,
    pub block_size: u32,
    padding: [u8; 52],
}

#[repr(C)]
pub struct FixedTable<'a, T, const N: usize> {
    vtable: *const (),
    pub entries: [FixedTableEntry<'a, T>; N],
    pub fixed_section: &'a FixedSection<T, N>,
    pub section_entries: &'a [T; N],
    end_of_section: *const u64,
}


impl<'a, T, const N: usize> FixedTable<'a, T, N> {
    pub fn get_table_mut(offset: usize) -> &'static mut FixedTable<'a, T, N> {
        return unsafe { *offset_to_addr::<&'static mut FixedTable<'a, T, N>>(offset) };
    }
    pub fn get_entry(&'a mut self, index: usize) -> &'a mut T {
        if index < N { self.entries[index].entry}
        else { self.entries[0].entry }
    }

}

pub trait FixedDataTable<T, const N: usize> {
    fn get_table() -> &'static mut FixedTable<'static, T, N>;
    fn get_entry(index: usize) -> &'static mut T {
        Self::get_table().get_entry(index)
    }
}



#[repr(C)]
pub struct FlowEntry {
    pub cutscene: [i16; 12],
    pub unk: u8,
    pub monastery_script: i8,
    pub unk_0x1a: u8,
    pub bgm: u8,
    pub chapter_string: u8,
    pub month: u8,
    pub scenario_id: i8,
}

pub struct CalendarEvent {}