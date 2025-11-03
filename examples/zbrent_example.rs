use nrir::algorithms::root_finding_and_nonlinear_sets_of_equation::zbrent::{zbrent, Zbrent};

fn main() -> nrir::core::Result<()> {
    // Solve cos(x) - x = 0 on [0, 1]
    let f = |x: f64| x.cos() - x;
    let a = 0.0;
    let b = 1.0;
    let eps = 1.0e-10;
    let expected = 0.739_085_133_215_160_7_f64; // known root

    println!("-- Function API --");
    let root_fn = zbrent(&f, a, b, eps)?;
    println!("Root:   {}", root_fn);
    println!("Target: {}", expected);

    println!("\n-- Struct API (Zbrent) --");
    let solver = Zbrent;
    let root_struct = solver.solve(&f, a, b, eps)?;
    println!("Root:   {}", root_struct);
    println!("Target: {}", expected);

    Ok(())
}

