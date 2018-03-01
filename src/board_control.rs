/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

/// Used for detecting GPIO input changes 
pub enum GpioEdgeDetect {
  RisingEdge = 0,
  FallingEdge = 1,
  EitherEdge = 2,
}

/// Pull Up/Down options for GPIO pins
pub enum GpioPullOption {
  Off = 0,
  Down = 1,
  Up = 2,
}

/// I/O direction options for GPIO pins
pub enum GpioMode {
  Input = 0,
  Output = 1,
}

// Hardware PWM
pub const PI_HW_PWM_MIN_FREQ: u32 =  1;
pub const PI_HW_PWM_MAX_FREQ: u32 =  125000000;
pub const PI_HW_PWM_RANGE: u32 = 1000000;


// Used for callbacks from pigpiod to us
pub type CBFuncEx = extern fn(i32, u32, u32, u32, u32);

#[link(name = "pigpiod_if2")]
extern  {
  // start/stop the pigpiod access socket
  fn pigpio_start(addr: *const u8, port: *const u8) -> i32 ;
  fn pigpio_stop(daemon_id: i32); 

  // GPIO configuration
  fn set_mode(daemon_id: i32, gpio: u32, mode: u32) -> i32;
  fn get_mode(daemon_id: i32, gpio: u32) -> i32;
  fn set_pull_up_down(daemon_id: i32, gpio: u32, pud: u32) -> i32;
  fn gpio_read(daemon_id: i32, gpio: u32) -> i32;
  fn gpio_write(daemon_id: i32, gpio: u32, level: u32) -> i32;

  // PWM
  fn set_PWM_dutycycle(daemon_id: i32, gpio: u32, duty: u32) -> i32;
  fn set_PWM_range(daemon_id: i32, gpio: u32, range: u32) -> i32;
  fn set_PWM_frequency(daemon_id: i32, gpio: u32, freq: u32) -> i32;
  fn get_PWM_frequency(daemon_id: i32, gpio: u32) -> i32;
  fn hardware_PWM(daemon_id: i32, gpio: u32, freq: u32, duty: u32) -> i32; 

  //edge event detection
  fn callback_ex(daemon_id: i32, gpio: u32, edge: u32, cb_func: CBFuncEx, userdata: u32) -> i32;
}



pub struct BoardController {
  daemon_id: i32,
}

impl BoardController {

  pub fn new() -> BoardController {
      let mut this = BoardController {
        daemon_id: -1 ,
      };
      this.init_daemon();
      this
  }

  fn init_daemon(&mut self) {
    unsafe {
      self.daemon_id = pigpio_start(0 as *const u8, 0 as *const u8);
    }
    if self.daemon_id < 0 {
     panic!("Could not start pigpiod access!");
    }
  } 

  pub fn set_gpio_mode(&self, gpio: u32, mode: GpioMode) -> i32 {
    unsafe { set_mode(self.daemon_id, gpio, mode as u32) }
  }

  pub fn get_gpio_mode(&self, gpio: u32) -> i32 {
    unsafe { get_mode(self.daemon_id, gpio) }
  }

  pub fn set_pull_up_down(&self, gpio: u32, pud: GpioPullOption) -> i32 {
    unsafe { set_pull_up_down(self.daemon_id, gpio, pud as u32) }
  }

  pub fn set_pwm_frequency(&self, gpio: u32, freq: u32) -> i32 {
    unsafe { set_PWM_frequency(self.daemon_id, gpio, freq) }
  }

  pub fn get_pwm_frequency(&self, gpio: u32) -> i32 {
    unsafe { get_PWM_frequency(self.daemon_id, gpio) } 
  }
 
  pub fn set_pwm_range(&self, gpio: u32, range: u32) -> i32 {
    unsafe { set_PWM_range(self.daemon_id, gpio, range) }
  }

  pub fn set_pwm_dutycycle(&self, gpio: u32, duty: u32) -> i32 {
    unsafe { set_PWM_dutycycle(self.daemon_id, gpio, duty) }
  }

  pub fn set_hardware_pwm(&self, gpio: u32, freq: u32, duty: u32) -> i32 {
    unsafe { hardware_PWM(self.daemon_id, gpio, freq, duty) }
  }

  pub fn gpio_read(&self, gpio: u32) -> i32 {
    unsafe { gpio_read(self.daemon_id, gpio) }
  }

  pub fn gpio_write(&self, gpio: u32, value: u32) -> i32 {
    unsafe { gpio_write(self.daemon_id, gpio, value) }
  }
 
  pub fn add_edge_detector(&self, gpio: u32, edge: GpioEdgeDetect, cb: CBFuncEx ) -> i32 {
    let raw : *const BoardController = self;
    let raw = raw as u32;
    unsafe {
      callback_ex(self.daemon_id, gpio, edge as u32, cb, raw)
    }
  }
}


impl Drop for BoardController {

  fn drop(&mut self) {
    if self.daemon_id >= 0 {
      unsafe { pigpio_stop(self.daemon_id); };
      self.daemon_id= -1;
    }
  }

}
