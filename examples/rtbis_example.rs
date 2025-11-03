use nrir::algorithms::root_finding_and_nonlinear_sets_of_equation::rtbis::{rtbis, Rtbis};

fn main() -> nrir::core::Result<()> {
    let f = |x: f64| x * x - 2.0;
    let a = 0.0;
    let b = 2.0;
    let eps = 1.0e-8;
    let expected = 2.0_f64.sqrt();

    println!("-- Function API --");
    let root_fn = rtbis(&f, a, b, eps)?;
    println!("Root:   {}", root_fn);
    println!("Target: {}", expected);

    println!("\n-- Struct API (Rtbis) --");
    let solver = Rtbis;
    let root_struct = solver.solve(&f, a, b, eps)?;
    println!("Root:   {}", root_struct);
    println!("Target: {}", expected);

    Ok(())
}
