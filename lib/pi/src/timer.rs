use crate::common::IO_BASE;

use volatile::prelude::*;
use volatile::{ReadVolatile, Volatile};
use core::time::Duration;
/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4],
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns Duration.
    /// `CLO` and `CHI` together can represent the number of elapsed microseconds.
    pub fn read(&self) -> Duration {
        Duration::from_micros(((self.registers.CHI.read() as u64) << 32) + (self.registers.CLO.read() as u64))
    }
}

/// Returns the current time
pub fn current_time() -> Duration {
    let timer = Timer::new();
    timer.read()
}

/// Spins until `t` have passed.
pub fn spin_sleep(t: Duration) {
    let timer = Timer::new();
    timer.registers.COMPARE[0].write(timer.read().checked_add(t).unwrap().as_millis() as u32);
    while 0u32 == (timer.registers.CS.read() & 1u32) {}
    timer.registers.CS.write(0u32);
}

