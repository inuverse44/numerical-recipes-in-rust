use nrir::algorithms::integration::qromb::{qromb, Qromb};

fn main() {
    // Integrate exp(-x^2) from 0 to 1.
    // Exact value is (sqrt(pi)/2) * erf(1) ≈ 0.7468241328124271
    let integrand = |x: f64| (-x * x).exp();
    let a = 0.0;
    let b = 1.0;
    let eps = 1.0e-8;
    let expected = 0.746_824_132_812_427_1_f64;

    println!(
        "Integrating exp(-x^2) from {} to {} with precision {}",
        a, b, eps
    );
    println!("Reference value (≈): {}", expected);

    println!("\n-- Function API --");
    match qromb(&integrand, a, b, eps) {
        Ok(val) => {
            println!("Result: {}", val);
            println!("Error:  {}", (val - expected).abs());
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }

    println!("\n-- Struct API (Qromb) --");
    let q = Qromb;
    match q.integrate(&integrand, a, b, eps) {
        Ok(val) => {
            println!("Result: {}", val);
            println!("Error:  {}", (val - expected).abs());
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
