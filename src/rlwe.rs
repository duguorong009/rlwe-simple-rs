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

fn discrete_uniform(n: usize, q: i64, min: Option<i64>, max: Option<i64>) -> Rq {
    let mut rng = rand::thread_rng();

    let min = min.unwrap_or(0);
    let max = max.unwrap_or(q);

    let mut coeffs = vec![0; n];
    for i in 0..n {
        coeffs[i] = rng.gen_range(min..=max);
    }
    Rq::new(coeffs, q)
}
