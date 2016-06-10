Copyright (C) 2016  Kliemann-Service-GmbH <s.mueller@it.kls-glt.de>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]

/// Shift register, Zugriff und Steuerung der an die Shift Register angeschlossenen Hardware
pub mod shift_register;
/// Zonen, Alarmzonen wie Störung und Schwellenwert Auswertungen
pub mod zone;
/// Sensoren, Messzellen auf den Sensorplatinen (Module)
pub mod sensor;
/// Module, Sensorplatinen mit Modbus Transceiver und einem oder mehreren Sensoren
pub mod module;
/// Server, Kernkomponente der 'xMZ-Mod-Touch'-Plattform
///
/// Der Server verbindet alle Komponenten untereinander und sorgt für deren Zusammenspiel.
pub mod server;
