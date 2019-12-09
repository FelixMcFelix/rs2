/// Starting address of useg/suseg/kuseg.
///
/// Useg is the region from `USEG_START` to `KSEG0_START - 1`,
/// i.e., with 3 MSBs set to `000`.
/// Addresses must be mapped to physical space by the TLB.
///
/// User/supervisor/kernel access.
pub const USEG_START: usize = 0x0000_0000;

/// Starting address of kseg0.
///
/// Kseg0 is the region from `KSEG0_START` to `KSEG1_START - 1`.
/// This maps to the physical address space `0x0000_0000` to `0x1FFF_FFFF`,
/// i.e., with 3 MSBs set to `100`,
/// which contains the BIOS and boot code.
///
/// Kernel access only. Caching controlled by k0 field of Config register (COP0?).
pub const KSEG0_START: usize = 0x8000_0000;

/// Starting address of kseg1.
///
/// Kseg1 is the region from `KSEG1_START` to `SSEG_START - 1`.
/// This maps to the physical address space `0x0000_0000` to `0x1FFF_FFFF`,
/// i.e., with 3 MSBs set to `101`,
/// which contains the BIOS and boot code.
///
/// Kernel access only. Caching disabled.
pub const KSEG1_START: usize = 0xA000_0000;

/// Starting address of ksseg/sseg.
///
/// Sseg is the region from `SSEG1_START` to `KSEG3_START - 1`,
/// i.e., with 3 MSBs set to `110`.
/// Addresses must be mapped to physical space by the TLB.
///
/// Supervisor/kernel access.
pub const SSEG_START: usize = 0xC000_0000;

/// Starting address of kseg3.
///
/// Kseg3 is the region from `KSEG3_START` to `0xFFFF_FFFF`,
/// i.e., with 3 MSBs set to `111`.
/// Addresses must be mapped to physical space by the TLB.
///
/// Kernel access only.
pub const KSEG3_START: usize = 0xE000_0000;