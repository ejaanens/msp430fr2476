#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430FR2476 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = "Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn ECOMP0();
    fn PORT6();
    fn PORT5();
    fn PORT4();
    fn PORT3();
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn EUSCI_B1();
    fn EUSCI_B0();
    fn EUSCI_A1();
    fn EUSCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ECOMP0 },
    Vector { _handler: PORT6 },
    Vector { _handler: PORT5 },
    Vector { _handler: PORT4 },
    Vector { _handler: PORT3 },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector { _handler: ADC },
    Vector { _handler: EUSCI_B1 },
    Vector { _handler: EUSCI_B0 },
    Vector { _handler: EUSCI_A1 },
    Vector { _handler: EUSCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER0_B1,
    },
    Vector {
        _handler: TIMER0_B0,
    },
    Vector {
        _handler: TIMER3_A1,
    },
    Vector {
        _handler: TIMER3_A0,
    },
    Vector {
        _handler: TIMER2_A1,
    },
    Vector {
        _handler: TIMER2_A0,
    },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "19 - 0xFFCA"]
    ECOMP0 = 19,
    #[doc = "20 - 0xFFCC"]
    PORT6 = 20,
    #[doc = "21 - 0xFFCE"]
    PORT5 = 21,
    #[doc = "22 - 0xFFD0"]
    PORT4 = 22,
    #[doc = "23 - 0xFFD2"]
    PORT3 = 23,
    #[doc = "24 - 0xFFD4"]
    PORT2 = 24,
    #[doc = "25 - 0xFFD6"]
    PORT1 = 25,
    #[doc = "26 - 0xFFD8"]
    ADC = 26,
    #[doc = "27 - 0xFFDA"]
    EUSCI_B1 = 27,
    #[doc = "28 - 0xFFDC"]
    EUSCI_B0 = 28,
    #[doc = "29 - 0xFFDE"]
    EUSCI_A1 = 29,
    #[doc = "30 - 0xFFE0"]
    EUSCI_A0 = 30,
    #[doc = "31 - 0xFFE2"]
    WDT = 31,
    #[doc = "32 - 0xFFE4"]
    RTC = 32,
    #[doc = "33 - 0xFFE6"]
    TIMER0_B1 = 33,
    #[doc = "34 - 0xFFE8"]
    TIMER0_B0 = 34,
    #[doc = "35 - 0xFFEA"]
    TIMER3_A1 = 35,
    #[doc = "36 - 0xFFEC"]
    TIMER3_A0 = 36,
    #[doc = "37 - 0xFFEE"]
    TIMER2_A1 = 37,
    #[doc = "38 - 0xFFF0"]
    TIMER2_A0 = 38,
    #[doc = "39 - 0xFFF2"]
    TIMER1_A1 = 39,
    #[doc = "40 - 0xFFF4"]
    TIMER1_A0 = 40,
    #[doc = "41 - 0xFFF6"]
    TIMER0_A1 = 41,
    #[doc = "42 - 0xFFF8"]
    TIMER0_A0 = 42,
    #[doc = "43 - 0xFFFA"]
    UNMI = 43,
    #[doc = "44 - 0xFFFC"]
    SYSNMI = 44,
}
#[doc = "P1"]
pub struct P1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1 {}
impl P1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p1::RegisterBlock = 0x0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P1 {
    type Target = p1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1").finish()
    }
}
#[doc = "P1"]
pub mod p1;
#[doc = "P2"]
pub struct P2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P2 {}
impl P2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p2::RegisterBlock = 0x0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P2 {
    type Target = p2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2").finish()
    }
}
#[doc = "P2"]
pub mod p2;
#[doc = "P3"]
pub struct P3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P3 {}
impl P3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p3::RegisterBlock = 0x0220 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P3 {
    type Target = p3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3").finish()
    }
}
#[doc = "P3"]
pub mod p3;
#[doc = "P4"]
pub struct P4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P4 {}
impl P4 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p4::RegisterBlock = 0x0220 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p4::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P4 {
    type Target = p4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4").finish()
    }
}
#[doc = "P4"]
pub mod p4;
#[doc = "P5"]
pub struct P5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P5 {}
impl P5 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p5::RegisterBlock = 0x0240 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p5::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P5 {
    type Target = p5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P5").finish()
    }
}
#[doc = "P5"]
pub mod p5;
#[doc = "P6"]
pub struct P6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P6 {}
impl P6 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const p6::RegisterBlock = 0x0240 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p6::RegisterBlock {
        Self::PTR
    }
}
impl Deref for P6 {
    type Target = p6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for P6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P6").finish()
    }
}
#[doc = "P6"]
pub mod p6;
#[doc = "SFR"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sfr::RegisterBlock = 0x0100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SFR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFR").finish()
    }
}
#[doc = "SFR"]
pub mod sfr;
#[doc = "PMM"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pmm::RegisterBlock = 0x0120 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PMM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMM").finish()
    }
}
#[doc = "PMM"]
pub mod pmm;
#[doc = "SYS"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sys::RegisterBlock = 0x0140 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS").finish()
    }
}
#[doc = "SYS"]
pub mod sys;
#[doc = "CS"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cs::RegisterBlock = 0x0180 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS").finish()
    }
}
#[doc = "CS"]
pub mod cs;
#[doc = "FRCTL"]
pub struct FRCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRCTL {}
impl FRCTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const frctl::RegisterBlock = 0x01a0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const frctl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FRCTL {
    type Target = frctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FRCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRCTL").finish()
    }
}
#[doc = "FRCTL"]
pub mod frctl;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crc::RegisterBlock = 0x01c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "WDT_A"]
pub struct WDT_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_A {}
impl WDT_A {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt_a::RegisterBlock = 0x01cc as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt_a::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT_A {
    type Target = wdt_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT_A {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_A").finish()
    }
}
#[doc = "WDT_A"]
pub mod wdt_a;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x0300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "PJ"]
pub struct PJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PJ {}
impl PJ {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pj::RegisterBlock = 0x0320 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pj::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PJ {
    type Target = pj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PJ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PJ").finish()
    }
}
#[doc = "PJ"]
pub mod pj;
#[doc = "TA0"]
pub struct TA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA0 {}
impl TA0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ta0::RegisterBlock = 0x0380 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TA0 {
    type Target = ta0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TA0").finish()
    }
}
#[doc = "TA0"]
pub mod ta0;
#[doc = "TA1"]
pub struct TA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA1 {}
impl TA1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ta1::RegisterBlock = 0x03c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TA1 {
    type Target = ta1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TA1").finish()
    }
}
#[doc = "TA1"]
pub mod ta1;
#[doc = "TA2"]
pub struct TA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA2 {}
impl TA2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ta2::RegisterBlock = 0x0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TA2 {
    type Target = ta2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TA2").finish()
    }
}
#[doc = "TA2"]
pub mod ta2;
#[doc = "TA3"]
pub struct TA3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA3 {}
impl TA3 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ta3::RegisterBlock = 0x0440 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta3::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TA3 {
    type Target = ta3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TA3").finish()
    }
}
#[doc = "TA3"]
pub mod ta3;
#[doc = "TB0"]
pub struct TB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB0 {}
impl TB0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tb0::RegisterBlock = 0x0480 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TB0 {
    type Target = tb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TB0").finish()
    }
}
#[doc = "TB0"]
pub mod tb0;
#[doc = "MPY32"]
pub struct MPY32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY32 {}
impl MPY32 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mpy32::RegisterBlock = 0x04c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy32::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MPY32 {
    type Target = mpy32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MPY32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPY32").finish()
    }
}
#[doc = "MPY32"]
pub mod mpy32;
#[doc = "eUSCI_A0"]
pub struct E_USCI_A0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_A0 {}
impl E_USCI_A0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const e_usci_a0::RegisterBlock = 0x0500 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for E_USCI_A0 {
    type Target = e_usci_a0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for E_USCI_A0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E_USCI_A0").finish()
    }
}
#[doc = "eUSCI_A0"]
pub mod e_usci_a0;
#[doc = "eUSCI_A1"]
pub struct E_USCI_A1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_A1 {}
impl E_USCI_A1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const e_usci_a1::RegisterBlock = 0x0520 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for E_USCI_A1 {
    type Target = e_usci_a1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for E_USCI_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E_USCI_A1").finish()
    }
}
#[doc = "eUSCI_A1"]
pub mod e_usci_a1;
#[doc = "eUSCI_B0"]
pub struct E_USCI_B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_B0 {}
impl E_USCI_B0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const e_usci_b0::RegisterBlock = 0x0540 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for E_USCI_B0 {
    type Target = e_usci_b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for E_USCI_B0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E_USCI_B0").finish()
    }
}
#[doc = "eUSCI_B0"]
pub mod e_usci_b0;
#[doc = "eUSCI_B1"]
pub struct E_USCI_B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_USCI_B1 {}
impl E_USCI_B1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const e_usci_b1::RegisterBlock = 0x0580 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for E_USCI_B1 {
    type Target = e_usci_b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for E_USCI_B1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E_USCI_B1").finish()
    }
}
#[doc = "eUSCI_B1"]
pub mod e_usci_b1;
#[doc = "BKMEM"]
pub struct BKMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BKMEM {}
impl BKMEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bkmem::RegisterBlock = 0x0660 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bkmem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BKMEM {
    type Target = bkmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BKMEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKMEM").finish()
    }
}
#[doc = "BKMEM"]
pub mod bkmem;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x0700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "eCOMP0"]
pub struct E_COMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for E_COMP0 {}
impl E_COMP0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const e_comp0::RegisterBlock = 0x08e0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_comp0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for E_COMP0 {
    type Target = e_comp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for E_COMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E_COMP0").finish()
    }
}
#[doc = "eCOMP0"]
pub mod e_comp0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "P1"]
    pub P1: P1,
    #[doc = "P2"]
    pub P2: P2,
    #[doc = "P3"]
    pub P3: P3,
    #[doc = "P4"]
    pub P4: P4,
    #[doc = "P5"]
    pub P5: P5,
    #[doc = "P6"]
    pub P6: P6,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "FRCTL"]
    pub FRCTL: FRCTL,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "WDT_A"]
    pub WDT_A: WDT_A,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "PJ"]
    pub PJ: PJ,
    #[doc = "TA0"]
    pub TA0: TA0,
    #[doc = "TA1"]
    pub TA1: TA1,
    #[doc = "TA2"]
    pub TA2: TA2,
    #[doc = "TA3"]
    pub TA3: TA3,
    #[doc = "TB0"]
    pub TB0: TB0,
    #[doc = "MPY32"]
    pub MPY32: MPY32,
    #[doc = "E_USCI_A0"]
    pub E_USCI_A0: E_USCI_A0,
    #[doc = "E_USCI_A1"]
    pub E_USCI_A1: E_USCI_A1,
    #[doc = "E_USCI_B0"]
    pub E_USCI_B0: E_USCI_B0,
    #[doc = "E_USCI_B1"]
    pub E_USCI_B1: E_USCI_B1,
    #[doc = "BKMEM"]
    pub BKMEM: BKMEM,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "E_COMP0"]
    pub E_COMP0: E_COMP0,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Self::conjure()
    }
    /// Unchecked version of `Peripherals::take` that does not prevent the Peripheral from being
    /// taken in the future
    #[inline]
    pub unsafe fn conjure() -> Self {
        Peripherals {
            P1: P1 {
                _marker: PhantomData,
            },
            P2: P2 {
                _marker: PhantomData,
            },
            P3: P3 {
                _marker: PhantomData,
            },
            P4: P4 {
                _marker: PhantomData,
            },
            P5: P5 {
                _marker: PhantomData,
            },
            P6: P6 {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            FRCTL: FRCTL {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            WDT_A: WDT_A {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            PJ: PJ {
                _marker: PhantomData,
            },
            TA0: TA0 {
                _marker: PhantomData,
            },
            TA1: TA1 {
                _marker: PhantomData,
            },
            TA2: TA2 {
                _marker: PhantomData,
            },
            TA3: TA3 {
                _marker: PhantomData,
            },
            TB0: TB0 {
                _marker: PhantomData,
            },
            MPY32: MPY32 {
                _marker: PhantomData,
            },
            E_USCI_A0: E_USCI_A0 {
                _marker: PhantomData,
            },
            E_USCI_A1: E_USCI_A1 {
                _marker: PhantomData,
            },
            E_USCI_B0: E_USCI_B0 {
                _marker: PhantomData,
            },
            E_USCI_B1: E_USCI_B1 {
                _marker: PhantomData,
            },
            BKMEM: BKMEM {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            E_COMP0: E_COMP0 {
                _marker: PhantomData,
            },
        }
    }
}
