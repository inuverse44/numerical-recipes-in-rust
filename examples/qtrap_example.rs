use nrir::algorithms::integration::qtrap::Qtrap;

fn main() {
    let qtrap = Qtrap;
    let integrand = |x: f64| x.powi(3); // f(x) = x^3
    let a = 0.0;
    let b = 1.0;
    let expected = 0.25;

    println!("--- Case 1: Successful Integration ---\n");
    let eps_success = 1.0e-6;
    println!(
        "Integrating x^3 from {} to {} with precision {}",
        a, b, eps_success
    );
    println!("Analytical solution: {}", expected);

    match qtrap.integrate(&integrand, a, b, eps_success) {
        Ok(val) => {
            println!("Result: {}", val);
            println!("Error:  {}", (val - expected).abs());
        }
        Err(e) => {
            println!("An unexpected error occurred: {}", e);
        }
    }

    println!("\n--- Case 2: Deliberate Convergence Failure ---\n");
    let eps_fail = 1.0e-18;
    println!(
        "Integrating x^3 from {} to {} with precision {}",
        a, b, eps_fail
    );
    println!("(This is expected to fail to demonstrate error handling)");

    match qtrap.integrate(&integrand, a, b, eps_fail) {
        Ok(val) => {
            println!("Unexpected success!");
            println!("Result: {}", val);
        }
        Err(e) => {
            println!("An expected error occurred: {}", e);
        }
    }
}