#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Assign each segment pin individually
    let mut seg_a = pins.d2.into_output();
    let mut seg_b = pins.d3.into_output();
    let mut seg_c = pins.d4.into_output();
    let mut seg_d = pins.d5.into_output();
    let mut seg_e = pins.d6.into_output();
    let mut seg_f = pins.d7.into_output();
    let mut seg_g = pins.d8.into_output();

    const DIGITS: [[bool; 7]; 10] = [
        [true, true, true, true, true, true, false],     // 0
        [false, true, true, false, false, false, false], // 1
        [true, true, false, true, true, false, true],    // 2
        [true, true, true, true, false, false, true],    // 3
        [false, true, true, false, false, true, true],   // 4
        [true, false, true, true, false, true, true],    // 5
        [true, false, true, true, true, true, true],     // 6
        [true, true, true, false, false, false, false],  // 7
        [true, true, true, true, true, true, true],      // 8
        [true, true, true, true, false, true, true],     // 9
    ];

    loop {
        for digit in DIGITS.iter() {
            if digit[0] {
                seg_a.set_high();
            } else {
                seg_a.set_low();
            }
            if digit[1] {
                seg_b.set_high();
            } else {
                seg_b.set_low();
            }
            if digit[2] {
                seg_c.set_high();
            } else {
                seg_c.set_low();
            }
            if digit[3] {
                seg_d.set_high();
            } else {
                seg_d.set_low();
            }
            if digit[4] {
                seg_e.set_high();
            } else {
                seg_e.set_low();
            }
            if digit[5] {
                seg_f.set_high();
            } else {
                seg_f.set_low();
            }
            if digit[6] {
                seg_g.set_high();
            } else {
                seg_g.set_low();
            }

            arduino_hal::delay_ms(1000);
        }
    }
}
