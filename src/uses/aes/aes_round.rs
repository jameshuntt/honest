use crate::{
    types::FieldElement,
    uses::aes::{mix_columns, s_box, shift_rows}
};

pub fn aes_round(
    state: &mut [FieldElement; 16],
    key: &[FieldElement; 16],
    last_round: bool) {
    // 1. SubBytes
    for byte in state.iter_mut() {
        *byte = s_box(*byte);
    }

    // 2. ShiftRows
    shift_rows(state);

    // 3. MixColumns (Skipped in the final round)
    if !last_round {
        mix_columns(state);
    }

    // 4. AddRoundKey
    for i in 0..16 {
        state[i] = state[i] + key[i];
    }
}