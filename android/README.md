# Android NDK build (M1)

Builds Meilisearch as a PIE binary for bionic (`x86_64-linux-android`, `aarch64-linux-android`).

```bash
# WSL
./android/build-android.sh x86_64   # emulator first
./android/build-android.sh both
```

Feature profile: `--no-default-features` (latin tokenization only — skip `all-tokenizations` / lindera).

Do **not** enable `lmdb-posix-sem` (PoC 4a: `sem_open` → ENOSYS).

After build:

```bash
./android/copy-jniLibs.sh
```

Runtime on device requires a **writable cwd** and explicit `--db-path` / `--dump-dir` / `--snapshot-dir`.
