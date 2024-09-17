use skyline::from_offset;
use skyline::libc::c_void;

#[from_offset(0x540b30)]
pub fn kt_malloc(size: u32) -> *const c_void;

#[from_offset(0x5BAB80)]
pub fn kt_aligned_malloc(size: u64, align: u64) -> *const c_void;

#[from_offset(0x540b50)]
pub fn kt_free(pointer: *mut c_void) -> *const c_void;

#[from_offset(0x540b70)]
pub fn kt_realloc(pointer: *mut c_void, size: u64) -> *const c_void;

#[from_offset(0x540b90)]
pub fn kt_calloc(t1: u32, t2: u32) -> *const c_void;