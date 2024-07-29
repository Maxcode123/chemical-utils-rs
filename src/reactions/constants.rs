use crate::reactions::object::{
    ChemicalReaction, ChemicalReactionFactor, ChemicalReactionOperand,
};
use crate::species::constants::*;
use std::sync::OnceLock;

macro_rules! operand {
    ( $( ($sp:expr, $st:literal) ),* ) => {{
        let mut vec: Vec<ChemicalReactionFactor> = Vec::new();
        $(
            vec.push(ChemicalReactionFactor::new($sp, $st));
        )*
        ChemicalReactionOperand::new(vec)
    }};
}

macro_rules! reaction {
    ($f:ident, $NAME:ident, $r:expr, $p:expr) => {
        pub fn $f() -> &'static ChemicalReaction<'static> {
            static $NAME: OnceLock<ChemicalReaction> = OnceLock::new();
            $NAME.get_or_init(|| ChemicalReaction::new($r, $p))
        }
    };
}

// CH4 + H20 -> CO + 3H2
reaction! {steam_methane_reforming, STEAM_METHANE_REFORMING, operand!((methane(), 1), (water(), 1)), operand!((carbon_monoxide(), 1), (hydrogen2(), 2))}
// CO + H20 -> CO2 + H2
reaction! {water_gas_shift, WATER_GAS_SHIFT, operand!((carbon_monoxide(), 1), (water(), 1)), operand!((carbon_dioxide(), 1), (hydrogen2(), 1))}
