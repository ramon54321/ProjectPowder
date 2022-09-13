<div align="center">
  <span><img src="./docs/powder_logo.svg" width="50%"></span>
</div>

### Powder

Lightweight wrapper over [Femtovg](https://github.com/femtovg/femtovg) and [Glutin](https://github.com/rust-windowing/glutin) with an immediate style drawing API.

#### Getting Started

The examples can be run as follows.

```rust
cd powder
cargo run --example hello_world
```

You will also need to depend on `femtovg` and `nalgebra-glm`.

```toml
[dependencies]
femtovg = "0.3.5"
nalgebra-glm = "0.17.0"
```

