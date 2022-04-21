# Lightweight Rust WASM Template

Write Rust stuff in `src/lib.rs`, write web stuff in `www`.

## Building
```bash
$ wasm-pack build --target web --out-dir www/pkg
```

## Serving
```bash
$ npx serve www
```