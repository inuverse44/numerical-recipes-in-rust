use nrir::algorithms::integration::qromb::qromb;

fn main() {
    // Integrate sin(x) from 0 to pi. Exact value is 2.
    let integrand = |x: f64| x.sin();
    let a = 0.0;
    let b = std::f64::consts::PI;
    let eps = 1.0e-8;
    let expected = 2.0;

    println!(
        "Integrating sin(x) from {} to {} with precision {}",
        a, b, eps
    );
    println!("Analytical solution: {}", expected);

    match qromb(&integrand, a, b, eps) {
        Ok(val) => {
            println!("Result: {}", val);
            println!("Error:  {}", (val - expected).abs());
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
