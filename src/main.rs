mod gaussian_number;
mod polynomials;

fn main() {
    let a = gaussian_number::GaussanNumber::new(3, 4);
    let b = gaussian_number::GaussanNumber::new(1, 3);
    let gcd = a.gcd(&b);
    println!("GCD: {}", gcd);
    let lcm = a.lcm(&b);
    println!("LCM: {}", lcm);
    let p1 = polynomials::Polynomial::new(vec![1., 0., 1.0]);
    let p2 = polynomials::Polynomial::new(vec![1.0, 2., 1.]);
    println!("GCD: {:?}", p1.gcd(p2.clone()));
    println!("LCM: {:?}", p1.lcm(p2));
}
