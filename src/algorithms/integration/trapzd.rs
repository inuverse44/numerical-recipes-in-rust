pub struct Trapzd;

impl Trapzd {
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: &F, a: f64, b: f64, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }

        // First step (n=1)
        let mut s = 0.5 * (b - a) * (f(a) + f(b));

        if n == 1 {
            return s;
        }

        // Iterative refinement for n > 1
        for i in 2..=n {
            let it = 1 << (i - 2);
            let tnm = it as f64;
            let del = (b - a) / tnm;
            let sum: f64 = (0..it)
                .map(|j| f(a + (j as f64 + 0.5) * del))
                .sum();
            s = 0.5 * (s + (b - a) * sum / tnm);
        }
        s
    }
}