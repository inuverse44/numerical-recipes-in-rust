use nalgebra::DVector;

pub trait ODE {
    fn rhs(&self, t: f64, y: &DVector<f64>) -> DVector<f64>;
}

pub trait Integrator {
    fn integrate(&self, a: f64, b: f64, n: usize) -> f64;
    fn integrand(&self, x: f64) -> f64;
}   