use crate::atoms::object::Atom;

#[derive(Debug)]
pub struct AtomTuple {
    pub atom: &'static Atom,
    pub size: u16,
}

impl AtomTuple {
    pub fn new(atom: &'static Atom, size: u16) -> AtomTuple {
        AtomTuple { atom, size }
    }
}

#[derive(Debug)]
pub struct ChemicalSpecies {
    pub atom_tuples: Vec<AtomTuple>,
}

impl ChemicalSpecies {
    pub fn new(atom_tuples: Vec<AtomTuple>) -> ChemicalSpecies {
        ChemicalSpecies { atom_tuples }
    }
}
