use core::ops::Mul;
#[derive(Debug, Copy, Clone)] pub struct Coeff { mult: u32, shift: u32, max_multiplier: u64 }
impl Coeff {
	pub fn new(n: u64, d: u64, max: u64) -> Self {
		let mut shift_acc: u32 = 32; let mut tmp = max >> 32;
		while tmp > 0 { tmp >>= 1; shift_acc -= 1; }
		let mut shift = 32; let mut mult = 0;
		while shift > 0 { mult = n << shift; mult += d / 2; mult /= d; if (mult >> shift_acc) == 0 { break; } shift -= 1; }
		Self { mult: mult as u32, shift, max_multiplier: max }
	}
	pub fn mult(&self) -> u32 { self.mult } pub fn shift(&self) -> u32 { self.shift }
}
impl Mul<u64> for Coeff { type Output = u64; fn mul(self, rhs: u64) -> u64 { (rhs * self.mult as u64) >> self.shift } }
impl Mul<u32> for Coeff { type Output = u32; fn mul(self, rhs: u32) -> u32 { ((rhs as u64 * self.mult as u64) >> self.shift) as u32 } }