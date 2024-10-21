use rand::Rng;
use rlwe::RLWE;
use rq::Rq;

mod rlwe;
mod rq;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    let n = 8; // power of 2
    let q = 67108289; // prime number, q = 1 (mod 2n)
    let t = 37; // prime number t < q
    let std_ = 3.; // standard deviation of the gaussian distribution

    let rlwe = RLWE::new(n, q, t, std_);
    let (sk, pk) = rlwe.generate_keys();

    // plaintexts
    let m0 = {
        let coeffs = vec![rng.gen_range(t..q); n];
        Rq::new(coeffs, t)
    };
    let m1 = {
        let coeffs = vec![rng.gen_range(t..q); n];
        Rq::new(coeffs, t)
    };

    let c0 = rlwe.encrypt(m0.clone(), pk.clone());
    let c1 = rlwe.encrypt(m1.clone(), pk.clone());

    let m_0 = rlwe.decrypt(vec![c0.0.clone(), c0.1.clone()], sk.clone());
    let m_1 = rlwe.decrypt(vec![c1.0.clone(), c1.1.clone()], sk.clone());

    println!("m0: {}", m0);
    println!("m_0: {}", m_0);
    
    println!("m1: {}", m1);
    println!("m_1: {}", m_1);
}
