
use bit_field::BitField;

pub const CLOCK_REG_BASE: usize = 0x1000_0000;

// ANOTHER FIXME GINA // We should check PRCI_PLLS to verify the presence
// of each PLL before assuming it's there

#[allow(dead_code)]
pub fn get_core_frequency() -> u64 {
    // COREPLL is configured in software by setting the corepllcfg0 PRCI control register.
    // The input reference frequency for COREPLL is 26 MHz.

    // The minimum supported post-divide frequency is 7 MHz; thus, valid settings are
    // 0, 1, and 2.
    // if divr > 2 { divr = 2; }  // in case we find it at 3, we compute as if it were 3
    //                            // but we never set it to 3.

    // The maximum value of DIVQ is 6, and the valid output
    // range is 20 to 2400 MHz
    // if divq > 6 { divq = 6; }  // in case we find it at 7, we compute as if it were 7
    //                            // but we never set it to 7.

    let core_pllcfg_register = core_pllcfg::get_register();
    let divr = core_pllcfg_register.get_bits(0..=5) as u32;
    let divf = core_pllcfg_register.get_bits(6..=14) as u32;
    let divq = core_pllcfg_register.get_bits(15..=17) as u32;

    // There is a reference frequency divider before the PLL loop. The divider value is
    // equal to the PRCI PLL configuration register field divr + 1.
    let pre_divide = divr + 1;

    // The valid PLL VCO range is 2400 MHz to 4800 MHz.  The VCO feedback divider
    // value is equal to 2 * (divf + 1).
    let pll_loop = 2 * (divf + 1);

    // There is a further output divider after the PLL loop. The divider value is
    // equal to 2**divq.
    let post_divide = 2_u32.pow(divq);

    ( (26_000_000 / pre_divide) * pll_loop / post_divide ) as u64
}

// Try to set the core frequency to the target value.
// The actual value set will be returned (as close as we could get it)
#[allow(dead_code)]
pub fn set_core_frequency(target: u64) -> u64
{
    if let Some((divr, divf, divq)) = compute_pll_params(target) {

        // If we are using corepll (as expected), divert to dvfscorepll
        if core_clk_sel_reg::using_coreclk_not_hfclk() {
            // Switch frequency of dvfs core pll
            unsafe {
                dvfs_core_pllcfg::put_pll_params(divr as i32, divf as i32, divq as i32)
            }

            // Wait for PLL to lock
            while ! dvfs_core_pllcfg::get_plllock() {
                super::pause();
            }

            // Switch to it
            corepllsel::use_dvfscorepll_not_corepll();
        }

        // Switch frequency of core pll
        unsafe {
            core_pllcfg::put_pll_params(divr as i32, divf as i32, divq as i32)
        }

        // Wait for PLL to lock
        while ! core_pllcfg::get_plllock() {
            super::pause();
        }

        // Switch to it
        corepllsel::use_corepll_not_dvfscorepll();
    }

    get_core_frequency()
}

fn compute_pll_params(mut target_freq: u64) -> Option<(u32, u32, u32)> {
    // Put target_freq values in range
    if target_freq <= 37_500_000 {
        target_freq = 37_500_000;
    } else if target_freq >= 2_400_000_000 {
        target_freq = 2_400_000_000;
    }

    // Determine divq
    let mut divq = 0;
    while target_freq < 2_400_000_000 / 2_u64.pow(divq) {
        divq += 1;
    }

    // Determine stage2 output
    let stage2 = target_freq * 2_u64.pow(divq);

    // Place to store best-so-far settings
    struct Params {
        divr: u32,
        divf: u32,
        err: u64
    }
    let mut best: Option<Params> = None;

    // Setup closures
    let _freq = |divr: u32, divf: u32, divq: u32| -> u64 {
        ((26_000_000 / (divr + 1)) * (2 * (divf + 1)) / 2_u32.pow(divq)) as u64
    };
    let _dist = |a: u64, b: u64| -> u64 {
        if a > b { a - b } else { b - a }
    };
    let mut _contend = |divr: u32, divf: u32| {
        let freq = _freq(divr, divf, divq);
        let err = _dist(freq, target_freq);
        if best.is_none() {
            best = Some(Params { divr, divf, err })
        } else {
            if err < best.as_ref().unwrap().err {
                best = Some(Params { divr, divf, err })
            }
        }
    };

    // Try all divr settings
    for divr in 0..=2 {
        let stage1: u64 = 26_000_000 / (divr + 1) as u64;
        let divf_plus1 = ((stage2 / stage1) as u32) / 2;
        _contend(divr, divf_plus1);
        if divf_plus1 > 0 { _contend(divr, divf_plus1 - 1); }
    }

    if let Some(b) = best {
        Some((b.divr, b.divf, divq))
    } else {
        None
    }
}

