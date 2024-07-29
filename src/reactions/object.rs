use crate::species::object::ChemicalSpecies;

#[derive(Debug)]
pub struct ChemicalReactionFactor<'a> {
    species: &'a ChemicalSpecies,
    stoichiometric_coefficient: u16,
}

impl<'a> ChemicalReactionFactor<'a> {
    pub fn new(
        species: &'a ChemicalSpecies,
        stoichiometric_coefficient: u16,
    ) -> ChemicalReactionFactor<'a> {
        ChemicalReactionFactor {
            species,
            stoichiometric_coefficient,
        }
    }
}

#[derive(Debug)]
pub struct ChemicalReactionOperand<'a> {
    factors: Vec<ChemicalReactionFactor<'a>>,
}

impl<'a> ChemicalReactionOperand<'a> {
    pub fn new(factors: Vec<ChemicalReactionFactor>) -> ChemicalReactionOperand {
        ChemicalReactionOperand { factors }
    }
}

#[derive(Debug)]
pub struct ChemicalReaction<'a> {
    reactants: ChemicalReactionOperand<'a>,
    products: ChemicalReactionOperand<'a>,
}

impl<'a> ChemicalReaction<'a> {
    pub fn new(
        reactants: ChemicalReactionOperand<'a>,
        products: ChemicalReactionOperand<'a>,
    ) -> ChemicalReaction<'a> {
        ChemicalReaction {
            reactants,
            products,
        }
    }
}
