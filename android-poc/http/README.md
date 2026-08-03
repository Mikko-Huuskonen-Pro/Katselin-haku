# Android PoC 2 — localhost HTTP

Pieni Rust-HTTP-palvelin NDK-targeteille. Stdlib only (ei Meilisearchia, ei actixia).

Kuuntelee `127.0.0.1:17700`, `GET /` → `OK`.

```bash
# WSL
cd android-poc/http
cargo build --release --target x86_64-linux-android
cargo build --release --target aarch64-linux-android
```

Kopioi `target/*/release/libhttp` →
`../Katselin/android/apk/servoapp/src/main/jniLibs/<abi>/libhttp.so`

Abi-kansiot: `x86_64`, `arm64-v8a`.

Testi emulaattorissa:

```bash
adb shell '.../lib/x86_64/libhttp.so' &
adb shell 'toybox wget -O- http://127.0.0.1:17700/'
# odotus: OK
```
