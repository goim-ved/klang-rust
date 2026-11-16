pub struct DenormalGuard {
    #[allow(dead_code)]
    prev: usize,
}

impl DenormalGuard {
    #[inline]
    pub fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            #[allow(deprecated)]
            unsafe {
                use core::arch::x86_64{_mm_getcsr, _mm_setcsr};
                let prev = _mm_getcsr();
                _mm_setcsr(prev | 0x8040);
                Self { prev: prev as usize } 
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            unsafe {
                let prev: u64;
                core::arch::asm!("mrs {}, fpcr", out(reg) prev, options(nomeme, nostack, preserves_flags));
                core::arch::asm!("msr fpcr, {}", in(reg) prev | (1 << 24), options(nomem, nostack, preserves_flags));
                Self { prev: prev as usize }
            }
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Self { prev: 0 }
        }
    }
}

impl Drop for DenormalGuard {
    #[inline]
    fn drop(&mut self) {
        #[cfg(target_arch = "x86_64")]
        #[allow(deprecated)]
        unsafe {
            core::arch::x86_64::_mm_setcsr(self.prev as u32);
        }
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("msr fpcr, {}", in(reg) self.prev as u64, options(nomem, nostack, preserves_flags));
        }
    }
}