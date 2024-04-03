mod gaussian_number;
mod polynomials;

fn main() {
    let a = gaussian_number::GaussanNumber::new(3, 4);
    let b = gaussian_number::GaussanNumber::new(1, 3);
    let gcd = a.gcd(&b);
    println!("GCD: {}", gcd);
    let lcm = a.lcm(&b);
    println!("LCM: {}", lcm);
    let p1 = polynomials::Polynomial::new(vec![1., 0., 1.]);
    let p2 = polynomials::Polynomial::new(vec![1., 2., 1.]);
    println!("GCD: {:?}", p1.gcd(p2.clone()));
    println!("LCM: {:?}", p1.lcm(p2));
    let p3 = polynomials::Polynomial::new(vec![1., 0., 1., 0., 1.]);
    let p4 = polynomials::Polynomial::new(vec![-1., -2., -1., 0., 1.]);
    let p5 = polynomials::Polynomial::new(vec![-1., 0., 0., 1.]);
    println!("folded GCD v1: {:?}", polynomials::Polynomial::gdc_all(&vec![p3, p4, p5]));
    let p6 = polynomials::Polynomial::new(vec![-4., -4., 1., 1.]);
    let p7 = polynomials::Polynomial::new(vec![4., -4., -1., 1.]);
    let p8 = polynomials::Polynomial::new(vec![2., -1., -2., 1.]);
    println!("folded GCD v2: {:?}", polynomials::Polynomial::gdc_all(&vec![p6, p7, p8]));
}
