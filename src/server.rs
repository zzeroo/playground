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
    use zone::{ZoneType};

    #[test]
    fn zone_defaults() {
        let server = Server::new();
        assert_eq!(server.zones[0].zone_type, ZoneType::STOERUNG);
        assert_eq!(server.zones[1].zone_type, ZoneType::SCHWELLENWERT);
        // TODO: teste das
        // assert_eq!(server.zones[6].zone_type, true);
        assert_eq!(server.zones[0].alarmpunkte.len(), 1);
        assert_eq!(server.zones[1].alarmpunkte.len(), 4);
    }

    #[test]
    fn zonen_und_module() {
        let server = Server::new();
        assert_eq!(server.module.len(), 1);
    }

    #[test]
    fn server_kann_alarmpunkte_setzen() {
        let mut server = Server::new();
        assert_eq!(server.zones[0].alarmpunkte[0], false);
        server.zones[0].alarmpunkte[0] = true;
        assert_eq!(server.zones[0].alarmpunkte[0], true);
    }
}
