/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

extern crate pigrust;
extern crate libc;

use pigrust::board_control::*;

use std::{thread, time};
use libc::{c_void};

// the pins where the power LED and switch are attached
const LED_GPIO_PIN: u32 = 23;
const BUTT_IN_PIN: u32 = 3;

//const PWM_FULL_RANGE: u32 = 100;
const LED_PWM_FREQ_HZ: u32 = 1000;


#[no_mangle]
pub extern fn cb_fn_ex_hook(daemon_id: i32, gpio: u32, level: u32, tick: u32, _userdata: *mut c_void ) {
  println!("main got callback! with {} {} {} {} ", daemon_id, gpio, level, tick);
}


fn main() {
  let bc = BoardController::new();

  bc.set_gpio_mode(LED_GPIO_PIN, GpioMode::Output);
  // GPIO  set up as an input, pulled up, connected to ground on button press
  bc.set_pull_up_down(BUTT_IN_PIN, GpioPullOption::Up );

  let half_sec = time::Duration::from_millis(500);

  for _i in 0..4 {
    bc.gpio_write(LED_GPIO_PIN, 1);
    thread::sleep(half_sec);
    bc.gpio_write(LED_GPIO_PIN, 0);
    thread::sleep(half_sec);
  }

  // TODO remove this once C-style callbacks are removed
  // bc.add_edge_detector(BUTT_IN_PIN, GpioEdgeDetect::RisingEdge, cb_fn_ex_hook);
  bc.add_edge_detector_closure(BUTT_IN_PIN, GpioEdgeDetect::FallingEdge,
      |gpio, level| {
          println!("main closure! with {} {} ", gpio, level);
      }
  );

  bc.set_hardware_pwm(LED_GPIO_PIN, LED_PWM_FREQ_HZ, PI_HW_PWM_RANGE / 2 );

  loop {
    //loop forever with PWM active
    thread::sleep(half_sec);
  }
}
