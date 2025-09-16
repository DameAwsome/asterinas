#![no_std]

pub mod coeff {
    #[derive(Clone, Copy)]
    pub struct Coeff { num: u64, den: u64, max: u64 }
    impl Coeff {
        pub fn new(num: u64, den: u64, max: u64) -> Self { Self { num, den, max } }
    }
    impl core::ops::Mul<u64> for Coeff {
        type Output = u64;
        fn mul(self, rhs: u64) -> u64 {
            let rhs = rhs.min(self.max);
            (rhs.saturating_mul(self.num)) / self.den.max(1)
        }
    }
}

