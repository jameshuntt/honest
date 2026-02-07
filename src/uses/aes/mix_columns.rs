use crate::{types::FieldElement, uses::aes::xtime};

pub fn mix_columns(state: &mut [FieldElement; 16]) {
    let two = FieldElement(0x02);
    let three = FieldElement(0x03);

    // We process each of the 4 columns manually
    // Column 0
    let s0 = state[0]; let s1 = state[1]; let s2 = state[2]; let s3 = state[3];
    state[0] = (two * s0) ^ (three * s1) ^ s2 ^ s3;
    state[1] = s0 ^ (two * s1) ^ (three * s2) ^ s3;
    state[2] = s0 ^ s1 ^ (two * s2) ^ (three * s3);
    state[3] = (three * s0) ^ s1 ^ s2 ^ (two * s3);

    // Column 1
    let s4 = state[4]; let s5 = state[5]; let s6 = state[6]; let s7 = state[7];
    state[4] = (two * s4) ^ (three * s5) ^ s6 ^ s7;
    state[5] = s4 ^ (two * s5) ^ (three * s6) ^ s7;
    state[6] = s4 ^ s5 ^ (two * s6) ^ (three * s7);
    state[7] = (three * s4) ^ s5 ^ s6 ^ (two * s7);

    // Column 2
    let s8 = state[8]; let s9 = state[9]; let s10 = state[10]; let s11 = state[11];
    state[8] = (two * s8) ^ (three * s9) ^ s10 ^ s11;
    state[9] = s8 ^ (two * s9) ^ (three * s10) ^ s11;
    state[10] = s8 ^ s9 ^ (two * s10) ^ (three * s11);
    state[11] = (three * s8) ^ s9 ^ s10 ^ (two * s11);

    // Column 3
    let s12 = state[12]; let s13 = state[13]; let s14 = state[14]; let s15 = state[15];
    state[12] = (two * s12) ^ (three * s13) ^ s14 ^ s15;
    state[13] = s12 ^ (two * s13) ^ (three * s14) ^ s15;
    state[14] = s12 ^ s13 ^ (two * s14) ^ (three * s15);
    state[15] = (three * s12) ^ s13 ^ s14 ^ (two * s15);
}

pub fn mix_columns_xtime(state: &mut [FieldElement; 16]) {
    // Process each column manually for formal verification clarity
    
    // Column 0 (Indices 0, 1, 2, 3)
    let s0 = state[0]; let s1 = state[1]; let s2 = state[2]; let s3 = state[3];
    state[0] = xtime(s0) ^ (xtime(s1) ^ s1) ^ s2 ^ s3;
    state[1] = s0 ^ xtime(s1) ^ (xtime(s2) ^ s2) ^ s3;
    state[2] = s0 ^ s1 ^ xtime(s2) ^ (xtime(s3) ^ s3);
    state[3] = (xtime(s0) ^ s0) ^ s1 ^ s2 ^ xtime(s3);

    // Column 1 (Indices 4, 5, 6, 7)
    let s4 = state[4]; let s5 = state[5]; let s6 = state[6]; let s7 = state[7];
    state[4] = xtime(s4) ^ (xtime(s5) ^ s5) ^ s6 ^ s7;
    state[5] = s4 ^ xtime(s5) ^ (xtime(s6) ^ s6) ^ s7;
    state[6] = s4 ^ s5 ^ xtime(s6) ^ (xtime(s7) ^ s7);
    state[7] = (xtime(s4) ^ s4) ^ s5 ^ s6 ^ xtime(s7);

    // Column 2 (Indices 8, 9, 10, 11)
    let s8 = state[8]; let s9 = state[9]; let s10 = state[10]; let s11 = state[11];
    state[8] = xtime(s8) ^ (xtime(s9) ^ s9) ^ s10 ^ s11;
    state[9] = s8 ^ xtime(s9) ^ (xtime(s10) ^ s10) ^ s11;
    state[10] = s8 ^ s9 ^ xtime(s10) ^ (xtime(s11) ^ s11);
    state[11] = (xtime(s8) ^ s8) ^ s9 ^ s10 ^ xtime(s11);

    // Column 3 (Indices 12, 13, 14, 15)
    let s12 = state[12]; let s13 = state[13]; let s14 = state[14]; let s15 = state[15];
    state[12] = xtime(s12) ^ (xtime(s13) ^ s13) ^ s14 ^ s15;
    state[13] = s12 ^ xtime(s13) ^ (xtime(s14) ^ s14) ^ s15;
    state[14] = s12 ^ s13 ^ xtime(s14) ^ (xtime(s15) ^ s15);
    state[15] = (xtime(s12) ^ s12) ^ s13 ^ s14 ^ xtime(s15);
}

pub fn mix_columns_dry(state: &mut [FieldElement; 16]) {
    let two = FieldElement(0x02);
    let three = FieldElement(0x03);

    for i in 0..4 {
        let c = i * 4;
        let s0 = state[c];
        let s1 = state[c + 1];
        let s2 = state[c + 2];
        let s3 = state[c + 3];

        state[c]     = (two * s0)   + (three * s1) + s2 + s3;
        state[c + 1] = s0           + (two * s1)   + (three * s2) + s3;
        state[c + 2] = s0           + s1           + (two * s2)   + (three * s3);
        state[c + 3] = (three * s0) + s1           + s2           + (two * s3);
    }
}