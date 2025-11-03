use crate::algorithms::integration::trapzd::Trapzd;

pub struct Qtrap;

const MAX_STEPS: u32 = 20;
const MIN_STEPS_FOR_CHECK: u32 = 5;
const DUMMY_PREVIOUS_VALUE: f64 = -1.0e-30;

impl Qtrap {
    pub fn integrate<F: Fn(f64) -> f64>(
        &self, 
        f: &F, 
        a: f64, 
        b: f64, 
        eps: f64
    ) -> crate::core::Result<f64> {
        qtrap(f, a, b, eps)
    }
        
}

pub fn qtrap<F: Fn(f64) -> f64>(
    f: &F, 
    a: f64, 
    b: f64, 
    eps: f64
) -> crate::core::Result<f64> {
    let trapzd = Trapzd;
    let mut olds = DUMMY_PREVIOUS_VALUE; 

    for i in 1..=MAX_STEPS {
        let s = trapzd.integrate(f, a, b, i as usize);
        if i > MIN_STEPS_FOR_CHECK {
            if (s - olds).abs() < eps {
                return Ok(s);
            }
        }
        olds = s;
    }
    
    Err(crate::core::errors::NumericalError::ConvergenceError { context: "qtrap::integrate".to_string() })
}