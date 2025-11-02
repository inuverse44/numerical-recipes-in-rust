use crate::core::traits::ODE;
use nalgebra::DVector;

pub struct Rk4;

impl Rk4 {
    pub fn step<Eq: ODE>(&self, eq: &Eq, t: f64, y: &DVector<f64>, h: f64) -> DVector<f64> {
        let k1 = eq.rhs(t, y);
        let k2 = eq.rhs(t + 0.5 * h, &(y + &k1 * (0.5 * h)));
        let k3 = eq.rhs(t + 0.5 * h, &(y + &k2 * (0.5 * h)));
        let k4 = eq.rhs(t + h, &(y + &k3 * h));
        y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
    }
}