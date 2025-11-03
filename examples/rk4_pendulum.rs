use nalgebra::DVector;
use nrir::core::traits::ODE;
use nrir::algorithms::ode::rk4::Rk4;

pub struct Pendulum {
    g: f64,
    l: f64,
}

impl ODE for Pendulum {
    fn rhs(&self, _t: f64, y: &DVector<f64>) -> DVector<f64> {
        let theta = y[0];
        let omega = y[1];
        DVector::from_vec(vec![omega, -self.g / self.l * theta.sin()])
    }
}

fn main() {
    let pendulum = Pendulum { g: 9.81, l: 1.0 };
    let initial_state = DVector::from_vec(vec![0.0, 1.0]);
    let rk4 = Rk4;

    let t_i = 0.0;
    let t_f = 10.0;
    let n = 100;
    let h = (t_f - t_i) / n as f64;

    let mut current_state = initial_state.clone();

    for i in 0..n {
        let t = t_i + i as f64 * h;
        let new_state = rk4.step(&pendulum, t, &current_state, h);
        println!("t: {:.4}, theta: {:.4}, omega: {:.4}", t, new_state[0], new_state[1]);
        current_state = new_state;
    }
}