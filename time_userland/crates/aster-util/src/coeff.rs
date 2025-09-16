use core::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Coeff {
    mult: u32,
    shift: u32,
    max_multiplier: u64,
}

impl Coeff {
    pub fn new(numerator: u64, denominator: u64, max_multiplier: u64) -> Self {
        let mut shift_acc: u32 = 32;
        let mut tmp = max_multiplier >> 32;
        while tmp > 0 { tmp >>= 1; shift_acc -= 1; }
        let mut shift = 32;
        let mut mult = 0;
        while shift > 0 {
            mult = numerator << shift;
            mult += denominator / 2;
            mult /= denominator;
            if (mult >> shift_acc) == 0 { break; }
            shift -= 1;
        }
        Self { mult: mult as u32, shift, max_multiplier }
    }
    pub fn mult(&self) -> u32 { self.mult }
    pub fn shift(&self) -> u32 { self.shift }
}

impl Mul<u64> for Coeff {
    type Output = u64;
    fn mul(self, rhs: u64) -> Self::Output { (rhs * self.mult as u64) >> self.shift }
}
impl Mul<u32> for Coeff {
    type Output = u32;
    fn mul(self, rhs: u32) -> Self::Output { ((rhs as u64 * self.mult as u64) >> self.shift) as u32 }
}

