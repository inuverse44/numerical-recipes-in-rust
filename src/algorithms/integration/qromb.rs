use crate::algorithms::integration::trapzd::Trapzd;
use crate::algorithms::interpolation_and_extrapolation::polint;
use nalgebra::DVector;

const MAX_STEPS: u32 = 20; // Corresponds to JMAX in NR
const K: usize = 5; // Number of points used for Romberg (polynomial) extrapolation

/// Romberg integration using polynomial extrapolation of the trapezoidal rule sequence.
///
/// - Builds the trapezoidal estimates S_j = trapzd(f, a, b, j)
/// - Forms the sequence of step-size parameters h_j = 1, 1/4, 1/4^2, ...
/// - Applies polynomial extrapolation (Neville) on the last K points to x=0
/// - Stops when the estimated change |dy| < eps, otherwise errors after MAX_STEPS
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
        // Trapezoidal estimate at level j
        let sj = trapzd.integrate(f, a, b, j);
        s_vals.push(sj);

        // h sequence: 1, 1/4, 1/4^2, ...
        if j == 1 {
            h_vals.push(1.0);
        } else {
            h_vals.push(h_vals[j - 2] * 0.25);
        }

        // Once we have at least K points, extrapolate last K to h=0
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
