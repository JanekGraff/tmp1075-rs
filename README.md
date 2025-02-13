# TMP1075

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![dependency status][deps-image]][deps-link]
![MIT licensed][license-image]

Platform agnostic rust driver for the TMP1075 temperature sensor.

Can be used with async and sync I2C interfaces that implement the `embedded_hal` or `embedded_hal_async` traits.

## Resources

- [TMP1075 product page][product-page]
- [TMP1075 datasheet][datasheet]

## Missing Features

- [ ] Set High and Low limit registers as floats for finer granularity. Currently only setting the MSB as i8 is supported.

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

    let mut temp_sens = Tmp1075::new(i2c);

    loop {
        match temp_sens.get_temperature().await {
            Ok(temp) => info!("Temperature: {}", temp),
            Err(e) => error!("Error: {:?}", e),
        }

        Timer::after_millis(500).await;
    }
}
```

## Contributing

If you have any problems feel free to open an issue, if i find the time i might review and fix it.

Also feel free to open PRs if you miss some features or find bugs. PRs for documentation, tests, examples, etc. are also very welcome!

## License

Dual licensed under your choice of either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crate-image]: https://img.shields.io/crates/v/tmp1075.svg
[crate-link]: https://crates.io/crates/tmp1075
[docs-image]: https://docs.rs/tmp1075/badge.svg
[docs-link]: https://docs.rs/tmp1075/
[build-image]: https://github.com/JanekGraff/tmp1075-rs/actions/workflows/ci.yml/badge.svg?branch=main
[build-link]: https://github.com/JanekGraff/tmp1075-rs/actions
[deps-image]: https://deps.rs/repo/github/janekgraff/tmp1075-rs/status.svg
[deps-link]: https://deps.rs/repo/github/janekgraff/tmp1075-rs/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[product-page]: https://www.ti.com/product/TMP1075
[datasheet]: https://www.ti.com/lit/gpn/tmp1075
