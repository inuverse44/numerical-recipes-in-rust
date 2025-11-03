use numerical_recipes_rs::algorithms::interpolation_and_extrapolation::polint::polint;
use nalgebra::DVector;

fn main() {
    let xv = DVector::from_vec(vec![1.0, 2.0, 3.0, 4.0]);
    let yv = DVector::from_vec(vec![2.0, 3.0, 5.0, 4.0]);
    match polint(&xv, &yv, 2.5) {
        Ok((y, dy)) => {
            println!("Interpolated value: {}, Estimated error: {}", y, dy);
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
