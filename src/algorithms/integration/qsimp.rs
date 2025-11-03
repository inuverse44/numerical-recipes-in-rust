use crate::algorithms::integration::trapzd::Trapzd;

pub struct Qsimp;

const MAX_STEPS: u32 = 20;
const MIN_STEPS_FOR_CHECK: u32 = 5;
const DUMMY_PREVIOUS_VALUE: f64 = -1.0e-30;

impl Qsimp {
    pub fn integrate<F: Fn(f64) -> f64>(
        &self,
        f: &F,
        a: f64,
        b: f64,
        eps: f64,
    ) -> crate::core::Result<f64> {
        qsimp(f, a, b, eps)
    }
}

pub fn qsimp<F: Fn(f64) -> f64>(
    f: &F,
    a: f64,
    b: f64,
    eps: f64,
) -> crate::core::Result<f64> {
    let trapzd = Trapzd;
    let mut os = DUMMY_PREVIOUS_VALUE; 
    let mut ost = DUMMY_PREVIOUS_VALUE; 

    for i in 1..=MAX_STEPS {
        let st = trapzd.integrate(f, a, b, i as usize);
        let s = (4.0 * st - ost) / 3.0;
        if i > MIN_STEPS_FOR_CHECK {
            if (s - os).abs() < eps { // Using absolute error check
                return Ok(s);
            }
        }
        os = s;
        ost = st;
    }
    
    Err(crate::core::errors::NumericalError::ConvergenceError { context: "qsimp::integrate".to_string() })
}
