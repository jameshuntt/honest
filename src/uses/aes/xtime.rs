
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
