
use crate::types::FieldElement;

// A high-performance "Multiply by 2" for AES
pub(crate)fn xtime(a: FieldElement) -> FieldElement {
    let raw = a.0;
    let res = if raw & 0x80 != 0 {
        (raw << 1) ^ 0x1B
    } else {
        raw << 1
    };
    FieldElement(res)
}
// #[pure]
pub(crate)fn xtime_simple(x: FieldElement) -> FieldElement {
    // If the top bit is 1, we XOR with 0x1B after shifting
    let mask = (x.0 as i8 >> 7) as u8; // Arithmetic shift creates 0xFF if bit 7 is 1
    FieldElement((x.0 << 1) ^ (mask & 0x1B))
}