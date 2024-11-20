

// #[inline(always)]
// pub(crate) fn enable_interrupts() {
//     #[cfg(target_arch = "aarch64")]
//     unsafe { asm!("cpsie i", options(preserves_flags, nostack)) };

//     #[cfg(target_arch = "x86_64")]
//     unsafe { asm!("sti", options(preserves_flags, nostack)) };
// }

#[inline(always)]
pub(crate) fn enable() {
    #[cfg(target_arch = "aarch64")]
    aarch64::irq::enable();

    #[cfg(target_arch = "x86_64")]
    x86_64::instructions::interrupts::enable();
}