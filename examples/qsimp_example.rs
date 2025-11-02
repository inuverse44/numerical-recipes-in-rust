use numerical_recipes_rs::algorithms::integration::qsimp::Qsimp;

fn main() {
    let qsimp = Qsimp;
    // Let's integrate a function where Simpson's rule is very accurate, like a cubic polynomial.
    let integrand = |x: f64| 3.0 * x.powi(2) - 2.0 * x + 5.0;

    let a = 0.0;
    let b = 1.0;
    let eps = 1.0e-8;

    // Analytical solution: x^3 - x^2 + 5x from 0 to 1 => (1 - 1 + 5) - 0 = 5
    let expected = 5.0;

    println!("Integrating 3x^2 - 2x + 5 from {} to {} with precision {}", a, b, eps);
    println!("Analytical solution: {}", expected);

    match qsimp.integrate(&integrand, a, b, eps) {
        Ok(val) => {
            println!("Result: {}", val);
            println!("Error:  {}", (val - expected).abs());
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
