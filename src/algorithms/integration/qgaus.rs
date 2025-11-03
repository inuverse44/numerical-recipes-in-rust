pub struct Qgaus;

impl Qgaus {
    pub fn integrate<F: Fn(f64) -> f64>(
        &self,
        f: &F,
        a: f64,
        b: f64,
        eps: f64,
    ) -> crate::core::Result<f64> {
        qgaus(f, a, b, eps)
    }
}

pub fn qgaus<F: Fn(f64) -> f64>(
    f: &F,
    a: f64,
    b: f64,
    _eps: f64,
) -> crate::core::Result<f64> {
    if a == b {
        return Ok(0.0);
    }

    const X: [f64; 5] = [
        0.148_874_338_9,
        0.433_395_394_1,
        0.679_409_568_2,
        0.865_063_366_6,
        0.973_906_528_5,
    ];
    const W: [f64; 5] = [
        0.295_524_224_7,
        0.269_266_719_3,
        0.219_086_362_5,
        0.149_451_349_1,
        0.066_671_344_3,
    ];

    let xm = 0.5 * (b + a);
    let xr = 0.5 * (b - a);
    let mut s = 0.0;
    for i in 0..5 {
        let dx = xr * X[i];
        s += W[i] * (f(xm + dx) + f(xm - dx));
    }
    Ok(s * xr)
}
