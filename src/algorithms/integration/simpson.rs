pub struct Simpson;

impl Simpson {
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: &F, a: f64, b: f64, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        if n % 2 != 0 {
            panic!("Number of intervals (n) must be even for Simpson's rule.");
        }

        let h = (b - a) / n as f64;
        let mut sum = f(a) + f(b);

        for i in (1..n).step_by(2) {
            sum += 4.0 * f(a + i as f64 * h);
        }

        for i in (2..n-1).step_by(2) {
            sum += 2.0 * f(a + i as f64 * h);
        }

        sum * h / 3.0
    }
}