macro_rules! impl_pllcfg {
    ($reg:ident, $offset:expr) => (
        #[allow(dead_code)]
        pub mod $reg {
            use crate::register::AtomicRegisterI32RWSpinlock;
            use super::CLOCK_REG_BASE;

            #[inline(always)]
            unsafe fn register() -> AtomicRegisterI32RWSpinlock {
                AtomicRegisterI32RWSpinlock::new(CLOCK_REG_BASE + $offset)
            }

            pub fn get_register() -> i32 {
                unsafe { self::register().fetch() }
            }

            #[inline(always)]
            pub fn get_pllr() -> i32 {
                unsafe { self::register().get_bits(0..=5) }
            }

            #[inline(always)]
            pub fn get_pllf() -> i32 {
                unsafe { self::register().get_bits(6..=14) }
            }

            #[inline(always)]
            pub fn get_pllq() -> i32 {
                unsafe { self::register().get_bits(15..=17) }
            }

            #[inline(always)]
            pub unsafe fn put_pll_params(divr: i32, divf: i32, divq: i32) {
                use bit_field::BitField;
                let mut w: i32 = 0;
                w.set_bits(0..=5, divr);
                w.set_bits(6..=14, divf);
                w.set_bits(15..=17, divq);
                self::register().put_bits(0..=17, w);
            }

            #[inline(always)]
            pub fn get_pllrange() -> i32 {
                unsafe { self::register().get_bits(18..=20) }
            }

            #[inline(always)]
            pub unsafe fn put_pllrange(v: i32) {
                self::register().put_bits(18..=20, v)
            }

            #[inline(always)]
            pub fn get_pllbypass() -> bool {
                unsafe { self::register().get_bit(24) }
            }

            #[inline(always)]
            pub unsafe fn set_pllbypass() {
                self::register().set_bit(24);
            }

            #[inline(always)]
            pub unsafe fn clear_pllbypass() {
                self::register().clear_bit(24);
            }

            #[inline(always)]
            pub fn get_pllfsebypass() -> bool {
                unsafe { self::register().get_bit(25) }
            }

            #[inline(always)]
            pub unsafe fn set_pllsfebypass() {
                self::register().set_bit(25);
            }

            #[inline(always)]
            pub unsafe fn clear_pllsfebypass() {
                self::register().clear_bit(25);
            }

            #[inline(always)]
            pub fn get_plllock() -> bool {
                unsafe { self::register().get_bit(31) }
            }
        }
    );
}

macro_rules! impl_plloutdiv {
    ($name:ident, $offset:expr) => (
        #[allow(dead_code)]
        pub mod $name {
            use crate::register::AtomicRegisterI32RW;
            use super::CLOCK_REG_BASE;

            #[inline(always)]
            unsafe fn register() -> AtomicRegisterI32RW {
                AtomicRegisterI32RW::new(CLOCK_REG_BASE + $offset)
            }

            #[inline(always)]
            pub fn get_pllcke() -> bool {
                unsafe { self::register().get_bit(31) }
            }

            #[inline(always)]
            pub unsafe fn set_pllcke() {
                self::register().set_bit(31);
            }

            #[inline(always)]
            pub unsafe fn clear_pllcke() {
                self::register().clear_bit(31);
            }
        }
    );
}

