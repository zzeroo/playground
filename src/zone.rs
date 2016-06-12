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

/// Datenstruktur die die Zonen representiert
#[derive(Debug, Eq, PartialEq)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub alarmpunkte: Vec<bool>,
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
    /// assert_eq!(stoerung.alarmpunkte.len(), 1);
    /// assert_eq!(zone1.alarmpunkte.len(), 4);
    /// ```
    pub fn new(zone_type: ZoneType) -> Self {
        match zone_type {
            ZoneType::STOERUNG => Zone {
                zone_type: ZoneType::STOERUNG,
                alarmpunkte: vec![false],
            },
            ZoneType::SCHWELLENWERT => Zone {
                zone_type: ZoneType::SCHWELLENWERT,
                alarmpunkte: vec![false; 4],
            }
        }
    }

}
