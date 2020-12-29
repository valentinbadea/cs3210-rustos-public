/// A raw `ATAG` as laid out in memory.
#[repr(C)]
pub struct Atag {
    pub dwords: u32,
    pub tag: u32,
    pub kind: Kind,
}

impl Atag {
    pub const NONE: u32 = 0x0000_0000;
    pub const CORE: u32 = 0x5441_0001;
    pub const MEM: u32 = 0x5441_0002;
    pub const VIDEOTEXT: u32 = 0x5441_0003;
    pub const RAMDISK: u32 = 0x5441_0004;
    pub const INITRD2: u32 = 0x5442_0005;
    pub const SERIAL: u32 = 0x5441_0006;
    pub const REVISION: u32 = 0x5441_0007;
    pub const VIDEOLFB: u32 = 0x5441_0008;
    pub const CMDLINE: u32 = 0x5441_0009;

    /// FIXME: Returns the ATAG following `self`, if there is one.
    pub fn next(&self) -> Option<&Atag> {
        if self.dwords > 1u32 {
            let ptr = self as *const Atag as *const u32;
            unsafe { (ptr.add(self.dwords as usize) as *const Atag).as_ref() }
        } else {
            None
        }
    }
}

/// The possible variant of an ATAG.
#[repr(C)]
pub union Kind {
    pub core: Core,
    pub mem: Mem,
    pub cmd: Cmd,
}

/// A `CORE` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Core {
    pub flags: u32,
    pub page_size: u32,
    pub root_dev: u32,
}

/// A `MEM` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mem {
    pub size: u32,
    pub start: u32,
}

/// A `CMDLINE` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cmd {
    /// The first byte of the command line string.
    pub cmd: u8,
}
