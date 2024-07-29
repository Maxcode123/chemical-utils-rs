use std::sync::OnceLock;

use crate::{
    atoms::constants::*,
    species::object::{AtomTuple, ChemicalSpecies},
};

macro_rules! create {
    ( $( ($tpl_a:expr, $tpl_s:literal) ),* ) => {{
        let mut vec: Vec<AtomTuple> = Vec::new();
        $(
            vec.push(AtomTuple::new($tpl_a, $tpl_s));
        )*
        ChemicalSpecies::new(vec)
    }};
}

macro_rules! species {
    ($f:ident, $NAME:ident, $( ($tpl_a:expr, $tpl_s:literal) ),* ) => {
        pub fn $f() -> &'static ChemicalSpecies {
            static $NAME: OnceLock<ChemicalSpecies> = OnceLock::new();
            $NAME.get_or_init(|| create!($( ($tpl_a, $tpl_s) ),*))
        }
    };
}

species! {hydrogen2, HYDROGEN2, (hydrogen(), 2)}
species! {water, WATER, (hydrogen(), 2), (oxygen(), 1)}
species! {carbon_monoxide, CARBON_MONOXIDE, (carbon(), 1), (oxygen(), 1)}
species! {methane, METHANE, (carbon(), 1), (hydrogen(), 4)}
species! {carbon_dioxide, CARBON_DIOXIDE, (carbon(), 1), (oxygen(), 2)}