#[allow(dead_code)]
pub mod hfxosccfg {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x00)
    }

    /// Is HFX OSC enabled?  This should be enabled at reset.
    #[inline(always)]
    pub fn get_hfxoscen() -> bool {
        unsafe { self::register().get_bit(30) }
    }

    #[inline(always)]
    pub unsafe fn set_hfxoscen() {
        self::register().set_bit(30);
    }

    #[inline(always)]
    pub unsafe fn clear_hfxoscen() {
        self::register().clear_bit(30);
    }

    #[inline(always)]
    pub fn get_hfxoscrdy() -> bool {
        unsafe { self::register().get_bit(31) }
    }
}

impl_pllcfg!(core_pllcfg, 0x04);
// core_plloutdiv at 0x08 is wholly reserved

impl_pllcfg!(dvfs_core_pllcfg, 0x38);
impl_plloutdiv!(dvfs_core_plloutdiv, 0x3C);

impl_pllcfg!(hfpclk_pllcfg, 0x50);
impl_plloutdiv!(hfpclk_plloutdiv, 0x54);

#[allow(dead_code)]
pub mod hfpclk_div_reg {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x5C)
    }

    #[inline(always)]
    pub fn get_hfpclk_div_reg() -> i32 {
        unsafe { self::register().fetch() }
    }

    #[inline(always)]
    pub unsafe fn put_hfpclk_div_reg(v: i32) {
        self::register().store(v);
    }
}

impl_pllcfg!(ddr_pllcfg, 0x0C);
impl_plloutdiv!(ddr_plloutdiv, 0x10);

impl_pllcfg!(gemgxl_pllcfg, 0x1C);
impl_plloutdiv!(gemgcl_plloutdiv, 0x20);

#[allow(dead_code)]
pub mod core_clk_sel_reg {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x24)
    }

    #[inline(always)]
    pub fn use_coreclk_not_hfclk() {
        unsafe { self::register().store(0); }
    }

    #[inline(always)]
    pub fn use_hfclk_not_coreclk() {
        unsafe { self::register().store(1); }
    }

    #[inline(always)]
    pub fn using_coreclk_not_hfclk() -> bool {
        unsafe { self::register().fetch() == 0 }
    }
}

#[allow(dead_code)]
pub mod devices_reset_n {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x28)
    }

    #[inline(always)]
    pub fn get_ddrctrl_reset_n() -> bool {
        unsafe { self::register().get_bit(0) }
    }

    #[inline(always)]
    pub unsafe fn set_ddrctrl_reset_n() {
        self::register().set_bit(0);
    }

    #[inline(always)]
    pub unsafe fn clear_ddrctrl_reset_n() {
        self::register().clear_bit(0);
    }

    #[inline(always)]
    pub fn get_ddraxi_reset_n() -> bool {
        unsafe { self::register().get_bit(1) }
    }

    #[inline(always)]
    pub unsafe fn set_ddraxi_reset_n() {
        self::register().set_bit(1);
    }

    #[inline(always)]
    pub unsafe fn clear_ddraxi_reset_n() {
        self::register().clear_bit(1);
    }

    #[inline(always)]
    pub fn get_ddrahb_reset_n() -> bool {
        unsafe { self::register().get_bit(2) }
    }

    #[inline(always)]
    pub unsafe fn set_ddrahb_reset_n() {
        self::register().set_bit(2);
    }

    #[inline(always)]
    pub unsafe fn clear_ddrahb_reset_n() {
        self::register().clear_bit(2);
    }

    #[inline(always)]
    pub fn get_ddrphy_reset_n() -> bool {
        unsafe { self::register().get_bit(3) }
    }

    #[inline(always)]
    pub unsafe fn set_ddrphy_reset_n() {
        self::register().set_bit(3);
    }

    #[inline(always)]
    pub unsafe fn clear_ddrphy_reset_n() {
        self::register().clear_bit(3);
    }

    #[inline(always)]
    pub fn get_pcieaux_reset_n() -> bool {
        unsafe { self::register().get_bit(4) }
    }

    #[inline(always)]
    pub unsafe fn set_pcieaux_reset_n() {
        self::register().set_bit(4);
    }

    #[inline(always)]
    pub unsafe fn clear_pcieaux_reset_n() {
        self::register().clear_bit(4);
    }

    #[inline(always)]
    pub fn get_gemgxl_reset_n() -> bool {
        unsafe { self::register().get_bit(5) }
    }

    #[inline(always)]
    pub unsafe fn set_gemgxl_reset_n() {
        self::register().set_bit(5);
    }

    #[inline(always)]
    pub unsafe fn clear_gemgxl_reset_n() {
        self::register().clear_bit(5);
    }
}

