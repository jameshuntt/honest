use crate::{types::FieldElement, uses::aes::{aes_round, expand_key}};

pub fn aes_encrypt(message: [u8; 16], master_key: [u8; 16]) -> [u8; 16] {
    let mut state: [FieldElement; 16] = message.map(FieldElement);
    let round_keys = expand_key(master_key);

    // Initial Round: Just AddRoundKey
    add_round_key(&mut state, &round_keys[0..16]);

    // // Rounds 1 through 9
    // for r in 1..10 {
    //     let key_start = r * 16;
    //     aes_round(&mut state, &round_keys[key_start..key_start+16], false);
    // }
    // // Round 10: Final Round (last_round = true skips MixColumns)
    // aes_round(&mut state, &round_keys[160..176], true);
    
    // In your loop (Rounds 1-9)
    for r in 1..10 {
        let key_start = r * 16;
        let key_slice = &round_keys[key_start..key_start + 16];
        
        // Convert &[FieldElement] -> &[FieldElement; 16]
        let key_array: &[FieldElement; 16] = key_slice.try_into().expect("Key slice should be 16 bytes");
        
        aes_round(&mut state, key_array, false);
    }

    // In your final round (Round 10)
    let final_key: &[FieldElement; 16] = round_keys[160..176].try_into().expect("Final key slice should be 16 bytes");
    aes_round(&mut state, final_key, true);
    // Convert back to u8
    let mut out = [0u8; 16];
    for i in 0..16 { out[i] = state[i].0; }
    out
}

fn add_round_key(state: &mut [FieldElement; 16], key: &[FieldElement]) {
    for i in 0..16 {
        state[i] = state[i] + key[i];
    }
}