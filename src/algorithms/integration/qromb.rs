use crate::algorithms::integration::trapzd::Trapzd;
use crate::algorithms::interpolation_and_extrapolation::polint;
use nalgebra::DVector;

const MAX_STEPS: u32 = 20;
const K: usize = 5;

pub struct Qromb;

impl Qromb {
    pub fn integrate<F: Fn(f64) -> f64>(
        &self,
        f: &F,
        a: f64,
        b: f64,
        eps: f64,
    ) -> crate::core::Result<f64> {
        qromb(f, a, b, eps)
    }
}

pub fn qromb<F: Fn(f64) -> f64>(
    f: &F,
    a: f64,
    b: f64,
    eps: f64,
) -> crate::core::Result<f64> {
    let trapzd = Trapzd;
    let jmax = MAX_STEPS as usize;

    let mut s_vals: Vec<f64> = Vec::with_capacity(jmax);
    let mut h_vals: Vec<f64> = Vec::with_capacity(jmax);

    for j in 1..=jmax {
        let sj = trapzd.integrate(f, a, b, j);
        s_vals.push(sj);

        if j == 1 {
            h_vals.push(1.0);
        } else {
            h_vals.push(h_vals[j - 2] * 0.25);
        }
        
        if j >= K {
            let start = j - K;
            let xv = DVector::from_iterator(K, h_vals[start..j].iter().cloned());
            let yv = DVector::from_iterator(K, s_vals[start..j].iter().cloned());

            let (ss, dss) = polint::polint(&xv, &yv, 0.0)?;
            if dss.abs() < eps {
                return Ok(ss);
            }
        }
    }

    Err(crate::core::errors::NumericalError::ConvergenceError {
        context: "qromb::integrate".to_string(),
    })
}
