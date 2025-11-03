# nrir — Numerical Recipes in Rust (WIP)

`nrir` は Numerical Recipes の代表的な数値計算アルゴリズムを Rust で実装したライブラリです。まずはスタンドアロンで実装し、将来的な拡張に備えて一部は ZST 構造体 + メソッドの API も提供します。

- Rust edition: 2024
- 依存: `nalgebra`

## インストール

ローカルのパス依存、もしくは Git 依存で利用できます。クレート名は `nrir` です。

### パス依存（ローカル）
```toml
# other-project/Cargo.toml
[dependencies]
nrir = { path = "../numerical-recipes-in-rust/nrir" }
```

### Git 依存（例）
```toml
# other-project/Cargo.toml
[dependencies]
nrir = { git = "https://github.com/<you>/numerical-recipes-in-rust.git", package = "nrir", tag = "v0.1.0" }
```
- 再現性のため `tag` もしくは `rev`（コミット固定）を推奨します。

## 使い方

### 積分（Romberg, Gauss, ほか）
```rust
use nrir::algorithms::integration::qromb::{qromb, Qromb};

let f = |x: f64| x.sin();
let val = qromb(&f, 0.0, std::f64::consts::PI, 1e-8)?; // 関数API

let val2 = Qromb.integrate(&f, 0.0, std::f64::consts::PI, 1e-8)?; // 構造体API
```

Gauss-Legendre 5点（固定次数）:
```rust
use nrir::algorithms::integration::qgaus::Qgaus;
let v = Qgaus.integrate(&|x| x.sin(), 0.0, std::f64::consts::PI, 1e-8)?;
```

### 根探し（Bisection, Brent）
```rust
use nrir::algorithms::root_finding_and_nonlinear_sets_of_equation::rtbis::{rtbis, Rtbis};
use nrir::algorithms::root_finding_and_nonlinear_sets_of_equation::zbrent::{zbrent, Zbrent};

let g = |x: f64| x.cos() - x;
let r1 = rtbis(&g, 0.0, 1.0, 1e-8)?;       // 二分法（関数API）
let r2 = Zbrent.solve(&g, 0.0, 1.0, 1e-10)?; // Brent法（構造体API）
```

## エラー処理
すべての公開関数/メソッドは `nrir::core::Result<T>` を返します。主なエラー:
- `ConvergenceError`: 収束しなかった
- `RootError`: ブラケット成立せず等
- `InvalidArgument`: `eps <= 0` や `f` が `NaN` を返す等

## 設計メモ
- ZST（ゼロサイズ型）を使った API を併用し、将来の設定項目や統計情報の保持に拡張可能。
- 収束判定は基本的に絶対誤差（0 付近の不安定性を避けるため）。

## ライセンス
TBD（プロジェクト方針に従う）
