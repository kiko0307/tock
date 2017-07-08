#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone)]
pub enum NvicIdx {
    POWER_CLOCK = 0,
    RADIO = 1,
    UART0 = 2,
    SPI0_TWI0 = 3,
    SPI1_TWI1 = 4,
    NFCT = 5,
    GPIOTE = 6,
    ADC = 7,
    TIMER0 = 8,
    TIMER1 = 9,
    TIMER2 = 10,
    RTC0 = 11,
    TEMP = 12,
    RNG = 13,
    ECB = 14,
    CCM_AAR = 15,
    WDT = 16,
    RTC1 = 17,
    QDEC = 18,
    LPCOMP = 19,
    SWI0 = 20,
    SWI1 = 21,
    SWI2 = 22,
    SWI3 = 23,
    SWI4 = 24,
    SWI5 = 25,
    TIMER3 = 26,
    TIMER4 = 27,
    PWM0 = 28,
    PDM = 29,
    MWU = 32,
    PWM1 = 33,
    PWM2 = 34,
    SPIM2_SPIS2_SPI2 = 35,
    RTC2 = 36,
    I2S = 37,
    FPU = 38,
}
