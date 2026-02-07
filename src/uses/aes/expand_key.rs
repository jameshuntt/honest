use crate::{types::FieldElement, uses::aes::{get_rcon, s_box}};
pub fn expand_key(master_key: [u8; 16]) -> Vec<FieldElement> {
    // AES-128 needs 11 round keys (16 bytes each = 176 bytes / 44 words)
    let mut w: Vec<FieldElement> = master_key.iter().map(|&b| FieldElement(b)).collect();
    
    for i in 4..44 {
        // 'temp' is the previous 4-byte word
        let mut temp = [
            w[(i-1)*4], w[(i-1)*4 + 1], w[(i-1)*4 + 2], w[(i-1)*4 + 3]
        ];

        if i % 4 == 0 {
            // 1. RotWord: [a, b, c, d] -> [b, c, d, a]
            temp.rotate_left(1);

            // 2. SubWord: Apply S-Box to all 4 bytes
            for byte in temp.iter_mut() {
                *byte = s_box(*byte);
            }

            // 3. XOR with Rcon: Only the first byte of the word
            temp[0] = temp[0] + get_rcon(i / 4);
        }

        // w[i] = w[i-4] ^ temp
        for j in 0..4 {
            let val = w[(i-4)*4 + j] + temp[j];
            w.push(val);
        }
    }
    w
}