# TMP1075

Platform agnostic rust driver for the TMP1075 temperature sensor.

Can be used with async and sync I2C interfaces that implement the `embedded_hal` or `embedded_hal_async` traits.

## Example

This example runs the async configuration using embassy on an stm32.

```rust
use embassy_executor::Spawner;
use embassy_stm32::{
    i2c,
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use fmt::{error, info};
use tmp1075::Tmp1075;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default())

    let mut i2c_conf = i2c::Config::default();
    i2c_conf.timeout = Duration::from_millis(500);
    // Depending on your setup pullups might not be needed
    i2c_conf.scl_pullup = true;
    i2c_conf.sda_pullup = true;
    let mut i2c = i2c::I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Hertz::khz(400),
        i2c_conf,
    );

    let mut temp_sens = Tmp1075Async::new(i2c);

    loop {
        match temp_sens.get_temperature().await {
            Ok(temp) => info!("Temperature: {}", temp),
            Err(e) => error!("Error: {:?}", e),
        }

        Timer::after_millis(500).await;
    }
}
```
