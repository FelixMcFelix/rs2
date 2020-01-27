/// Starting virtual address of useg/suseg/kuseg.
///
/// Useg is the region from `USEG_START` to `KSEG0_START - 1`,
/// i.e., with 3 MSBs set to `000`.
/// Addresses must be mapped to physical space by the TLB.
///
/// User/supervisor/kernel access.
pub const USEG_START: u32 = 0x0000_0000;

pub const SPRAM_START: u32 = 0x7000_0000;

pub const USEG_END: u32 = KSEG0_START - 1;

/// Starting virtual address of kseg0.
///
/// Kseg0 is the region from `KSEG0_START` to `KSEG1_START - 1`.
/// This maps to the physical address space `0x0000_0000` to `0x1FFF_FFFF`,
/// i.e., with 3 MSBs set to `100`,
/// which contains the BIOS and boot code.
///
/// Kernel access only. Caching controlled by k0 field of Config register (COP0?).
pub const KSEG0_START: u32 = 0x8000_0000;

pub const KSEG0_END: u32 = KSEG1_START - 1;

/// Starting virtual address of kseg1.
///
/// Kseg1 is the region from `KSEG1_START` to `SSEG_START - 1`.
/// This maps to the physical address space `0x0000_0000` to `0x1FFF_FFFF`,
/// i.e., with 3 MSBs set to `101`,
/// which contains the BIOS and boot code.
///
/// Kernel access only. Caching disabled.
pub const KSEG1_START: u32 = 0xA000_0000;

pub const KSEG1_END: u32 = SSEG_START - 1;

/// Starting virtual address of ksseg/sseg.
///
/// Sseg is the region from `SSEG1_START` to `KSEG3_START - 1`,
/// i.e., with 3 MSBs set to `110`.
/// Addresses must be mapped to physical space by the TLB.
///
/// Supervisor/kernel access.
pub const SSEG_START: u32 = 0xC000_0000;

pub const SSEG_END: u32 = KSEG3_START - 1;

/// Starting virtual address of kseg3.
///
/// Kseg3 is the region from `KSEG3_START` to `0xFFFF_FFFF`,
/// i.e., with 3 MSBs set to `111`.
/// Addresses must be mapped to physical space by the TLB.
///
/// Kernel access only.
pub const KSEG3_START: u32 = 0xE000_0000;

pub const KSEG3_END: u32 = 0xFFFF_FFFF;

// These DMA addresses are courtesy of https://psi-rockin.github.io/ps2tek/.
pub const IO_REGISTERS_PHYSICAL: u32 = 0x1000_0000;

pub const VU0_CODE_PHYSICAL: u32 = 0x1100_0000;
pub const VU0_DATA_PHYSICAL: u32 = 0x1100_4000;
pub const VU1_CODE_PHYSICAL: u32 = 0x1100_8000;
pub const VU1_DATA_PHYSICAL: u32 = 0x1100_C000;

pub const GS_PRIV_REGISTERS_PHYSICAL: u32 = 0x1200_0000;

pub const IOP_RAM_PHYSICAL: u32 = 0x1C00_0000;

pub const BIOS_PHYSICAL: u32 = 0x1FC0_0000;
pub const BIOS_START: u32 = BIOS_PHYSICAL + KSEG1_START;

/// Amount of physical memory in the PS2: 32 MiB.
pub const PHYSICAL_MEMORY_SIZE: usize = 32 * (1 << (10 * 2));