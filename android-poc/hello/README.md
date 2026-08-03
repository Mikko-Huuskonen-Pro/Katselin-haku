# Android PoC 1 — Hello exec

Pieni Rust-binääri NDK-targeteille. Ei Meilisearch-riippuvuuksia.

```bash
# WSL
cd android-poc/hello
cargo build --release --target x86_64-linux-android
cargo build --release --target aarch64-linux-android
```

Kopioi `target/*/release/libhello` →
`../Katselin/android/apk/servoapp/src/main/jniLibs/<abi>/libhello.so`

Abi-kansiot: `x86_64`, `arm64-v8a`.
