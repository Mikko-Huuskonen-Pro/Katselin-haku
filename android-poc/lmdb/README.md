# Android PoC 4a — heed / LMDB (posix-sem)

Testaa millin LMDB-pinoa Android-bionicissa ennen täyttä meilisearch-käännöstä.

- heed `0.22.1`, `default-features=false`, `serde-json` — **ei** `posix-sem` (Android: POSIX mutex)
- Käynnistyessä: write → read → reopen → read
- HTTP `127.0.0.1:17702`: `/health`, `/put?key=&value=`, `/get?key=`

```bash
# WSL — NDK CC/AR tarvitaan lmdb-master-sys C-käännökseen
export NDK=/home/gigli/Android/Sdk/ndk/28.2.13676358
export TOOLCHAIN=$NDK/toolchains/llvm/prebuilt/linux-x86_64
export CC_x86_64_linux_android=$TOOLCHAIN/bin/x86_64-linux-android29-clang
export AR_x86_64_linux_android=$TOOLCHAIN/bin/llvm-a
export CC_aarch64_linux_android=$TOOLCHAIN/bin/aarch64-linux-android29-clang
export AR_aarch64_linux_android=$TOOLCHAIN/bin/llvm-a

cd android-poc/lmdb
cargo build --release --target x86_64-linux-android
cargo build --release --target aarch64-linux-android
```

Testi:

```bash
adb push .../liblmdb.so /data/local/tmp/liblmdb.so
adb shell chmod 755 /data/local/tmp/liblmdb.so
adb shell 'sh -c "/data/local/tmp/liblmdb.so >/data/local/tmp/poc4a.log 2>&1 &"'
adb shell cat /data/local/tmp/poc4a.log
adb shell "echo -e 'GET /put?key=k&value=v HTTP/1.0\r\n\r\n' | toybox nc 127.0.0.1 17702"
adb shell "echo -e 'GET /get?key=k HTTP/1.0\r\n\r\n' | toybox nc 127.0.0.1 17702"
```