#[allow(dead_code)]
pub mod clk_mux_status {
    use crate::register::AtomicRegisterI32RO;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RO {
        AtomicRegisterI32RO::new(CLOCK_REG_BASE + 0x2C)
    }

    #[inline(always)]
    pub fn get_coreclkpllsel() -> bool {
        unsafe { self::register().get_bit(0) }
    }

    #[inline(always)]
    pub fn get_tlclksel() -> bool {
        unsafe { self::register().get_bit(1) }
    }

    #[inline(always)]
    pub fn get_rtcxsel() -> bool {
        unsafe { self::register().get_bit(2) }
    }

    #[inline(always)]
    pub fn get_ddrctrlclksel() -> bool {
        unsafe { self::register().get_bit(3) }
    }

    #[inline(always)]
    pub fn get_ddrphyclksel() -> bool {
        unsafe { self::register().get_bit(4) }
    }

    #[inline(always)]
    pub fn get_reserved0() -> bool {
        unsafe { self::register().get_bit(5) }
    }

    #[inline(always)]
    pub fn get_gemgxlclksel() -> bool {
        unsafe { self::register().get_bit(6) }
    }

    #[inline(always)]
    pub fn get_mainmemclksel() -> bool {
        unsafe { self::register().get_bit(7) }
    }
}

#[allow(dead_code)]
pub mod corepllsel {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x40)
    }

    #[inline(always)]
    pub fn use_corepll_not_dvfscorepll() {
        unsafe { self::register().store(0); }
    }

    #[inline(always)]
    pub fn use_dvfscorepll_not_corepll() {
        unsafe { self::register().store(1); }
    }

    #[inline(always)]
    pub fn using_corepll_not_dvfscorepll() -> bool {
        unsafe {self::register().fetch() == 0 }
    }
}

#[allow(dead_code)]
pub mod hfpclkpllsel {
    use crate::register::AtomicRegisterI32RW;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RW {
        AtomicRegisterI32RW::new(CLOCK_REG_BASE + 0x58)
    }

    #[inline(always)]
    pub fn use_hfpclkpll_not_hfclk() {
        unsafe { self::register().store(0); }
    }

    #[inline(always)]
    pub fn use_hfclk_not_hfpclkpll() {
        unsafe { self::register().store(1); }
    }

    #[inline(always)]
    pub fn using_hfpclkpll_not_hfclk() -> bool {
        unsafe {self::register().fetch() == 0 }
    }
}

#[allow(dead_code)]
pub mod prci_plls {
    use crate::register::AtomicRegisterI32RO;
    use super::CLOCK_REG_BASE;

    #[inline(always)]
    unsafe fn register() -> AtomicRegisterI32RO {
        AtomicRegisterI32RO::new(CLOCK_REG_BASE + 0xE0)
    }

    #[inline(always)]
    pub fn get_cltxpll() -> bool {
        unsafe { self::register().get_bit(0) }
    }

    #[inline(always)]
    pub fn get_gemgxlpll() -> bool {
        unsafe { self::register().get_bit(1) }
    }

    #[inline(always)]
    pub fn get_ddrpll() -> bool {
        unsafe { self::register().get_bit(2) }
    }

    #[inline(always)]
    pub fn get_hfpclkpll() -> bool {
        unsafe { self::register().get_bit(3) }
    }

    #[inline(always)]
    pub fn get_dvfscorepll() -> bool {
        unsafe { self::register().get_bit(4) }
    }

    #[inline(always)]
    pub fn get_corepll() -> bool {
        unsafe { self::register().get_bit(5) }
    }
}
