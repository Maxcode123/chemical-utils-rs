#[derive(Debug, Clone)]
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
