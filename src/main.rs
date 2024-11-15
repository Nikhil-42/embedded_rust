#![no_std]
#![no_main]
mod animations;

enum Orientation {
    YDown,
    XUp,
    YUp,
    XDown,
}

use rand::Rng;

use animations::{
    rotate_180, rotate_270, rotate_90, Animation, FromRaw, Life, Rainbow,
};
// pick a panicking behavior
use panic_halt as _;

// Board specific imports
use adafruit_feather_rp2040::hal::{self as hal, Clock};
use adafruit_feather_rp2040::pac;

use fugit::RateExtU32;
use rp2040_hal::pio::PIOExt;
use rp2040_hal::rosc::RingOscillator;
use smart_leds::brightness;
use smart_leds::SmartLedsWrite;
use ws2812_pio::Ws2812;

use adafruit_feather_rp2040::{
    entry,
    hal::{clocks::init_clocks_and_plls, watchdog::Watchdog, Sio},
    Pins, XOSC_CRYSTAL_FREQ,
};

use lis3dh::{Lis3dh, Lis3dhCore, SlaveAddr};

#[entry]
fn main() -> ! {
    // Grab our peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Configure clocks and PLLs
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Single-cycle I/O
    let sio = Sio::new(pac.SIO);

    // Set the pins to their initial state
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the NeoMatrix power pin
    pins.d10.into_push_pull_output_in_state(true.into());

    // Set up the pio and state machine
    let (mut pio, sm, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Set up the timer
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    // Set up the WS2812 pio
    let mut neomatrix = Ws2812::new(
        pins.d5.into_mode(),
        &mut pio,
        sm,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Set up the accelerometer
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        pins.sda.into_mode(),
        pins.scl.into_mode(),
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );
    let mut lis3dh = Lis3dh::new_i2c(i2c, SlaveAddr::Default).unwrap();

    // Set up the delay timer for enforing 24 fps
    let mut delay_timer =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    // Initialize Ring Oscillator for random number generation
    let mut rng = RingOscillator::new(pac.ROSC).initialize();

    // Initialize animations
    let mut rainbow: Rainbow = Default::default();
    let mut badapple = FromRaw::new(animations::BADAPPLE_FRAMES);
    let mut rick_roll = FromRaw::new(animations::RICK_ROLL);
    let mut life = Life::new(rng.gen(), rng.gen());

    let mut mode: Orientation = Orientation::YDown;

    // Scale acceleration to +/- 1G
    let scale = 64.0f32 / 0.004f32;

    loop {
        // Read data from accelerometer
        let bytes = lis3dh.read_accel_bytes().unwrap_or_default();
        let x = (i16::from_le_bytes([bytes[0], bytes[1]]) as f32) / scale;
        let y = (i16::from_le_bytes([bytes[2], bytes[3]]) as f32) / scale;

        // Update mode base on orientation
        if x > 0.9 {
            mode = Orientation::XUp;
        } else if x < -0.9 {
            mode = Orientation::XDown;
        } else if y > 0.9 {
            mode = Orientation::YUp;
        } else if y < -0.9 {
            mode = Orientation::YDown;
        }

        // Play the appropriate animation
        let (frame, scale) = match mode {
            Orientation::YDown => {
                let frame = rainbow.to_list();
                rainbow.next();
                (frame, 255)
            }
            Orientation::XUp => {
                let frame = life.to_list();
                life.next();
                (rotate_90(frame), 40)
            }
            Orientation::YUp => {
                let frame = rick_roll.to_list();
                rick_roll.next();
                (rotate_180(frame), 25)
            }
            Orientation::XDown => {
                let frame = badapple.to_list();
                badapple.next();
                (rotate_270(frame), 25)
            }
        };
        neomatrix
            .write(brightness(frame.iter().copied(), scale))
            .unwrap();

        // Wait for 24 fps
        delay_timer.delay_us(1000_000 / 24u32);
    }
}
