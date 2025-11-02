use crate::algorithms::integration::trapzd::Trapzd;

pub struct Qromb;

const MAX_STEPS: u32 = 20;
const MAX_STEPS_P: u32 = MAX_STEPS + 1;
const MIN_STEPS_FOR_CHECK: u32 = 5;
const DUMMY_PREVIOUS_VALUE: f64 = -1.0e-30;

impl Qromb {
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: &F, a: f64, b: f64, eps: f64) -> crate::core::Result<f64> {
        // polintが必要
        unimplemented!()
    }
}