use crate::types::FieldElement;
pub fn s_box(input: FieldElement) -> FieldElement {
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


pub fn inv_s_box(input: FieldElement) -> FieldElement {
    let s = input.0;
    // 1. Inverse Affine Transformation
    // This is the constant-time version of the AES inverse affine map
    let out = (s.rotate_right(1) ^ s.rotate_right(3) ^ s.rotate_right(6)) ^ 0x05;
    
    // 2. Multiplicative Inverse (using your existing logic)
    // 0 is still mapped to 0 in AES
    if out == 0 {
        FieldElement(0)
    } else {
        FieldElement(out).inv()
    }
}

#[test]
fn s_box_0x01() {
    let v = s_box(FieldElement(0x01));
    assert_eq!(v.0, 0x7C)
}




pub fn is_sbox_inverse(x: u8) -> bool {
    inv_s_box(s_box(FieldElement(x))) == FieldElement(x)
}