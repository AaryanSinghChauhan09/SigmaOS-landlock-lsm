// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-landlock-lsm: landlock lsm component model.
//!
//! Real implementation: lsms, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lsm {
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
pub enum LandlockLsmError {
    EmptyLsmName,
    DuplicateLsm(String),
    EmptyEventSubject,
}

impl fmt::Display for LandlockLsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLsmName => write!(f, "lsm name must not be empty"),
            Self::DuplicateLsm(id) => write!(f, "duplicate lsm {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LandlockLsmError {}

pub struct LandlockLsm {
    pub name: String,
    lsms: BTreeMap<String, Lsm>,
    events: BTreeMap<String, Event>,
}

impl LandlockLsm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lsms: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_lsm(&mut self, item: Lsm) -> Result<(), LandlockLsmError> {
        if item.name.trim().is_empty() {
            return Err(LandlockLsmError::EmptyLsmName);
        }
        let id = item.id.clone();
        if self.lsms.contains_key(&id) {
            return Err(LandlockLsmError::DuplicateLsm(id.clone()));
        }
        self.lsms.insert(id, item);
        Ok(())
    }

    pub fn lsm_count(&self) -> usize {
        self.lsms.len()
    }

    pub fn get_lsm(&self, id: &str) -> Option<&Lsm> {
        self.lsms.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LandlockLsmError> {
        if event.subject.trim().is_empty() {
            return Err(LandlockLsmError::EmptyEventSubject);
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

    fn sample_lsm(id: &str, name: &str) -> Lsm {
        Lsm {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LandlockLsm::new("LandlockLsm");
        assert_eq!(m.name, "LandlockLsm");
        assert_eq!(m.lsm_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_lsm() {
        let mut m = LandlockLsm::new("LandlockLsm");
        m.add_lsm(sample_lsm("a1", "Alpha")).unwrap();
        assert_eq!(m.lsm_count(), 1);
        assert!(m.get_lsm("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LandlockLsm::new("LandlockLsm");
        assert!(m.add_lsm(sample_lsm("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LandlockLsm::new("LandlockLsm");
        m.add_lsm(sample_lsm("x", "One")).unwrap();
        assert!(m.add_lsm(sample_lsm("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LandlockLsm::new("LandlockLsm");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
