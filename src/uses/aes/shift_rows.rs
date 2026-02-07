use crate::types::FieldElement;

pub(crate)fn shift_rows(state: &mut [FieldElement; 16]) {
    let temp = *state;
    // Row 1 (indices 1, 5, 9, 13)
    state[1]  = temp[5];
    state[5]  = temp[9];
    state[9]  = temp[13];
    state[13] = temp[1];

    // Row 2 (indices 2, 6, 10, 14)
    state[2]  = temp[10];
    state[6]  = temp[14];
    state[10] = temp[2];
    state[14] = temp[6];

    // Row 3 (indices 3, 7, 11, 15)
    state[3]  = temp[15];
    state[7]  = temp[3];
    state[11] = temp[7];
    state[15] = temp[11];
}