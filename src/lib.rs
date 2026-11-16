// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-art: art component model.
//!
//! Real implementation: art_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtError {
    EmptyArtItemName,
    DuplicateArtItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyArtItemName => write!(f, "art_item name must not be empty"),
            Self::DuplicateArtItem(id) => write!(f, "duplicate art_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArtError {}

pub struct Art {
    pub name: String,
    art_items: BTreeMap<String, ArtItem>,
    events: BTreeMap<String, Event>,
}

impl Art {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            art_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_art_item(&mut self, item: ArtItem) -> Result<(), ArtError> {
        if item.name.trim().is_empty() {
            return Err(ArtError::EmptyArtItemName);
        }
        let id = item.id.clone();
        if self.art_items.contains_key(&id) {
            return Err(ArtError::DuplicateArtItem(id.clone()));
        }
        self.art_items.insert(id, item);
        Ok(())
    }

    pub fn art_item_count(&self) -> usize {
        self.art_items.len()
    }

    pub fn get_art_item(&self, id: &str) -> Option<&ArtItem> {
        self.art_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArtError> {
        if event.subject.trim().is_empty() {
            return Err(ArtError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_art_item(id: &str, name: &str) -> ArtItem {
        ArtItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Art::new("Art");
        assert_eq!(m.name, "Art");
        assert_eq!(m.art_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_art_item() {
        let mut m = Art::new("Art");
        m.add_art_item(sample_art_item("a1", "Alpha")).unwrap();
        assert_eq!(m.art_item_count(), 1);
        assert!(m.get_art_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Art::new("Art");
        assert!(m.add_art_item(sample_art_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Art::new("Art");
        m.add_art_item(sample_art_item("x", "One")).unwrap();
        assert!(m.add_art_item(sample_art_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Art::new("Art");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
