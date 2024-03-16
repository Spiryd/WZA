use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GaussanNumber {
    pub real: i64,
    pub imag: i64,
}

impl GaussanNumber {
    pub fn new(real: i64, imag: i64) -> Self {
        Self { real, imag }
    }
    pub fn norm(&self) -> i64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn gcd(&self, other: &Self) -> Self {
        if self.norm() < other.norm() {
            return other.gcd(self);
        }
        let mut a = *self;
        let mut b = *other;
        loop {
            let (_, r) = a / b;
            if r == Self::new(0, 0) {
                return b; 
            }
            a = b;
            b = r;
        }
    }
    pub fn lcm(&self, other: &Self) -> Self {
        let gcd = self.gcd(other);
        let product = *self * *other;
        (product / gcd).0
    }
}

impl Display for GaussanNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.imag == 0 {
            return write!(f, "{}", self.real);
        }
        if self.real == 0 {
            return write!(f, "{}i", self.imag);
        }
        if self.imag < 0 {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}
impl std::ops::Add for GaussanNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}
impl std::ops::Sub for GaussanNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}
impl std::ops::Mul for GaussanNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl std::ops::Div for GaussanNumber {
    type Output = (GaussanNumber, GaussanNumber);
    // Division with remainder
    fn div(self, rhs: Self) -> Self::Output {
        let real_quotient = ((self.real * rhs.real + self.imag * rhs.imag) as f64 / (rhs.real * rhs.real + rhs.imag * rhs.imag) as f64).round() as i64;
        let imag_quotient = ((self.imag * rhs.real - self.real * rhs.imag) as f64 / (rhs.real * rhs.real + rhs.imag * rhs.imag) as f64).round() as i64;
        let quotient = GaussanNumber::new(real_quotient, imag_quotient);
        let remainder = self - rhs * quotient;
        (quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn div_test() {
        let a = GaussanNumber::new(6, 0);
        let b = GaussanNumber::new(2, 1);
        let (quotient, remainder) = a / b;
        assert!(remainder.norm() <= b.norm() / 2);
        assert_eq!(quotient, GaussanNumber::new(2, -1));
        assert_eq!(remainder, GaussanNumber::new(1, 0));
    }
    #[test]
    fn gcd_test() {
        let a = GaussanNumber::new(2, -3);
        let b = GaussanNumber::new(3, 5);
        assert_eq!(a.gcd(&b), GaussanNumber::new(0, 1));
    }
    #[test]
fn lcm_test() {
    let a = GaussanNumber::new(10, 11);
    let b = GaussanNumber::new(8, 1);
    assert_eq!(a.lcm(&b), GaussanNumber::new(31, 12));
}
}
