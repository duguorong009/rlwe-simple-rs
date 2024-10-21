use rand::Rng;

use crate::rq::Rq;


fn discrete_gaussian(n: usize, q: i64, std_: f64) -> Rq {
    let mut rng = rand::thread_rng();
    let mut coeffs = vec![0; n];
    for i in 0..n {
        coeffs[i] = (rng.gen::<f64>() * std_).round() as i64;
    }
    Rq::new(coeffs, q)
}