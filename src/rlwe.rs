use rand::Rng;

use crate::rq::{pow_rq, Rq};

pub struct RLWE {
    n: usize,
    p: i64,
    t: i64,
    std_: f64,
}

impl RLWE {
    pub fn new(n: usize, p: i64, t: i64, std_: f64) -> RLWE {
        let m = (n as f64).log2().round() as usize;
        assert!(m.pow(2) == n, "n must be a power of 2");

        RLWE { n, p, t, std_ }
    }

    pub fn generate_keys(&self) -> (Rq, (Rq, Rq)) {
        let s = discrete_gaussian(self.n, self.p, self.std_);
        let e = discrete_gaussian(self.n, self.p, self.std_);

        let a1 = discrete_uniform(self.n, self.p, None, None);
        let a0 = (a1.clone() * s.clone() + e * self.t) * -1;

        (s, (a0, a1))
    }

    pub fn encrypt(&self, m: Rq, a: Vec<Rq>) -> (Rq, Rq) {
        assert!(a.len() == 2);
        let a0 = a[0].clone();
        let a1 = a[1].clone();

        let mut e = vec![];
        for _ in 0..3 {
            e.push(discrete_gaussian(self.n, self.p, self.std_));
        }

        let m = Rq::new(m.poly.data().to_vec(), self.p);

        (m + a0 * e[0].clone() + e[2].clone() * self.t, a1 * e[0].clone() + e[1].clone() * self.t)
    }

    pub fn decrypt(&self, c: Vec<Rq>, s: Rq) -> Rq {
        let c: Vec<Rq> = c.into_iter().enumerate().map(|(i, ci)| ci * pow_rq(&s, i)).collect();

        let mut m = c[0].clone();
        for i in 1..c.len() {
            m = m + c[i].clone();
        }

        m = Rq::new(m.poly.data().to_vec(), self.t);

        m
    }

    pub fn add(&self, c0: Vec<Rq>, c1: Vec<Rq>) -> Vec<Rq> {
        let mut c: Vec<Rq> = vec![];

        let k0 = c0.len();
        let k1 = c1.len();

        let (mut c0, c1) = if k0 > k1 {
            (c1, c0)
        } else {
            (c0, c1)
        };

        for _ in 0..k0.abs_diff(k1) {
            c0.push(Rq::new(vec![0], self.p));
        }

        for i in 0..c0.len() {
            c.push(c0[i].clone() + c1[i].clone());
        }

        c
    }

    pub fn mul(&self, mut c0: Vec<Rq>, mut c1: Vec<Rq>) -> Vec<Rq> {
        let mut c: Vec<Rq> = vec![];

        let k0 = c0.len() - 1;
        let k1 = c1.len() - 1;

        for _ in 0..k1 {
            c0.push(Rq::new(vec![0], self.p));
        }

        for _ in 0..k0 {
            c1.push(Rq::new(vec![0], self.p));
        }

        for i in 0..k0 + k1 + 1 {
            let mut c_ = Rq::new(vec![0], self.p);
            for j in 0..i + 1 {
                c_ = c_ + c0[j].clone() * c1[i - j].clone();
            }
            c.push(c_);
        }

        c
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
