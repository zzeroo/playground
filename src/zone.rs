use std::sync::{Arc, RwLock};

/// Zonentypen
///
/// Die Anzahl der Alarmpunkte unterscheidet sich je nach Zonentype
///
/// * STOERUNG          - Die Zone Störung hat nur ein Alarmpunkt
/// * SCHWELLENWERT     - SCHWELLENWERT Zonen haben 4 Alarmpunkte
#[derive(Debug, Eq, PartialEq)]
pub enum ZoneType {
    STOERUNG,
    SCHWELLENWERT,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    NO,     // Normal open
    NC,     // Normal closed
}

/// Datenstruktur die die Zonen representiert
pub struct Zone {
    pub zone_type: ZoneType,
    pub alarmpunkte: Arc<RwLock<Vec<bool>>>,
    direction: Direction,
}

impl Zone {
    /// Erstellt eine neue Zone
    ///
    /// # Arguments
    /// * `zone_type`       - Art der Zone
    ///
    /// # Examples
    ///
    /// ```
    /// use playground::zone::{Zone, ZoneType};
    ///
    /// let stoerung = Zone::new(ZoneType::STOERUNG);
    /// let zone1 = Zone::new(ZoneType::SCHWELLENWERT);
    ///
    /// assert_eq!(stoerung.alarmpunkte.read().unwrap().len(), 1);
    /// assert_eq!(zone1.alarmpunkte.read().unwrap().len(), 4);
    /// ```
    pub fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::STOERUNG => Zone {
                zone_type: ZoneType::STOERUNG,
                alarmpunkte: Arc::new(RwLock::new(vec![false])),
                direction: Direction::NC,
            },
            ZoneType::SCHWELLENWERT => Zone {
                zone_type: ZoneType::SCHWELLENWERT,
                alarmpunkte: Arc::new(RwLock::new(vec![false; 4])),
                direction: Direction::NO,
            }
        }
    }

    // TODO: Dokumentation!
    pub fn alarmpunkt(&self, id: usize) -> Option<bool> {
        match self.alarmpunkte.read().unwrap().len() > id {
            true => Some(self.alarmpunkte.read().unwrap()[id]),
            _ => None,
        }
    }

    // TODO: Dokumentation
    pub fn alarmpunkt_set(&mut self, id: usize, value: bool) {
        match self.alarmpunkt(id) {
            Some(_) => { self.alarmpunkte.write().unwrap()[id] = value; },
            None => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zone_stoerung() {
        let zone = Zone::new(ZoneType::STOERUNG);
        assert_eq!(zone.alarmpunkte.read().unwrap().len(), 1);
    }

    #[test]
    fn zone_schwellenwert() {
        let zone = Zone::new(ZoneType::SCHWELLENWERT);
        assert_eq!(zone.alarmpunkte.read().unwrap().len(), 4);
    }

    #[test]
    fn zone_stoerung_ist_normal_closed() {
        let zone = Zone::new(ZoneType::STOERUNG);
        assert_eq!(zone.direction, Direction::NC);
    }

    #[test]
    fn zone_schwellenwert_ist_normal_open() {
        let zone = Zone::new(ZoneType::SCHWELLENWERT);
        assert_eq!(zone.direction, Direction::NO);
    }
}
