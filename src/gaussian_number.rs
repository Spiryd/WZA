use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct GaussanNumber {
    pub real: i64,
    pub imag: i64,
}

impl GaussanNumber {
    pub fn norm(&self) -> i64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn gcd(&self, other: &Self) -> Self {
        todo!()
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

impl std::ops::Div for GaussanNumber {
    type Output = (GaussanNumber, GaussanNumber);

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
