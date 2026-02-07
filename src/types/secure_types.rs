use classified::classified_data::ClassifiedData;
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use secrecy::SecretBox;
use zeroize::Zeroize;

#[derive(Clone)]
pub struct SecureBigUint(pub BigUint);

impl Zeroize for SecureBigUint {
    fn zeroize(&mut self) {
        self.0 = BigUint::zero();
    }
}

pub type SecretBigUint = SecretBox<SecureBigUint>;
pub type ClassifiedBigUint = ClassifiedData<BigUint>;


#[derive(Clone)]
pub struct SecureBigInt(pub BigInt);

impl Zeroize for SecureBigInt {
    fn zeroize(&mut self) {
        self.0 = BigInt::zero();
    }
}

pub type SecretBigInt = SecretBox<SecureBigInt>;
