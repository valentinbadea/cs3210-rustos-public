/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two(){
        addr - (addr % align)
    }else{
        panic!("Not power of 2")
    }
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        if addr % align != 0 {
            (addr / align + 1).checked_mul(align).unwrap()
        } else {
            addr
        }
    } else {
        panic!("Not power of 2")
    }
}
