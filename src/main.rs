/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

extern crate pigrust;
use pigrust::board_control::*;

use std::{thread, time};

// the pins where the power LED and switch are attached
const LED_GPIO_PIN: u32 = 18;
const BUTT_IN_PIN: u32 = 17;

const PWM_FULL_RANGE: u32 = 100;
const LED_PWM_FREQ_HZ: u32 = 1000;

fn main() {
  let bc = BoardController::new();

  bc.set_gpio_mode(LED_GPIO_PIN, GpioMode::Output);
  // GPIO 17 set up as an input, pulled down, connected to 3V3 on button press
  bc.set_pull_up_down(BUTT_IN_PIN, GpioPullOption::Down );

  let half_sec = time::Duration::from_millis(500);

  for _i in 0..4 {
    bc.gpio_write(LED_GPIO_PIN, 1);
    thread::sleep(half_sec);
    bc.gpio_write(LED_GPIO_PIN, 0);
    thread::sleep(half_sec);
  }

  // bc.set_pwm_frequency(LED_GPIO_PIN, LED_PWM_FREQ_HZ);
  // bc.set_pwm_range(LED_GPIO_PIN, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;
  // bc.set_pwm_dutycycle(LED_GPIO_PIN, (PWM_FULL_RANGE / 2)  );

  bc.set_hardware_pwm(LED_GPIO_PIN, LED_PWM_FREQ_HZ, (PI_HW_PWM_RANGE / 2));

  loop {
    //loop forever with PWM active
    thread::sleep(half_sec);
  }
}
