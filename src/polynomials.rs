#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

#[allow(dead_code)]
impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Polynomial {
        Polynomial { coefficients }
    }
    pub fn degree(&self) -> isize {
        if self.coefficients.len() == 1 && self.coefficients[0] == 0.0 {
            -1
        } else {
            self.coefficients.len() as isize - 1
        }
    }
    pub fn lc(&self) -> f64 {
        *self.coefficients.last().unwrap()
    }
    pub fn gcd(&self, other: Polynomial) -> Polynomial {
        if self.degree() < other.degree() {
            return other.gcd(self.clone());
        }
        let mut a = self.clone();
        let mut b = other.clone();
        while b.coefficients != vec![0.0] {
            let (_, r) = a.clone() / b.clone();
            a = b;
            b = r;
        }
        a
    }
    pub fn lcm(&self, other: Polynomial) -> Polynomial {
        let gcd = self.gcd(other.clone());
        let product = self.clone() * other.clone();
        (product / gcd).0
    }
    pub fn gdc_all(polynomials: &[Polynomial]) -> Polynomial {
        polynomials.iter().fold(Self::default(), |acc, x| acc.gcd(x.clone()))
    }
    pub fn ext_gcd(a: Polynomial, b: Polynomial) -> (Polynomial, Polynomial, Polynomial) {
        if b.coefficients == vec![0.0] {
            return (a, Polynomial::new(vec![1.0]), Polynomial::new(vec![0.0]));
        }
        let (q, r) = a.clone() / b.clone();
        let (d, s, t) = Self::ext_gcd(b.clone(), r);
        (d, t.clone(), s - q * t)
    }
}

impl Default for Polynomial{
    fn default() -> Self {
        Self { coefficients: vec![0.0] }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut terms = Vec::new();
        for (i, coeff) in self.coefficients.iter().enumerate().rev() {
            if *coeff != 0.0 {
                let term = match i {
                    0 => format!("{}", coeff),
                    1 => format!("{}x", coeff),
                    _ => format!("{}x^{}", coeff, i),
                };
                terms.push(term);
            }
        }
        let polynomial = terms.join(" + ");
        write!(f, "{}", polynomial)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        let mut result = vec![0.0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];
        for i in 0..result.len() {
            if i < self.coefficients.len() {
                result[i] += self.coefficients[i];
            }
            if i < other.coefficients.len() {
                result[i] += other.coefficients[i];
            }
        }
        while result.last() == Some(&0.0) && result.len() > 1{
            result.pop();
        }
        Polynomial::new(result)
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, other: Polynomial) -> Polynomial {
        let mut result = vec![0.0; std::cmp::max(self.coefficients.len(), other.coefficients.len())];
        for i in 0..result.len() {
            if i < self.coefficients.len() {
                result[i] += self.coefficients[i];
            }
            if i < other.coefficients.len() {
                result[i] -= other.coefficients[i];
            }
        }
        while result.last() == Some(&0.0) && result.len() > 1{
            result.pop();
        }
        Polynomial::new(result)
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
        let mut result = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];
        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }
        Polynomial::new(result)
    }
}

impl std::ops::Div for Polynomial {
    type Output = (Polynomial, Polynomial);

    // Euclidean division
    fn div(self, rhs: Self) -> Self::Output {
        let mut q = Polynomial::new(vec![0.0]);
        let mut r = self.clone();
        let d = rhs.clone();
        let c = rhs.lc();
        while r.degree() >= d.degree() {
            let mut s = vec![0.0; (r.degree() - d.degree()) as usize];
            s.push(r.lc() / c);
            let t = Polynomial::new(s);
            q = q + t.clone();
            r = r - (t * d.clone());
        }
        (q, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn test_lc() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(p.lc(), 3.0);
    }

    #[test]
    fn test_add() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 1.0]);
        let p3 = p1 + p2;
        assert_eq!(p3.coefficients, vec![3.0, 3.0, 2.0]);
    }

    #[test]
    fn test_sub() {
        let p1 = Polynomial::new(vec![3.0, 1.0, 1.0]);
        let p2 = Polynomial::new(vec![2.0, 2.0, 1.0]);
        let p3 = p1 - p2;
        assert_eq!(p3.coefficients, vec![1.0, -1.0]);
    }

    #[test]
    fn test_mul() {
        let p1 = Polynomial::new(vec![1.0, 2.0, 1.0]);
        let p2 = Polynomial::new(vec![1.0, 1.0]);
        let p3 = p1 * p2;
        assert_eq!(p3.coefficients, vec![1.0, 3.0, 3.0, 1.0]);
    }

    #[test]
    fn test_div() {
        let p1 = Polynomial::new(vec![5.0, 4.0, -3.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 1.0]);
        let (q, r) = p1 / p2;
        assert_eq!(q.coefficients, vec![18.0, -7.0, 2.0]);
        assert_eq!(r.coefficients, vec![-31.0]);
    }

    #[test]
    fn test_gcd() {
        let p1 = Polynomial::new(vec![6., 7., 1.]);
        let p2 = Polynomial::new(vec![-6., -5., 1.]);
        let p3 = p1.gcd(p2);
        assert_eq!(p3.coefficients, vec![12., 12.]);
    }

    #[test]
    fn test_lcm() {
        let p1 = Polynomial::new(vec![6., 7., 1.]);
        let p2 = Polynomial::new(vec![-6., -5., 1.]);
        let p3 = p1.lcm(p2);
        assert_eq!(p3.coefficients, vec![-3., -3., 1./12., 1./12.]);
    }
}
