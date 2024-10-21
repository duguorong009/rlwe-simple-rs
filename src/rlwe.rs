use rand::Rng;

use crate::rq::Rq;

pub struct RLWE {
    n: usize,
    p: i64,
    t: usize,
    std_: f64,
}

impl RLWE {
    pub fn new(n: usize, p: i64, t: usize, std_: f64) -> RLWE {
        let m = (n as f64).log2().round() as usize;
        assert!(m.pow(2) == n, "n must be a power of 2");

        RLWE { n, p, t, std_ }
    }
}

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
