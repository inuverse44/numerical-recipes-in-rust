use nalgebra::DVector;

pub fn polint(
    xv: &DVector<f64>, 
    yv: &DVector<f64>, 
    x: f64
) -> crate::core::Result<(f64, f64)>  {
    let n = xv.len();
    if yv.len() != n {
        return Err(crate::core::errors::NumericalError::LengthError { context: "polint".to_string() })
    }

    let mut dif = (x - xv[0]).abs();
    let mut c = yv.clone();
    let mut d = yv.clone();
    let mut n_start: isize = 0; // Changed to isize

    // find xv[i] close to x
    for i in 0..n {
        let dif_tmp = (x - xv[i]).abs();
        if dif_tmp < dif {
            n_start = i as isize; // Cast to isize
            dif = dif_tmp;
        }
    }

    let mut y = yv[n_start as usize]; // Cast back to usize for indexing
    n_start -= 1;

    let mut dy = 0.0;

    // Neville's algorithms
    for m in 1..n {
        for i in 0..(n - m) {
            let ho = xv[i] - x;
            let hp = xv[i + m] - x;
            let w = c[i + 1] - d[i];
            let den = ho - hp;
            if den == 0.0 {
                return Err(crate::core::errors::NumericalError::MultipleRootError { context: "polint".to_string() })
            }
            let den = w / den;
            d[i] = hp * den;
            c[i] = ho * den;
        }

        dy = if 2 * (n_start + 1) < (n as isize - m as isize) { // Cast to isize for comparison
            c[(n_start + 1) as usize] // Cast back to usize for indexing
        } else {
            d[n_start as usize] // Cast back to usize for indexing
        };

        y += dy;
    }

    Ok((y, dy))
}