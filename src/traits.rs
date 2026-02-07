pub trait GaloisField: Sized + Copy {
    fn run_add(self, other: Self) -> Self;
    fn run_sub(self, other: Self) -> Self;
    fn run_mul(self, other: Self) -> Self;
    fn run_inv(self) -> Self;
    const ZERO: Self;
    const ONE: Self;
}

