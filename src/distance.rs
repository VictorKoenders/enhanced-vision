use gpio_cdev::{Chip, EventRequestFlags, EventType, Line, LineHandle, LineRequestFlags};
use std::{thread, time};

const SOUND_SPEED: f64 = 343.0;

pub struct UltrasonicSensor {
    trig: LineHandle,
    echo: Line,
}

impl UltrasonicSensor {
    pub fn new(trig_pin: u32, echo_pin: u32) -> gpio_cdev::errors::Result<UltrasonicSensor> {
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let echo = chip.get_line(echo_pin)?;
        let trig = chip
            .get_line(trig_pin)?
            .request(LineRequestFlags::OUTPUT, 0, "ultrasonic")?;

        Ok(UltrasonicSensor { echo, trig })
    }

    fn trigger(&self) -> gpio_cdev::errors::Result<()> {
        self.trig.set_value(1)?;
        thread::sleep(time::Duration::from_nanos(10));
        self.trig.set_value(0)?;

        Ok(())
    }

    pub fn poll(&self) -> gpio_cdev::errors::Result<f64> {
        let mut last_echo_time = time::Instant::now();
        self.trigger()?;
        for event in self.echo.events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "ultrasonic",
        )? {
            let evt = event?;
            match evt.event_type() {
                EventType::RisingEdge => {
                    last_echo_time = time::Instant::now();
                }
                EventType::FallingEdge => {
                    let t = last_echo_time.elapsed().as_nanos();
                    return Ok((t as f64 / 1_000_000_000.0) * 0.5 * SOUND_SPEED);
                }
            }
        }
        panic!("ultrasonic event loop stopped")
    }
}
