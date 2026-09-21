#![no_std]

pub mod dac;

use embassy_rp::{
    bind_interrupts,
    dma,
    gpio::{Level, Output},
    i2c,
    peripherals::{DMA_CH0, I2C0},
    pio_programs::{
        clk::{PioClk, PioClkProgram},
        i2s::{PioI2sOut, PioI2sOutProgram},
    },
};

use xpanse_api::{
    with_pio,
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    interfaces::buttons::{
        pin_button,
        A,
        B,
    },
    metadata::{
        ModuleDetectResistor,
        ModuleID,
        ModuleSlot,
    },
    registry::Registry,
};

const SAMPLE_RATE: u32 = 48_000;
const BIT_DEPTH: u32 = 16;
const MCLK_FREQUENCY: u32 = 12_288_000;

bind_interrupts!(struct I2cIrqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

bind_interrupts!(struct AudioDmaIrqs {
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});


pub struct SpeakerDriver;

impl DriverMeta for SpeakerDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K,
        md1: ModuleDetectResistor::R68K,
    };
}


impl<G> Driver<G> for SpeakerDriver
where
    G: BankPins,
    G::GPIO0: embassy_rp::i2c::SclPin<I2C0>,
    G::GPIO1: embassy_rp::i2c::SdaPin<I2C0>,
{
    async fn create(
        bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        buses: &mut BusAllocator,
    ) -> Result<(), DriverError> {


        let mut reset = Output::new(
            bank.gpio6,
            Level::Low,
        );

        embassy_time::Timer::after_millis(5).await;

        reset.set_high();

        embassy_time::Timer::after_millis(10).await;

        core::mem::forget(reset);

        let mut i2c_bus = buses
            .create_i2c_hardware::<I2C0, _>(
                bank.gpio0,
                bank.gpio1,
                I2cIrqs,
                i2c::Config::default(),
            )
            .map_err(|_| DriverError::InitFailed)?;

        dac::reset(&mut i2c_bus).await?;

        dac::configure(&mut i2c_bus).await?;

        registry.register(
            slot,
            Self::ID,
            pin_button::<A>(
                bank.gpio7.into(),
            ),
        );

        registry.register(
            slot,
            Self::ID,
            pin_button::<B>(
                bank.gpio8.into(),
            ),
        );



        let dma = buses
            .request_dma::<DMA_CH0>()
            .map_err(|_| DriverError::InitFailed)?;


        let mclk_pio = buses
            .request_pio(&[
                &bank.gpio5,
            ])
            .ok_or(DriverError::InitFailed)?;

        with_pio!(
            mclk_pio,
            mclk_common,
            mclk_sm,
            {
                let mclk_program = PioClkProgram::new(
                    &mut *mclk_common,
                );

                let mut mclk = PioClk::new(
                    &mut *mclk_common,
                    mclk_sm,
                    bank.gpio5,
                    &mclk_program,
                    MCLK_FREQUENCY,
                );

                mclk.start();

                core::mem::forget(mclk);
            }
        );


        let i2s_pio = buses
            .request_pio(&[
                &bank.gpio2,
                &bank.gpio3,
                &bank.gpio4,
            ])
            .ok_or(DriverError::InitFailed)?;

        with_pio!(
            i2s_pio,
            i2s_common,
            i2s_sm,
            {
                let i2s_program = PioI2sOutProgram::new(
                    &mut *i2s_common,
                );

                let mut i2s = PioI2sOut::new(
                    &mut *i2s_common,
                    i2s_sm,
                    dma,
                    AudioDmaIrqs,
                    bank.gpio3, 
                    bank.gpio2, 
                    bank.gpio4, 
                    SAMPLE_RATE,
                    BIT_DEPTH,
                    &i2s_program,
                );

                i2s.start();

                registry.register(
                    slot,
                    Self::ID,
                    i2s,
                );
            }
        );

        Ok(())
    }
}