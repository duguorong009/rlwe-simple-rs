use std::ops::{Add, Mul};

use polynomial::Polynomial;

/// Ring-Polynomial: Fq[x] / (x ^ n + 1)
/// range of the reminder is set to (-q/2, q/2)
#[derive(Clone, Debug)]
pub struct Rq {
    f: Polynomial<i64>,
    q: i64,                           // modulus
    pub(crate) poly: Polynomial<i64>, // coefficients
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
        let f = Polynomial::new(f);

        let coeffs = coeffs
            .into_iter()
            .map(|i| (i % q + q) % q)
            .collect::<Vec<i64>>();
        let coeffs = crange(coeffs, q);
        let poly = Polynomial::new(coeffs);

        Rq { f, q, poly }
    }
}

impl std::fmt::Display for Rq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Rq: {} (mod {}), reminder range: ({}, {})",
            self.poly.pretty("x"),
            self.q,
            -self.q / 2,
            self.q / 2
        )
    }
}

impl Add for Rq {
    type Output = Rq;

    fn add(self, other: Rq) -> Rq {
        let poly = self.poly + other.poly;
        Rq::new(poly.data().to_vec(), self.q)
    }
}

impl Mul for Rq {
    type Output = Rq;

    fn mul(self, other: Rq) -> Rq {
        let poly = self.poly * other.poly;
        let (_, r) = poly_div(&poly, &self.f);
        Rq::new(r.data().to_vec(), self.q)
    }
}

impl Mul<i64> for Rq {
    type Output = Rq;

    fn mul(self, other: i64) -> Rq {
        let coeffs = self
            .poly
            .data()
            .to_vec()
            .into_iter()
            .map(|i| i * other)
            .collect();
        Rq::new(coeffs, self.q)
    }
}

pub(crate) fn pow_rq(x: &Rq, n: usize) -> Rq {
    if n == 0 {
        return Rq::new(vec![1], x.q);
    }
    let mut ret = x.clone();
    for _ in 1..n {
        ret = ret * x.clone();
    }
    ret
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

fn poly_div(
    dividend: &Polynomial<i64>,
    divisor: &Polynomial<i64>,
) -> (Polynomial<i64>, Polynomial<i64>) {
    let mut dividend = dividend.data().to_vec().clone(); // We will modify the dividend
    let divisor = divisor.data().to_vec().clone();

    if dividend.len() < divisor.len() {
        return (Polynomial::new(vec![0]), Polynomial::new(dividend));
    }

    let mut quotient = vec![0; dividend.len() - divisor.len() + 1];

    // Perform division until degree of the dividend is less than divisor
    while dividend.len() >= divisor.len() {
        // Leading term of the quotient
        let lead_coeff = dividend[dividend.len() - 1] / divisor[divisor.len() - 1];
        let degree_diff = dividend.len() - divisor.len();

        // Update the quotient with the leading term
        quotient[degree_diff] = lead_coeff;

        // Subtract (divisor * lead_coeff * x^degree_diff) from dividend
        for i in 0..divisor.len() {
            dividend[degree_diff + i] -= lead_coeff * divisor[i];
        }

        // Remove the last (leading) term of dividend if it is zero
        dividend.pop();
    }

    let quotient = Polynomial::new(quotient);
    let dividend = Polynomial::new(dividend);
    (quotient, dividend) // Quotient and remainder
}

#[test]
fn test_poly_div() {
    // (x^2 - 2x + 1) / (x - 1) = (x - 1, 0)
    let dividend = Polynomial::new(vec![1, -2, 1]);
    let divisor = Polynomial::new(vec![-1, 1]);
    let (q, r) = poly_div(&dividend, &divisor);
    assert_eq!(q, Polynomial::new(vec![-1, 1]));
    assert_eq!(r, Polynomial::new(vec![0]));

    // (2 x^3 + 3 x^2 + 4x + 3) / (x + 1) = (2 x^2 + x + 3, 0)
    let dividend = Polynomial::new(vec![3, 4, 3, 2]);
    let divisor = Polynomial::new(vec![1, 1]);
    let (q, r) = poly_div(&dividend, &divisor);
    assert_eq!(q, Polynomial::new(vec![3, 1, 2]));
    assert_eq!(r, Polynomial::new(vec![0]));

    let dividend = Polynomial::new(vec![2]);
    let divisor = Polynomial::new(vec![1, 1]);
    let (q, r) = poly_div(&dividend, &divisor);
    assert_eq!(q, Polynomial::new(vec![0]));
    assert_eq!(r, Polynomial::new(vec![2]));
}
