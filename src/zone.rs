use std::sync::{Arc, RwLock};

/// Zonentypen
///
/// Die Anzahl der Alarmpunkte unterscheidet sich je nach Zonentype
///
/// * Stoerung          - Die Zone Störung hat nur ein Alarmpunkt
/// * Schwellenwert     - Schwellenwert Zonen haben 4 Alarmpunkte
#[derive(Debug, Eq, PartialEq)]
pub enum ZoneType {
    Stoerung,
    Schwellenwert,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    NO,     // Normal open
    NC,     // Normal closed
}

/// Datenstruktur die die Zonen representiert
pub struct Zone {
    pub zone_type: ZoneType,
    alarmpunkte: Arc<RwLock<Vec<bool>>>,
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
    /// let stoerung = Zone::new(ZoneType::Stoerung);
    /// let zone1 = Zone::new(ZoneType::Schwellenwert);
    ///
    /// assert_eq!(stoerung.alarmpunkt(0), Some(false));
    /// assert_eq!(zone1.alarmpunkt(0), Some(false));
    /// ```
    pub fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::Stoerung => Zone {
                zone_type: ZoneType::Stoerung,
                alarmpunkte: Arc::new(RwLock::new(vec![false])),
                direction: Direction::NC,
            },
            ZoneType::Schwellenwert => Zone {
                zone_type: ZoneType::Schwellenwert,
                alarmpunkte: Arc::new(RwLock::new(vec![false; 4])),
                direction: Direction::NO,
            }
        }
    }

    /// Fragt ein Alarmpunkt ab
    ///
    /// # Arguments
    /// * `id`       - ID des Alarmpunkts
    ///
    /// # Examples
    ///
    /// ```
    /// use playground::zone::{Zone, ZoneType};
    ///
    /// let stoerung = Zone::new(ZoneType::Stoerung);
    ///
    /// assert_eq!(stoerung.alarmpunkt(0), Some(false));
    /// assert_eq!(stoerung.alarmpunkt(99), None);
    /// ```
    pub fn alarmpunkt(&self, id: usize) -> Option<bool> {
        match self.alarmpunkte.read().unwrap().len() > id {
            true => Some(self.alarmpunkte.read().unwrap()[id]),
            _ => None,
        }
    }

    /// Fragt ein Alarmpunkt ab
    ///
    /// # Arguments
    /// * `id`       - ID des Alarmpunkts
    /// * `value`    - zu setzender Wert des Alarmpunkts (entweder `true` oder `false`)
    ///
    /// # Examples
    ///
    /// ```
    /// use playground::zone::{Zone, ZoneType};
    ///
    /// let stoerung = Zone::new(ZoneType::Stoerung);
    ///
    /// stoerung.alarmpunkt_set(0, true);
    /// assert_eq!(stoerung.alarmpunkt(0), Some(true));
    /// ```
    pub fn alarmpunkt_set(&self, id: usize, value: bool) {
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
        let zone = Zone::new(ZoneType::Stoerung);
        assert_eq!(zone.alarmpunkt(0), Some(false));
        assert_eq!(zone.alarmpunkt(1), None);
        assert_eq!(zone.alarmpunkt(2), None);
        assert_eq!(zone.alarmpunkt(3), None);
    }

    #[test]
    fn zone_schwellenwert() {
        let zone = Zone::new(ZoneType::Schwellenwert);
        assert_eq!(zone.alarmpunkt(0), Some(false));
        assert_eq!(zone.alarmpunkt(1), Some(false));
        assert_eq!(zone.alarmpunkt(2), Some(false));
        assert_eq!(zone.alarmpunkt(3), Some(false));
    }

    #[test]
    fn zone_stoerung_ist_normal_closed() {
        let zone = Zone::new(ZoneType::Stoerung);
        assert_eq!(zone.direction, Direction::NC);
    }

    #[test]
    fn zone_schwellenwert_ist_normal_open() {
        let zone = Zone::new(ZoneType::Schwellenwert);
        assert_eq!(zone.direction, Direction::NO);
    }
}
