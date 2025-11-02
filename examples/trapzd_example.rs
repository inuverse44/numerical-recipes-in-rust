use numerical_recipes_rs::algorithms::integration::trapzd::Trapzd;

fn main() {
    let trapzd = Trapzd;
    let integrand = |x: f64| x.powi(3); // f(x) = x^3

    let a = 0.0;
    let b = 1.0;
    let expected = 0.25; // integral of x^3 from 0 to 1 is 1/4

    println!("Integrating x^3 from {} to {}. Analytical solution: {}", a, b, expected);
    println!("{:<5} {:<25} {:<25}", "N", "Result", "Error");

    for n in 1..=10 {
        let result = trapzd.integrate(&integrand, a, b, n);
        let error = (result - expected).abs();
        println!("{:<5} {:<25} {:<25}", n, result, error);
    }
}
