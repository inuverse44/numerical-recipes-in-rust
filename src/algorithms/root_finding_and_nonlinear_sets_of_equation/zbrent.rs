const MAX_STEPS: u32 = 100;

pub struct Zbrent;

impl Zbrent {
    pub fn solve<F: Fn(f64) -> f64>(
        &self,
        f: &F,
        x1: f64,
        x2: f64,
        eps: f64,
    ) -> crate::core::Result<f64> {
        zbrent(f, x1, x2, eps)
    }
}

/// Brent法（ブレント法）による 1 変数方程式の解法。
/// 区間 [x1, x2] で f(x1) と f(x2) の符号が異なる（ブラケット）ことが前提。
/// 収束判定は本ライブラリ方針に合わせて「絶対誤差 eps」。
pub fn zbrent<F: Fn(f64) -> f64>(
    f: &F,
    x1: f64,
    x2: f64,
    eps: f64,
) -> crate::core::Result<f64> {
    if eps <= 0.0 {
        return Err(crate::core::errors::NumericalError::InvalidArgument(
            "eps must be positive".to_string(),
        ));
    }

    let mut a = x1;
    let mut b = x2;
    let mut fa = f(a);
    let mut fb = f(b);
    if fa.is_nan() || fb.is_nan() {
        return Err(crate::core::errors::NumericalError::InvalidArgument(
            "f returned NaN at endpoints".to_string(),
        ));
    }
    if fa == 0.0 { return Ok(a); }
    if fb == 0.0 { return Ok(b); }
    if fa * fb > 0.0 {
        return Err(crate::core::errors::NumericalError::RootError { context: "zbrent: endpoints must bracket a root".to_string() });
    }

    // c は b と同じ符号を持たない端点。
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a; // 前回のステップ長
    let mut e = d;     // それ以前のステップ長

    for _ in 0..MAX_STEPS {
        if fb * fc > 0.0 {
            // b と c は反対符号に保つ
            c = a;
            fc = fa;
            d = b - a;
            e = d;
        }

        if fc.abs() < fb.abs() {
            // b に最小の |f| を置く
            a = b; fa = fb;
            b = c; fb = fc;
            c = a; fc = fa;
        }

        // 収束チェック（絶対誤差）
        let tol = eps;
        let m = 0.5 * (c - b);
        if m.abs() < tol || fb == 0.0 {
            return Ok(b);
        }

        // 逆二次/割線ステップの試行
        let mut p;
        let mut q;
        if e.abs() > tol && fa.abs() > fb.abs() {
            // 割線（secant） or 逆二次補間（条件により同式に収束）
            let s = fb / fa;
            if (a - c).abs() < std::f64::EPSILON {
                // 割線法
                p = 2.0 * m * s;
                q = 1.0 - s;
            } else {
                // 逆二次補間
                let q1 = fa / fc;
                let r = fb / fc;
                p = s * (2.0 * m * q1 * (q1 - r) - (b - a) * (r - 1.0));
                q = (q1 - 1.0) * (r - 1.0) * (s - 1.0);
            }
            if p > 0.0 { q = -q; } // p/q が b からの増分になるよう調整
            p = p.abs();

            // ステップの許容性判定（Brent の安全策）
            let cond1 = 2.0 * p < 3.0 * m * q.abs() - (tol * q).abs();
            let cond2 = 2.0 * p < (e * q).abs();
            if cond1 && cond2 {
                // 受理: 補間ステップ
                e = d;
                d = p / q;
            } else {
                // 却下: 二分法
                d = m;
                e = m;
            }
        } else {
            // 小さすぎる場合は二分法
            d = m;
            e = m;
        }

        // ステップが小さすぎる場合の下限
        let step = if d.abs() > tol { d } else { if m > 0.0 { tol } else { -tol } };
        a = b;
        fa = fb;
        b += step;
        fb = f(b);
        if fb.is_nan() {
            return Err(crate::core::errors::NumericalError::InvalidArgument("f returned NaN".to_string()));
        }
    }

    Err(crate::core::errors::NumericalError::ConvergenceError { context: "zbrent::solve".to_string() })
}

