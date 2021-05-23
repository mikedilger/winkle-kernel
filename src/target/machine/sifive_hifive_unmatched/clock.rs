
use bit_field::BitField;

pub const CLOCK_REG_BASE: usize = 0x1000_0000;

// note for future: The u32 can only hold up to about 4.29 GHz.
// This does not lock, and you might get an invalid value if the frequency is being changed
// while this function is running.
#[allow(dead_code)]
pub fn get_core_frequency() -> u32 {

    // COREPLL is configured in software by setting the corepllcfg0 PRCI control register.
    // The input reference frequency for COREPLL is 26 MHz.
    let mut freq = 26_000_000; // hfclk

    let core_pllcfg_register = core_pllcfg::get_register();
    let divr = core_pllcfg_register.get_bits(0..=5) as u32;
    let divf = core_pllcfg_register.get_bits(6..=14) as u32;
    let divq = core_pllcfg_register.get_bits(15..=17) as u32;

    // pre-divide
    {
        // The minimum supported post-divide frequency is 7 MHz; thus, valid settings are
        // 0, 1, and 2.
        // if divr > 2 { divr = 2; }

        // There is a reference frequency divider before the PLL loop. The divider value is
        // equal to the PRCI PLL configuration register field divr + 1.
        let pre_divide = divr + 1;

        freq = freq / pre_divide;
    }

    // pll loop
    {
        // The valid PLL VCO range is 2400 MHz to 4800 MHz.  The VCO feedback divider
        // value is equal to 2 * (divf + 1).
        // NOTE: this actually multiplies the frequency

        freq = freq * (2 * (divf as u32 + 1));
    }

    // output divider
    {
        // The maximum value of DIVQ is 6, and the valid output
        // range is 20 to 2400 MHz
        // if divq > 6 { divq = 6; }

        // There is a further output divider after the PLL loop. The divider value is
        // equal to 2**divq.
        let output_divide = 2_u32.pow(divq as u32);

        freq = freq / output_divide;
    }

    freq
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
            pub unsafe fn put_pllr(v: i32) {
                self::register().put_bits(0..=5, v);
            }

            #[inline(always)]
            pub fn get_pllf() -> i32 {
                unsafe { self::register().get_bits(6..=14) }
            }

            #[inline(always)]
            pub unsafe fn put_pllf(v: i32) {
                self::register().put_bits(6..=14, v);
            }

            #[inline(always)]
            pub fn get_pllq() -> i32 {
                unsafe { self::register().get_bits(15..=17) }
            }

            #[inline(always)]
            pub unsafe fn put_pllq(v: i32) {
                self::register().put_bits(15..=17, v);
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
// core_plloutdiv at 0x8 is wholly reserved

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

impl_pllcfg!(ddr_pllcfg, 0xC);
impl_plloutdiv!(ddr_plloutdiv, 0x10);

impl_pllcfg!(gemgxl_pllcfg, 0x1C);
impl_plloutdiv!(gemgcl_plloutdiv, 0x20);

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
