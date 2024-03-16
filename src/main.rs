mod gaussian_number;

fn main() {
    let a = gaussian_number::GaussanNumber::new(3, 4);
    let b = gaussian_number::GaussanNumber::new(1, 3);
    let gcd = a.gcd(&b);
    println!("GCD: {}", gcd);
    let lcm = a.lcm(&b);
    println!("LCM: {}", lcm);
}
