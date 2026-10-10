#![no_std]
#![no_main]

use panic_halt as _;

use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;

/// این بخش حتماً باید در ابتدای فلش قرار بگیرد
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// کریستال خارجی روی Pico معمولاً ۱۲ مگاهرتز است
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    // گرفتن مالکیت تمام peripheralها (فقط یک بار می‌شود این کار را کرد)
    // چرا یک بار چون ممکنه: race condition و دسترسی هزمان نا امن صورت بگیره
    // و Peripherals می‌کنه: Peripherals یعنی خودِ رجیسترها و واحدهای سخت‌افزاری چیپ که PAC برایشان یک API ساخته رو می‌گیریم که بتونیم استفاده کنیم
    let mut pac = pac::Peripherals::take().unwrap();

    // Watchdog برای تنظیم ساعت لازم است
    // Watchdog یک تایمرِ نظارتی در خودِ چیپ است.
    // یعنی به watchdog میگیم: «من هنوز زنده‌ام حزوم‌زاده ها، ریست نکن 😁».
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // راه‌اندازی ساعت‌ها و PLLها
    // PLL = مولد فرکانس دقیق
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

    // تایمر برای delay
    // اما Timer خودش کریستال نیست و
    // مستقیماً با آن کار نمی‌کند؛ clock tree سیستم از منابع clock، از جمله کریستال، استفاده می‌کند
    // و Timer یک مرجع زمانی مناسب دریافت می‌کند.
    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // دسترسی به GPIO
    // این بخش برای آماده‌کردن پایه‌های GPIO و روشن/خاموش کردن LED داخلی Pico است.
    let sio = hal::Sio::new(pac.SIO);
    ///این خط سیستم GPIO را راه می‌اندازد و پین‌ها را آماده می‌کند.
    //
    // ورودی‌ها:
    //
    // pac.IO_BANK0 → تنظیم عملکرد پایه‌ها
    // pac.PADS_BANK0 → تنظیم ویژگی‌های الکتریکی پایه‌ها
    // sio.gpio_bank0 → کنترل GPIO از سمت SIO
    // &mut pac.RESETS → خارج کردن این بخش‌ها از reset
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );


    // LED داخلی Pico روی GPIO25 است
    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        timer.delay_ms(2000);
        led.set_low().unwrap();
        timer.delay_ms(500);
    }
}