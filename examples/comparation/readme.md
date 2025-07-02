# 📊 Binary Size Comparison: Clap vs Minargs

This example compares the **binary size** of a minimal CLI app built with [`clap`](https://crates.io/crates/clap) vs [`minargs`](https://crates.io/crates/minargs).

---

## 📦 How to Run

```bash
cd examples/comparation

# Build both in release mode
cargo build --release -p minargs_demo
cargo build --release -p clap_demo

# Compare binary sizes
ls -lh target/release/{minargs_demo,clap_demo}
```

---

## 📈 Result (Release Mode, opt-level=z + LTO + strip)

#### to-do

_(Values may vary slightly depending on system and Rust version)_

---

## 🔍 Why the difference?

- `clap` brings in many features and dependencies, including help/usage formatting, derive macros, validators, etc.
- `minargs` is zero-dependency and optimized for minimal runtime overhead.

---
