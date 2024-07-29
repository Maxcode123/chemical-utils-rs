use core::fmt;

use crate::atoms::object::Atom;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AtomTuple {
    pub atom: &'static Atom,
    pub size: u16,
}

impl AtomTuple {
    pub fn new(atom: &'static Atom, size: u16) -> AtomTuple {
        AtomTuple { atom, size }
    }
}

impl fmt::Display for AtomTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size > 1 {
            write!(f, "{}{}", self.atom, self.size)
        } else {
            write!(f, "{}", self.atom)
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ChemicalSpecies {
    pub atom_tuples: Vec<AtomTuple>,
}

impl ChemicalSpecies {
    pub fn new(atom_tuples: Vec<AtomTuple>) -> ChemicalSpecies {
        ChemicalSpecies { atom_tuples }
    }

    pub fn eq(&self, other: &ChemicalSpecies) -> bool {
        self.atom_tuples
            .iter()
            .filter(|e| !other.atom_tuples.contains(e))
            .count()
            == 0
    }
}

impl fmt::Display for ChemicalSpecies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tpl in &self.atom_tuples {
            write!(f, "{}", tpl)?;
        }

        fmt::Result::Ok(())
    }
}
