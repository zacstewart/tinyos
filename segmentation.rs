pub const PL0: u8 = 0x0;
pub const CODE_RX: u8 = 0xA;
pub const CODE_RW: u8 = 0x2;
const SEGMENT_BASE: u32 = 0;
const SEGMENT_LIMIT: u32 = 0xFFFF;

#[repr(packed)]
pub struct GlobalDescriptorTable {
    pub size: u16,
    pub offset: *const GlobalDescriptorTableEntry
}

#[repr(packed)]
pub struct GlobalDescriptorTableEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_high_and_flags: u8,
    base_high: u8
}

impl GlobalDescriptorTableEntry {
    pub fn new(pl: u8, descriptor_type: u8) -> GlobalDescriptorTableEntry {
        GlobalDescriptorTableEntry {
            base_low: (SEGMENT_BASE & 0xFFFF) as u16,
            base_mid: ((SEGMENT_BASE >> 16) as u8) & 0xFF,
            base_high: ((SEGMENT_BASE >> 24) as u8) & 0xFF,
            limit_low: (SEGMENT_LIMIT as u16) & 0xFFFF,
            access: (0x01 << 7) | ((pl & 0x03) << 5) | (0x01 << 4) | (descriptor_type & 0x0F),
            limit_high_and_flags: (0x01 << 7) | (0x01 << 6) | 0x0F
        }
    }
}

extern {
    fn _gdt_load_and_set(gdt: *const GlobalDescriptorTable);
}

pub fn gdt_load_and_set(gdt: *const GlobalDescriptorTable) {
    unsafe { _gdt_load_and_set(gdt) }
}
