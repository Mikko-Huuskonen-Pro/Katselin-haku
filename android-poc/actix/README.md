# Android PoC 3 — actix-web + tokio

Testaa Meilisearchin HTTP-pinoa (`actix-web` + `tokio`) Android-bionicissa ennen täyttä meilisearch-käännöstä.

Kuuntelee `127.0.0.1:17701`, `GET /health` → `{"status":"ok"}`.

```bash
# WSL
cd android-poc/actix
cargo build --release --target x86_64-linux-android
cargo build --release --target aarch64-linux-android
```

Kopioi `target/*/release/libactix` →
`Katselin/android/apk/servoapp/src/main/jniLibs/<abi>/libactix.so`

Testi emulaattorissa:

```bash
adb push .../libactix.so /data/local/tmp/libactix.so
adb shell chmod 755 /data/local/tmp/libactix.so
adb shell 'sh -c "/data/local/tmp/libactix.so >/data/local/tmp/poc3.log 2>&1 &"'
adb shell "echo -e 'GET /health HTTP/1.0\r\n\r\n' | toybox nc 127.0.0.1 17701"
adb shell cat /data/local/tmp/poc3.log
```
