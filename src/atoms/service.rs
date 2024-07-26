use std::{collections::HashMap, sync::OnceLock};

use super::object::Atom;

/// Get an atom by symbol. Returns None if no atom with the given symbol exists.
pub fn get_by_symbol(symbol: &str) -> Option<&'static Atom> {
    match get_symbol_registry().get(symbol) {
        Some(atom) => Some(atom.clone()),
        None => None,
    }
}

pub fn get_by_atomic_number(atomic_number: u16) -> Option<&'static Atom> {
    match get_atomic_number_registry().get(&atomic_number) {
        Some(atom) => Some(atom.clone()),
        None => None,
    }
}

fn create_atomic_number_registry() -> HashMap<u16, &'static Atom> {
    let mut registry: HashMap<u16, &'static Atom> = HashMap::new();

    for atom in get_atoms_vector() {
        registry.insert(atom.atomic_number, atom);
    }

    registry
}

fn create_symbol_registry() -> HashMap<&'static str, &'static Atom> {
    let mut registry: HashMap<&str, &'static Atom> = HashMap::new();

    for atom in get_atoms_vector() {
        registry.insert(&atom.symbol, atom);
    }

    registry
}

fn create_atoms_vector() -> Vec<&'static Atom> {
    let mut vector: Vec<&'static Atom> = Vec::with_capacity(57);

    vector.push(hydrogen());
    vector.push(helium());

    vector
}

fn get_atomic_number_registry() -> &'static HashMap<u16, &'static Atom> {
    static ATOMIC_NUMBER_REGISTRY: OnceLock<HashMap<u16, &'static Atom>> =
        OnceLock::new();
    ATOMIC_NUMBER_REGISTRY.get_or_init(|| create_atomic_number_registry())
}

fn get_symbol_registry() -> &'static HashMap<&'static str, &'static Atom> {
    static SYMBOL_REGISTRY: OnceLock<HashMap<&str, &'static Atom>> = OnceLock::new();
    SYMBOL_REGISTRY.get_or_init(|| create_symbol_registry())
}

fn get_atoms_vector() -> &'static Vec<&'static Atom> {
    static ATOMS_VECTOR: OnceLock<Vec<&'static Atom>> = OnceLock::new();
    ATOMS_VECTOR.get_or_init(|| create_atoms_vector())
}

macro_rules! atom {
    ($f:ident, $NAME:ident, $an:literal, $am:literal, $s:literal) => {
        fn $f() -> &'static Atom {
            static $NAME: OnceLock<Atom> = OnceLock::new();
            $NAME.get_or_init(|| Atom::new($an, $am, $s))
        }
    };
}

atom! {hydrogen, HYDROGEN, 1, 1.0080, "H"}
atom! {helium, HELIUM, 2, 4.00260, "He"}
atom! {lithium, LITHIUM, 3, 7.0, "Li"}
atom! {beryllium, BERYLLIUM, 4, 9.012183, "Be"}
atom! {boron, BORON, 5, 10.81, "B"}
atom! {carbon, CARBON, 6, 12.011, "C"}
atom! {nitrogen, NITROGEN, 7, 14.007, "N"}
atom! {oxygen, OXYGEN, 8, 15.999, "O"}
atom! {fluorine, FLUORINE, 9, 18.99840316, "F"}
atom! {neon, NEON, 10, 20.180, "Ne"}
atom! {sodium, SODIUM, 11, 22.9897693, "Na"}
atom! {magnesium, MAGNESIUM, 12, 24.305, "Mg"}
atom! {aluminum, ALUMINUM, 13, 26.981538, "Al"}
atom! {sillicon, SILLICON, 14, 28.085, "Si"}
atom! {phosphorus, PHOSPHORUS, 15, 30.97376200, "P"}
atom! {sulfur, SULFUR, 16, 32.07, "S"}
atom! {chlorine, CHLORINE, 17, 35.45, "Cl"}
atom! {argon, ARGON, 18, 39.9, "Ar"}
atom! {potassium, POTASSIUM, 19, 39.0983, "K"}
atom! {calcium, CALCIUM, 20, 40.08, "Ca"}
atom! {scandium, SCANDIUM, 21, 44.95591, "Sc"}
atom! {titanium, TITANIUM, 22, 47.867, "Ti"}
atom! {vanadium, VANADIUM, 23, 50.9415, "V"}
atom! {chromium, CHROMIUM, 24, 51.996, "Cr"}
atom! {manganese, MANGANESE, 25, 54.93804, "Mn"}
atom! {iron, IRON, 26, 55.84, "Fe"}
atom! {cobalt, COBALT, 27, 58.93319, "Co"}
atom! {nickel, NICKEL, 28, 58.693, "Ni"}
atom! {copper, COPPER, 29, 63.55, "Cu"}
atom! {zinc, ZINC, 30, 65.4, "Zn"}
atom! {gallium, GALLIUM, 31, 69.723, "Ga"}
atom! {germanium, GERMANIUM, 32, 72.63, "Ge"}
atom! {arsenic, ARSENIC, 33, 74.92159, "As"}
atom! {selenium, SELENIUM, 34, 78.97, "Se"}
atom! {bromine, BROMINE, 35, 79.90, "Br"}
atom! {krypton, KRYPTON, 36, 83.80, "Kr"}
atom! {rubidium, RUBIDIUM, 37, 85.468, "Rb"}
atom! {strontium, STRONTIUM, 38, 87.62, "Sr"}
atom! {ytrium, YTRIUM, 39, 88.90584, "Y"}
atom! {zitronium, ZITRONIUM, 40, 91.22, "Zr"}
atom! {niobium, NIOBIUM, 41, 92.90637, "Nb"}
atom! {molybdenium, MOLYBDENIUM, 42, 95.95, "Mo"}
atom! {technetium, TECHNETIUM, 43, 96.90636, "Tc"}
atom! {ruthenium, RUTHENIUM, 44, 101.1, "Ru"}
atom! {rhodium, RHODIUM, 45, 102.9055, "Rh"}
atom! {palladium, PALLADIUM, 46, 106.42, "Pd"}
atom! {silver, SILVER, 47, 107.868, "Ag"}
atom! {cadmium, CADMIUM, 48, 112.41, "Cd"}
atom! {indium, INDIUM, 49, 114.818, "In"}
atom! {tin, TIN, 50, 118.71, "Sn"}
atom! {antimony, ANTIMONY, 51, 121.760, "Sb"}
atom! {tellurium, TELLURIUM, 52, 127.6, "Te"}
atom! {iodine, IODINE, 53, 126.9045, "I"}
atom! {xenon, XENON, 54, 131.29, "Xe"}
atom! {cesium, CESIUM, 55, 132.9054520, "Cs"}
atom! {barium, BARIUM, 56, 137.33, "Ba"}
atom! {hafnium, HAFNIUM, 72, 178.49, "Hf"}
atom! {tantalum, TANTALUM, 73, 180.9479, "Ta"}
