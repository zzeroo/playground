extern crate playground;

use playground::{Server, Sensor, Module};
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = Server::new();
    let module1 = Module::new();
    server.modules.push(module1);

    loop {
        thread::spawn(move || {
            // println!("Update Sensors");
            server.update_sensors();
        });

        thread::sleep(Duration::from_millis(1000));
    }
}
