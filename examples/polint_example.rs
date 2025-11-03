use nrir::algorithms::interpolation_and_extrapolation::polint::polint;
use nalgebra::DVector;

fn main() {
    // 固定長データなので from_row_slice を使用すると簡潔
    let xv = DVector::from_row_slice(&[1.0, 2.0, 3.0, 4.0]);
    let yv = DVector::from_row_slice(&[2.0, 3.0, 5.0, 4.0]);

    println!("Expected (manual check): 3.625");

    match polint(&xv, &yv, 2.5) {
        Ok((y, dy)) => {
            // 注意: dy は Neville 法の最終補正量（推定誤差）で、符号は負になる場合もあります。
            println!("Interpolated value: {}, Estimated error: {}", y, dy);
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
