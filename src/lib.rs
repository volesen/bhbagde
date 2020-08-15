#![no_std]

pub extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd21::*;

use hal::clock::GenericClockController;
#[cfg(feature = "usb")]
use hal::gpio::IntoFunction;
use hal::gpio::{Floating, Input, Port};
use hal::sercom::{I2CMaster2, PadPin};
use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusWrapper;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin a0 = a2,

    /// Analog Pin 1
    pin a1 = b8,
    /// Analog Pin 2
    pin a2 = b9,
    /// Analog Pin 3
    pin a3 = a4,
    /// Analog Pin 4
    pin a4 = a5,
    /// Analog Pin 5
    pin a5 = b2,

    /// Pin 0, rx.  Also analog input (A6)
    pin d0 = a11,
    /// Pin 1, tx.  Also analog input (A7)
    pin d1 = a10,
    /// Pin 2
    pin d2 = a14,
    /// Pin 3, PWM capable
    pin d3 = a22,
    /// Pin 4, PWM capable.  Also analog input (A8)
    pin d4 = a23,
    /// Pin 5, PWM capable.  Also analog input (A9)
    pin d5 = a15,
    /// Pin 6, PWM capable
    pin d6 = a20,
    /// Pin 7
    pin d7 = a21,
    /// Pin 8, PWM capable.  Also analog input (A10)
    pin d8 = a6,
    /// Pin 9, PWM capable.  Also analog input (A11)
    pin d9 = a7,
    /// Pin 10, PWM capable
    pin d10 = a18,
    /// Pin 11, PWM capable
    pin d11 = a16,
    /// Pin 12, PWM capable
    pin d12 = a19,
    /// Digital pin number 13, which is also attached to
    /// the red LED.  PWM capable.
    pin d13 = a17,

    pin sda = a8,
    pin scl = a9,

    /// The data line attached to the neopixel.
    /// Is also attached to SWCLK.
    pin neopixel = a30,

    /// The SPI SCK attached the to 2x3 header
    pin sck = b11,
    /// The SPI MOSI attached the to 2x3 header
    pin mosi = b10,
    /// The SPI MISO attached the to 2x3 header
    pin miso = a12,

    /// The SCK pin attached to the on-board SPI flash
    pin flash_sck = b23,
    /// The MOSI pin attached to the on-board SPI flash
    pin flash_mosi = b22,
    /// The MISO pin attached to the on-board SPI flash
    pin flash_miso = b3,
    /// The CS pin attached to the on-board SPI flash
    pin flash_cs = a13,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom2: pac::SERCOM2,
    pm: &mut pac::PM,
    sda: gpio::Pa8<Input<Floating>>,
    scl: gpio::Pa9<Input<Floating>>,
    port: &mut Port,
) -> hal::sercom::I2CMaster2<hal::sercom::Sercom2Pad0<gpio::Pa8<gpio::PfD>>, hal::sercom::Sercom2Pad1<gpio::Pa9<gpio::PfD>>> {
    let gclk0 = clocks.gclk0();
    I2CMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        bus_speed.into(),
        sercom2,
        pm,
        sda.into_pad(port),
        scl.into_pad(port),
    )
}
