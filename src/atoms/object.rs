use core::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub atomic_number: u16,
    pub atomic_mass: f32,
    pub symbol: String,
}

impl Atom {
    pub fn new(atomic_number: u16, atomic_mass: f32, symbol: &str) -> Atom {
        Atom {
            atomic_number,
            atomic_mass,
            symbol: symbol.to_string(),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Hash for Atom {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

impl Eq for Atom {}
