use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;

use xpanse_api::driver::DriverError;

const DAC_ADDRESS: u8 = 0x18;

async fn write_register<I>(
    i2c: &mut I,
    register: u8,
    value: u8,
) -> Result<(), DriverError>
where
    I: I2c,
{
    i2c.write(
        DAC_ADDRESS,
        &[register, value],
    )
    .await
    .map_err(|_| DriverError::InitFailed)
}

pub async fn reset<I>(
    i2c: &mut I,
) -> Result<(), DriverError>
where
    I: I2c,
{
    write_register(i2c, 0x00, 0x00).await?;

    write_register(i2c, 0x01, 0x01).await?;

    Timer::after_millis(10).await;

    Ok(())
}

pub async fn configure<I>(
    i2c: &mut I,
) -> Result<(), DriverError>
where
    I: I2c,
{
    
    write_register(i2c, 0x00, 0x00).await?;
    write_register(i2c, 0x04, 0x00).await?;
    // NDAC = 1, powered
    write_register(i2c, 0x0B, 0x81).await?;
    // MDAC = 2, powered
    write_register(i2c, 0x0C, 0x82).await?;
    // DOSR = 128
    write_register(i2c, 0x0D, 0x00).await?;
    write_register(i2c, 0x0E, 0x80).await?;
    write_register(i2c, 0x1B, 0x00).await?;
    write_register(i2c, 0x1C, 0x00).await?;
    write_register(i2c, 0x3C, 0x0B).await?;
    write_register(i2c, 0x00, 0x08).await?;
    write_register(i2c, 0x01, 0x04).await?;
    write_register(i2c, 0x00, 0x00).await?;
    write_register(i2c, 0x74, 0x00).await?;
    write_register(i2c, 0x00, 0x01).await?;
    write_register(i2c, 0x1F, 0x04).await?;
    write_register(i2c, 0x21, 0x4E).await?;
    write_register(i2c, 0x23, 0x44).await?;
    write_register(i2c, 0x28, 0x06).await?;
    write_register(i2c, 0x29, 0x06).await?;
    write_register(i2c, 0x1F, 0xC2).await?;
    // HPL analog volume = -9 dB
    write_register(i2c, 0x24, 0x92).await?;
    // HPR analog volume = -9 dB
    write_register(i2c, 0x25, 0x92).await?;
    write_register(i2c, 0x00, 0x00).await?;
    write_register(i2c, 0x3F, 0xD4).await?;
    write_register(i2c, 0x41, 0xD4).await?;
    // DAC right digital gain.
    write_register(i2c, 0x42, 0xD4).await?;
    // Unmute left + right DAC.
    write_register(i2c, 0x40, 0x00).await?;
    Timer::after_millis(20).await;

    Ok(())
}

