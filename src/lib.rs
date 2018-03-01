#![crate_type = "lib"]

pub mod board_control;

#[cfg(test)]
mod tests {
  use board_control::*;
  use std::{thread, time};

  const LED_GPIO_PIN: u32 = 18;
  const LED_PWM_FREQ_HZ: u32 = 1000;
  const PWM_FULL_RANGE: u32 = 100;


  #[test]
  fn check_led_control(){
    let bc = BoardController::new();

    bc.set_gpio_mode(LED_GPIO_PIN, GpioMode::Output);
    assert_eq!(bc.get_gpio_mode(LED_GPIO_PIN),  GpioMode::Output as i32, "set_gpio_mode failed"); 

    let delay = time::Duration::from_millis(250);
    for _i in 0..4 {
      bc.gpio_write(LED_GPIO_PIN, 1);
      thread::sleep(delay);
      bc.gpio_write(LED_GPIO_PIN, 0);
      thread::sleep(delay);
    }
    
    bc.set_hardware_pwm(LED_GPIO_PIN, LED_PWM_FREQ_HZ, (PI_HW_PWM_RANGE / 2));
    //TODO some way to verify hardware pwm?
    thread::sleep(time::Duration::from_millis(250));

    bc.set_pwm_frequency(LED_GPIO_PIN, LED_PWM_FREQ_HZ);
    assert_eq!(bc.get_pwm_frequency(LED_GPIO_PIN),LED_PWM_FREQ_HZ as i32,"set_pwm_frequency failed");
    bc.set_pwm_range(LED_GPIO_PIN, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;
    bc.set_pwm_dutycycle(LED_GPIO_PIN, (PWM_FULL_RANGE / 2)  );
    //TODO some way to verify soft pwm?
    thread::sleep(time::Duration::from_millis(250));
  }

}

