const MAX_STEPS: u32 = 40;

pub struct Rtbis;

impl Rtbis {
    pub fn solve<F: Fn(f64) -> f64>(
        &self,
        f: &F,
        x1: f64,
        x2: f64,
        eps: f64,
    ) -> crate::core::Result<f64> {
        rtbis(f, x1, x2, eps)
    }
}

/// Bisection method for root finding on a bracketing interval [x1, x2].
/// Requires f(x1) and f(x2) to have opposite signs.
pub fn rtbis<F: Fn(f64) -> f64>(
    f: &F,
    x1: f64,
    x2: f64,
    eps: f64,
) -> crate::core::Result<f64> {
    if eps <= 0.0 {
        return Err(crate::core::errors::NumericalError::InvalidArgument(
            "eps must be positive".to_string(),
        ));
    }

    let mut fa = f(x1);
    let mut fb = f(x2);
    if fa.is_nan() || fb.is_nan() {
        return Err(crate::core::errors::NumericalError::InvalidArgument(
            "f returned NaN at endpoints".to_string(),
        ));
    }
    if fa == 0.0 {
        return Ok(x1);
    }
    if fb == 0.0 {
        return Ok(x2);
    }
    if fa * fb >= 0.0 {
        return Err(crate::core::errors::NumericalError::RootError {
            context: "rtbis: f(x1) and f(x2) must have opposite signs".to_string(),
        });
    }

    // Ensure rtb is the endpoint where f is negative; dx points toward the other end.
    let mut dx;
    let mut rtb;
    if fa < 0.0 {
        dx = x2 - x1;
        rtb = x1;
    } else {
        dx = x1 - x2;
        rtb = x2;
        // swap so that fa corresponds to rtb and is negative
        std::mem::swap(&mut fa, &mut fb);
    }

    for _ in 0..MAX_STEPS {
        dx *= 0.5;
        let xmid = rtb + dx; // midpoint between rtb and the other endpoint
        let fmid = f(xmid);
        if fmid.is_nan() {
            return Err(crate::core::errors::NumericalError::InvalidArgument(
                "f returned NaN".to_string(),
            ));
        }
        if fmid <= 0.0 {
            rtb = xmid;
        }
        // Converged if half-interval < eps or exact root
        if dx.abs() < eps || fmid == 0.0 {
            return Ok(rtb);
        }
    }

    Err(crate::core::errors::NumericalError::ConvergenceError {
        context: "rtbis::solve".to_string(),
    })
}
