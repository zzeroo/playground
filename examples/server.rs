extern crate playground;

use playground::{Server, Sensor, Module};
use std::thread;
use std::time::Duration;

fn main() {

    loop {
        let mut server = Server::new();
        let mut module = Module::new();
        let sensor1 = Sensor::new();
        let sensor2 = Sensor::new();
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        server.modules.push(module);


        thread::spawn(move || {
            // println!("Update Sensors");
            server.update_sensors();
        });

        thread::sleep(Duration::from_millis(1000));
    }
}
