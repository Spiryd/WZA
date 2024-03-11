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
            let (q, r) = a / b;
            println!("q: {}, r: {}", q, r);
            if r == Self::new(0, 0) || r.norm() < b.norm() {
                return b;
            }
            a = b;
            b = r;
        }
    }
    pub fn lcm(&self, other: &Self) -> Self {
        todo!()
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
        let real_quotient = (self.real as f64 / rhs.real as f64).round() as i64;
        let imag_quotient = (self.imag as f64 / rhs.imag as f64).round() as i64;
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
        let a = GaussanNumber::new(10, 7);
        let b = GaussanNumber::new(3, 2);
        let (quotient, remainder) = a / b;
        assert_eq!(quotient, GaussanNumber::new(3, 4));
        assert_eq!(remainder, GaussanNumber::new(9, -11));
    }
    #[test]
    fn gcd_test() {
        let a = GaussanNumber::new(4, 7);
        let b = GaussanNumber::new(1, 3);
        println!("{:?}", a / b);
        assert_eq!(a.gcd(&b), GaussanNumber::new(1, 0));
    }
}
