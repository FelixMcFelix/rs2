// FIXME: magic number
pub type Tlb = [TlbLine; 48];

/// Defined on pp 123--124.
pub struct TlbLine {
	mask: u32,
	virtual_page_number: u32,
	global: bool,
	asid: u8,

	scratchpad: bool,
	odd_page_frame_number: u32,
	odd_cache_mode: u8,
	odd_dirty: bool,
	odd_valid: bool,

	even_page_frame_number: u32,
	even_cache_mode: u8,
	even_dirty: bool,
	even_valid: bool,
}