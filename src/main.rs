#![no_std]
#![no_main]

panic_serial::impl_panic_handler!(
  // This must be the exact type returned by `arduino_hal::default_serial!`.
  arduino_hal::usart::Usart<
    arduino_hal::pac::USART0,
    arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PD0>,
    arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PD1>
  >
);

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);

    // panic-serial takes ownership of the serial port, and returns a `&'static mut` reference to it:
    let serial = share_serial_port_with_panic(serial);

    // continue using `serial` as you would normally do:
    ufmt::uwriteln!(serial, "Hello from rust!\r").unwrap();

    // cause a panic
    let opt: Option<u8> = None;
    opt.expect("Oh no! There is no number in here!");

    loop {}
}
