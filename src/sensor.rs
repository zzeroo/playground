use zone::{Zone};

/// Sensortyp, Art der Messzelle
#[derive(Debug, Eq, PartialEq)]
pub enum SensorType {
    NemotoCO,
    NemotoNO,
}

/// Representiert eine Sensor Messzelle
///
/// Jeder Sensor kann keiner, einer oder mehreren Zonen (Alarmzonen) zugeordnet werden.
#[derive(Debug, Eq, PartialEq)]
pub struct Sensor<'a> {
    sensor_type: SensorType,
    adc_value: u32,
    zones: Vec<&'a Zone>,
}

impl<'a> Sensor<'a> {
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoCO => Sensor {
                sensor_type: SensorType::NemotoCO,
                adc_value: 0,
                zones: vec!(),
            },
            SensorType::NemotoNO => Sensor {
                sensor_type: SensorType::NemotoNO,
                adc_value: 0,
                zones: vec!(),
            },
        }
    }
}

#[cfg(test)]
mod sensor_tests {
    use super::*;
    use server::Server;

    #[test]
    fn sensor_nemoto_co() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.sensor_type, SensorType::NemotoCO);
    }

    #[test]
    fn sensor_mit_keiner_zone() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.zones.len(), 0);
    }

    #[test]
    fn sensor_mit_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        assert_eq!(sensor.zones.len(), 1);
    }

    #[test]
    fn sensor_mit_mehr_als_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        sensor.zones.push(&server.zones[1]);
        assert_eq!(sensor.zones.len(), 2);
    }

    #[test]
    fn sensor_mit_einer_zone_kann_alarmpunkt_setzen() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        assert_eq!(server.zones[0].alarmpunkte[0], false);
    }
}
