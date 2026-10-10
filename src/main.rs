#![no_std]
#![no_main]

use panic_halt as _;

use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;

/// This section must be placed at the beginning of flash.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// The external crystal on the Pico is usually 12 MHz.
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    // Take ownership of all peripherals (this can only be done once).
    // Why only once? Because a race condition and unsafe concurrent access could occur.
    // Peripherals are the chip's registers and hardware units, for which the PAC provides an API so we can use them.
    let mut pac = pac::Peripherals::take().unwrap();

    // The watchdog is needed to configure the clock.
    // The watchdog is a monitoring timer built into the chip.
    // In other words, we tell the watchdog: "I'm still alive, don't reset me 😁".
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Initialize the clocks and PLLs.
    // PLL = a precise frequency generator.
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .unwrap();

    // Timer for delays.
    // However, the Timer itself is not a crystal, and
    // it does not work with the crystal directly; the system clock tree uses clock sources, including the crystal,
    // and the Timer receives a suitable time reference.
    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Access GPIO.
    // This section prepares the GPIO pins and turns the Pico's built-in LED on and off.
    let sio = hal::Sio::new(pac.SIO);
    ///This line initializes the GPIO system and prepares the pins.
    //
    // Inputs:
    //
    // pac.IO_BANK0 → configures the pins' functions
    // pac.PADS_BANK0 → configures the pins' electrical characteristics
    // sio.gpio_bank0 → controls GPIO through the SIO
    // &mut pac.RESETS → takes these blocks out of reset
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );


    // The Pico's built-in LED is on GPIO25.
    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        timer.delay_ms(2000);
        led.set_low().unwrap();
        timer.delay_ms(500);
    }
}
