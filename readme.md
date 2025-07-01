# minargs

🚀 **Minargs** is a blazing-fast, ultra-lightweight argument parser for Rust — zero dependencies, zero bloat.

---

## ✨ Features

- ✅ Simple, expressive and functional API
- 🧠 Zero dependencies
- ⚡️ Extremely fast parsing logic
- 📦 Tiny binary size

---

## 📚️ Example

```rust
use minargs::App;

fn main() {
    let matches = App::new("mytool")
        .version("0.1.0")
        .about("Ultra-light CLI parser demo")
        .arg("input", |a| a.required(true).help("Path to input file"))
        .arg("verbose", |a| a.short('v').long("verbose").takes_value(false))
        .subcommand("init", |cmd| {
            cmd.about("Initialize a project").arg("path", |a| a.help("Optional path"))
        })
        .parse();

    match matches.subcommand() {
        Some("init") => {
            let sub = matches.sub_matches().unwrap();
            println!("Init path: {:?}", sub.get("path"));
        }
        _ => {
            println!("Input: {}", matches.get("input").unwrap());
            if matches.has("verbose") {
                println!("Verbose mode enabled.");
            }
        }
    }
}
```

---

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
minargs = "0.1.0"
```

Or install via Cargo:

```bash
cargo add minargs
```

---

## 📄 License

Licensed under the [MIT License](./license).

---
