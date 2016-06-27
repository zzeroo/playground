extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

pub struct Sensor {
    adc_value: u32,
}

pub struct Module {
    sensors: Vec<Sensor>,
}

pub struct Server {
    pub modules: Vec<Module>,
}


impl Sensor {
    fn new() -> Self {
        Sensor {
            adc_value: 0,
        }
    }

    fn adc_value(&self) -> u32 {
        self.adc_value
    }

    /// # Params
    /// `modbus-parameter`  - Device, Modbus Adresse, Register Adresse, ....
    fn update_adc(&mut self) {
        let between = Range::new(0u32, 1024);
        let mut rng = rand::thread_rng();
        self.adc_value = between.ind_sample(&mut rng);
    }
}

impl Module {
    pub fn new() -> Self {
        Module {
            sensors: vec![],
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            modules: vec![],
        }
    }

    pub fn update_sensors(&mut self) {
        for module in self.modules.iter_mut() {
            for sensor in module.sensors.iter_mut() {
                sensor.update_adc();
                println!("Sensor: ADC VALUE: {}", sensor.adc_value());
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use {Module, Server, Sensor};

    #[test]
    fn basic() {
        let server = Server::new();
        assert_eq!(server.modules.len(), 0);
    }

    #[test]
    fn sensor_update_adc() {
        let mut sensor = Sensor::new();
        assert_eq!(sensor.adc_value(), 0);
        sensor.update_adc();
        assert!(sensor.adc_value() >= 0 && sensor.adc_value() < 1024);
    }

    #[test]
    fn server_normal_setup() {
        let mut server = Server::new();
        let module1 = Module::new();
        server.modules.push(module1);
        assert_eq!(server.modules.len(), 1);
        assert_eq!(server.modules.get(0).unwrap().sensors.len(), 0);
    }

    #[test]
    fn server_background_thread_update_sensors() {
        let mut server = Server::new();
        let mut module1 = Module::new();
        server.modules.push(module1);
        server.update_sensors();
    }
}
