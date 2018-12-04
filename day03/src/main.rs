#[macro_use]
extern crate pest_derive;

mod claim;

use crate::claim::{Claim, ParseClaimError, Unit};
use hashbrown::HashMap;
use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let claims = claims()?;
    let unit_map = build_unit_map(&claims);

    let viable_claims = claims.iter().filter(|claim| {
        claim
            .units()
            .all(|x| unit_map.get(&x).map(|&x| x).unwrap_or(0) == 1)
    });

    for claim in viable_claims {
        println!("{}", claim.id());
    }

    Ok(())
}

fn claims() -> Result<Vec<Claim>, ParseClaimError> {
    grabinput::from_stdin().map(|s| s.trim().parse()).collect()
}

fn build_unit_map(claims: &[Claim]) -> HashMap<Unit, usize> {
    let mut units = HashMap::new();

    claims
        .iter()
        .flat_map(|claim| claim.units())
        .for_each(|unit| *units.entry(unit).or_insert(0) += 1);

    units
}
