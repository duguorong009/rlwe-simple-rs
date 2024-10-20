
/// Ring-Polynomial: Fq[x] / (x ^ n + 1)
/// range of the reminder is set to (-q/2, q/2)
pub struct Rq {
    f: Vec<i64>,
    q: i64, // modulus
    poly: Vec<i64>, // coefficients [x0, x1, ..., xn]
}

impl Rq {
    /// Create a new Rq
    /// coeffs: coefficients of the polynomial
    /// q: modulus
    pub fn new(coeffs: Vec<i64>, q: i64) -> Rq {
        let n = coeffs.len(); // degree of the polynomial
        
        let mut f = vec![0; n + 1];
        f[0] = 1;
        f[n] = 1;

        let coeffs = coeffs.into_iter().map(|i| i % q).collect::<Vec<i64>>();
        let coeffs = crange(coeffs, q);

        Rq {
            f,
            q,
            poly: coeffs.into_iter().rev().collect(),
        }
    }
}

fn crange(coeffs: Vec<i64>, q: i64) -> Vec<i64> {
    let mut coeffs = coeffs;
    for i in 0..coeffs.len() {
        if !(coeffs[i] >= 0 && coeffs[i] <= q / 2) {
            coeffs[i] -= q;
        }
    }
    coeffs
}
