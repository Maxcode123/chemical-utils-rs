use core::fmt;
use std::{collections::HashMap, ops};

use crate::species::object::ChemicalSpecies;

#[derive(Debug, Clone)]
pub struct ChemicalReactionFactor<'a> {
    pub species: &'a ChemicalSpecies,
    pub stoichiometric_coefficient: u16,
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

impl<'a> fmt::Display for ChemicalReactionFactor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stoichiometric_coefficient == 1 {
            write!(f, "{}", self.species)
        } else {
            write!(f, "{}{}", self.stoichiometric_coefficient, self.species)
        }
    }
}

#[derive(Debug)]
pub struct ChemicalReactionOperand<'a> {
    factors: Vec<ChemicalReactionFactor<'a>>,
}

fn add_operands<'a>(
    left: &ChemicalReactionOperand<'a>,
    right: &ChemicalReactionOperand<'a>,
) -> ChemicalReactionOperand<'a> {
    let mut map: HashMap<&ChemicalSpecies, u16> = HashMap::new();

    for factor in right.factors.iter().chain(&left.factors) {
        let entry = map.entry(&factor.species).or_insert(0);
        *entry += factor.stoichiometric_coefficient;
    }

    let factors = map
        .iter()
        .map(|(species, coeff)| ChemicalReactionFactor::new(species, *coeff))
        .collect();

    ChemicalReactionOperand::new(factors)
}

impl<'a> ChemicalReactionOperand<'a> {
    pub fn new(factors: Vec<ChemicalReactionFactor>) -> ChemicalReactionOperand {
        ChemicalReactionOperand { factors }
    }
}

impl<'a> fmt::Display for ChemicalReactionOperand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.factors[0])?;
        for factor in &self.factors[1..] {
            write!(f, " + {}", factor)?;
        }

        fmt::Result::Ok(())
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

impl<'a> ops::Add<&ChemicalReaction<'a>> for &ChemicalReaction<'a> {
    type Output = ChemicalReaction<'a>;

    fn add(self, rhs: &ChemicalReaction<'a>) -> ChemicalReaction<'a> {
        let mut reactants_map: HashMap<&ChemicalSpecies, u16> =
            add_operands(&self.reactants, &rhs.reactants)
                .factors
                .iter()
                .map(|factor| (factor.species, factor.stoichiometric_coefficient))
                .collect();
        let mut products_map: HashMap<&ChemicalSpecies, u16> =
            add_operands(&self.products, &rhs.products)
                .factors
                .iter()
                .map(|factor| (factor.species, factor.stoichiometric_coefficient))
                .collect();

        balance_stoichiometric_coefficients(&mut reactants_map, &mut products_map);

        let reactants: ChemicalReactionOperand = ChemicalReactionOperand::new(
            reactants_map
                .iter()
                .filter(|(_, coeff)| **coeff > 0)
                .map(|(species, coeff)| ChemicalReactionFactor::new(species, *coeff))
                .collect(),
        );

        let products: ChemicalReactionOperand = ChemicalReactionOperand::new(
            products_map
                .iter()
                .filter(|(_, coeff)| **coeff > 0)
                .map(|(species, coeff)| ChemicalReactionFactor::new(species, *coeff))
                .collect(),
        );

        ChemicalReaction::new(reactants, products)
    }
}

fn balance_stoichiometric_coefficients<'a>(
    reactants_map: &mut HashMap<&'a ChemicalSpecies, u16>,
    products_map: &mut HashMap<&'a ChemicalSpecies, u16>,
) {
    for (reactant, coeff) in reactants_map {
        match products_map.get_mut(reactant) {
            Some(other_coeff) => {
                if *coeff > *other_coeff {
                    *coeff -= *other_coeff;
                } else if *coeff == *other_coeff {
                    *coeff = 0;
                    *other_coeff = 0;
                } else {
                    *other_coeff -= *coeff;
                }
            }
            None => {}
        }
    }
}

impl<'a> fmt::Display for ChemicalReaction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.reactants, self.products)
    }
}
