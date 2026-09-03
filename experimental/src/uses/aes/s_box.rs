use crate::types::FieldElement;


pub fn s_box(input: FieldElement) -> FieldElement {
    // 1. Multiplicative Inverse (0 is mapped to 0)
    let b = if input.0 == 0 { 0 } else { input.inv().0 };
    
    // 2. Affine Transformation (Constant-time bit-munging)
    let mut s = b;
    let mut out = 0u8;
    for i in 0..8 {
        out |= ((s & 1) 
            ^ ((s >> 4) & 1) 
            ^ ((s >> 5) & 1) 
            ^ ((s >> 6) & 1) 
            ^ ((s >> 7) & 1) 
            ^ ((0x63 >> i) & 1)) << i;
        s = s.rotate_left(1);
    }
    FieldElement(out)
}

pub fn s_box_s(input: FieldElement) -> FieldElement {
    // 1. Multiplicative Inverse (0 is mapped to 0)
    let b = if input.0 == 0 { 0 } else { input.inv().0 };
    
    // 2. Affine Transformation (Constant-time bit-munging)
    let mut s = b;
    let mut out = 0u8;
    for i in 0..8 {
        let bit =
            ((b >> i) & 1) ^
            ((b >> ((i + 4) % 8)) & 1) ^
            ((b >> ((i + 5) % 8)) & 1) ^
            ((b >> ((i + 6) % 8)) & 1) ^
            ((b >> ((i + 7) % 8)) & 1) ^
            ((0x63 >> i) & 1);
        out |= bit << i;
    }
    FieldElement(out)
}



pub fn s_box_2(input: FieldElement) -> FieldElement {
    let b = if input.0 == 0 { 0 } else { input.inv().0 };
    
    // x ^ (x << 1) ^ (x << 2) ^ (x << 3) ^ (x << 4) ^ 0x63 (all mod x^8 + 1)
    let s = b;
    let out = s 
        ^ s.rotate_left(1) 
        ^ s.rotate_left(2) 
        ^ s.rotate_left(3) 
        ^ s.rotate_left(4) 
        ^ 0x63;

    FieldElement(out)
}

#[test]
fn s_box_0x01() {
    let v = s_box_s(FieldElement(0x01));
    assert_eq!(v.0, 0x7C)
}
pub fn inv_s_box(input: FieldElement) -> FieldElement {
    let s = input.0;
    // 1. Inverse Affine Transformation
    // This is the constant-time version of the AES inverse affine map
    let mut out = (s.rotate_right(1) ^ s.rotate_right(3) ^ s.rotate_right(6)) ^ 0x05;
    
    // 2. Multiplicative Inverse (using your existing logic)
    // 0 is still mapped to 0 in AES
    if out == 0 {
        FieldElement(0)
    } else {
        FieldElement(out).inv()
    }
}