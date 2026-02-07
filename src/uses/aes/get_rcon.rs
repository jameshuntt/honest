use crate::types::FieldElement;

pub fn get_rcon(round: usize) -> FieldElement {
    // Rcon(i) = 2^(i-1) in GF(2^8)
    // Round 1 -> 2^0 = 1, Round 2 -> 2^1 = 2, etc.
    let mut val = FieldElement(1);
    let two = FieldElement(2);
    for _ in 1..round {
        val = val * two;
    }
    val
}

