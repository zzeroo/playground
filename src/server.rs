use module::{Module, ModuleType};
use zone::{Zone, ZoneType};

/// Server Datenstruktur
pub struct Server<'a> {
    pub zones: Vec<Zone>,
    module: Vec<Module<'a>>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(ZoneType::STOERUNG),
                Zone::new(ZoneType::SCHWELLENWERT),
            ],
            module: vec![
                Module::new(ModuleType::RAGAS_CO_NO),
            ],
        }
    }
}


#[cfg(test)]
mod server_test {
    use super::*;
    use zone::{Zone, ZoneType};

    #[test]
    fn test_default_zones() {
        let server = Server::new();
        assert_eq!(server.zones[0].zone_type, ZoneType::STOERUNG);
        assert_eq!(server.zones[1].zone_type, ZoneType::SCHWELLENWERT);
    }
}
