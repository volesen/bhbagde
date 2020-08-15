//http://www.multiwingspan.co.uk/micro.php?page=matdrive
//https://github.com/adafruit/Adafruit_CircuitPython_IS31FL3731/blob/master/adafruit_is31fl3731.py
//https://github.com/adafruit/Adafruit_IS31FL3731/blob/master/Adafruit_IS31FL3731.cpp

#![no_std]
#![no_main]

mod lib;

extern crate cortex_m;

use panic_halt as _;

use lib as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::time::KiloHertz;
use embedded_hal::blocking::i2c;

type Option<T> = core::option::Option<T>;

const _MODE_REGISTER : u8 = 0x00;

const _CONFIG_BANK : u8 = 0x0B;
const _BANK_ADDRESS : u8 = 0xFD;
const _PICTURE_MODE : u8 = 0x00;

const ISSI_ADDR_DEFAULT : u8 = 0x74;

const ISSI_REG_CONFIG : u8 = 0x00;
const ISSI_REG_CONFIG_PICTUREMODE : u8 = 0x00;
const ISSI_REG_CONFIG_AUTOPLAYMODE : u8 = 0x08;
const ISSI_REG_CONFIG_AUDIOPLAYMODE : u8 = 0x18;

const ISSI_CONF_PICTUREMODE : u8 = 0x00;
const ISSI_CONF_AUTOFRAMEMODE : u8 = 0x04;
const ISSI_CONF_AUDIOMODE : u8 = 0x08;

const ISSI_REG_PICTUREFRAME : u8 = 0x01;

const ISSI_REG_SHUTDOWN : u8 = 0x0A;
const ISSI_REG_AUDIOSYNC : u8 = 0x06;

const ISSI_COMMANDREGISTER : u8 = 0xFD;
const ISSI_BANK_FUNCTIONREG : u8 = 0x0B;  // helpfully called 'page nine'

const ADDRESS : u8 = 0x74;

const REG_CONFIG  : u8 = 0x00;
const REG_CONFIG_PICTUREMODE : u8 = 0x00;
const REG_CONFIG_AUTOPLAYMODE : u8 = 0x08;
const REG_CONFIG_AUDIOPLAYMODE : u8 = 0x18;

const CONF_PICTUREMODE : u8 = 0x00;
const CONF_AUTOFRAMEMODE : u8 = 0x04;
const CONF_AUDIOMODE : u8 = 0x08;
 
const REG_PICTUREFRAME  : u8 = 0x01;

const REG_SHUTDOWN : u8 = 0x0A;
const REG_AUDIOSYNC : u8 = 0x06;

const COMMANDREGISTER : u8 = 0xFD;
const BANK_FUNCTIONREG : u8 = 0x0B; 

const FRAME : u8 = 0x00;

//fn _bank<I2C: i2c::Write>(i2c: &mut I2C, bank: Option<u8>) -> Option<u8> {
//    let mut result: [u8; 1] = [0];
//    match bank {
//        None => {
//            i2c.write_read(0x74, &[_BANK_ADDRESS], &mut result);
//            Some(result[0])
//        }
//        Some(x) => {
//            result[0] = x;
//            i2c.write_read(0x74, &[_BANK_ADDRESS], &mut result);
//            None
//        }
//    }
//}
//
//fn _register<I2C: i2c::Write>(i2c: &mut I2C, bank: u8, register: u8, value: Option<u8>) -> Option<u8> {
//    _bank(i2c, Some(bank));
//    let mut result: [u8; 1] = [1];
//    match value {
//        None => {
//            i2c.write_read(0x74, &[register], &mut result);
//            Some(result[0])
//        }
//        Some(x) =>{
//            result[0] = x;
//            i2c.write_read(0x74, &[register], &mut result);
//            None
//        }
//    }
//}

fn write_register<I2C: i2c::Write>(i2c: &mut I2C, reg: u8,  data: u8) {
   let mut d: [u8; 1] = [reg];
   i2c.write(0x74, &d);
   let mut d: [u8; 1] = [data];
   i2c.write(0x74, &d);
}

fn select_bank<I2C: i2c::Write>(i2c: &mut I2C, bank: u8){
    write_register(i2c, COMMANDREGISTER, bank);
}

fn write_register8<I2C: i2c::Write>(i2c: &mut I2C, bank: u8, reg: u8, value: u8){
    select_bank(i2c, bank);
    write_register(i2c, reg, value);
}

fn set_led_xy<I2C: i2c::Write>(i2c: &mut I2C, x: u8, y: u8, frame: u8, value: u8) {
    write_register8(i2c, frame, 0x24 + x + y * 16, value);
}

// fn fill<I2C: i2c::Write>(i2c: &mut I2C, value){
//     select_bank(i2c, 0);
//     for i in 0..6{
//         d = [0x24 + i * 24] + [value] * 24;
//         i2c.write_read(0x74, d, 0xff)
//     }
// }

fn _init<I2C: i2c::Write>(i2c: &mut I2C, delay: &mut Delay) {
    // Have you tryed to turn it off and on again?
    write_register8(i2c, BANK_FUNCTIONREG, REG_SHUTDOWN, 0x0);
    delay.delay_ms(10u32);
    write_register8(i2c, BANK_FUNCTIONREG, REG_SHUTDOWN, 0x1);
   
    // Set picture mode
    write_register8(i2c, BANK_FUNCTIONREG, REG_CONFIG, REG_CONFIG_PICTUREMODE);
    write_register8(i2c, BANK_FUNCTIONREG, REG_PICTUREFRAME, FRAME);

    for f in 0..8 {
        for i in 0..15{
            write_register8(i2c, f, i, 0xff);
        }
    }

    // Turn off audio sync
    write_register8(i2c, BANK_FUNCTIONREG, REG_AUDIOSYNC, 0x0);
}


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.a1.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    //  atsamd8oards/pygamer/examples/neopixel_tilt.rs
    // https://github.com/BenBergman/lis3dh-rs 
    let mut i2c = hal::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );
    // https://github.com/qmk/qmk_firmware/blob/master/drivers/issi/is31fl3731.c

    _init(&mut i2c, &mut delay);

    // fill(i2c, 127);
    // set_led_xy(&mut i2c, 5, 5, 0, 240);
    write_register(&mut i2c, 5, 0xff);
    
    loop {
        delay.delay_ms(100u32);
        red_led.set_high().unwrap();
        delay.delay_ms(100u32);
        red_led.set_low().unwrap();
    }
}