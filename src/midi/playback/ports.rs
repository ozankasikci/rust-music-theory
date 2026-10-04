//! MIDI port discovery.

use super::PlaybackError;
use midir::MidiOutput;

/// Lists available MIDI output ports on the system.
#[derive(Debug)]
pub struct MidiPorts {
    ports: Vec<String>,
}

impl MidiPorts {
    /// List all available MIDI output ports.
    pub fn list() -> Result<Self, PlaybackError> {
        let midi_out = MidiOutput::new("rust-music-theory")
            .map_err(|e| PlaybackError::InitError(e.to_string()))?;

        let ports: Vec<String> = midi_out
            .ports()
            .iter()
            .filter_map(|p| midi_out.port_name(p).ok())
            .collect();

        Ok(Self { ports })
    }

    /// Get the number of available ports.
    pub fn len(&self) -> usize {
        self.ports.len()
    }

    /// Check if there are no ports available.
    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    /// Get a port name by index.
    pub fn get(&self, index: usize) -> Option<&str> {
        self.ports.get(index).map(|s| s.as_str())
    }

    /// Iterate over port names.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.ports.iter().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_does_not_panic() {
        // Should not panic even if no MIDI devices
        let result = MidiPorts::list();
        assert!(result.is_ok());
    }

    #[test]
    fn empty_ports_methods() {
        let ports = MidiPorts { ports: vec![] };
        assert!(ports.is_empty());
        assert_eq!(ports.len(), 0);
        assert!(ports.get(0).is_none());
        assert_eq!(ports.iter().count(), 0);
    }

    #[test]
    fn ports_with_items() {
        let ports = MidiPorts {
            ports: vec!["Port A".into(), "Port B".into()],
        };
        assert!(!ports.is_empty());
        assert_eq!(ports.len(), 2);
        assert_eq!(ports.get(0), Some("Port A"));
        assert_eq!(ports.get(1), Some("Port B"));
        assert_eq!(ports.get(2), None);

        let names: Vec<_> = ports.iter().collect();
        assert_eq!(names, vec!["Port A", "Port B"]);
    }
}
