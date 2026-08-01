# Katselin-haku

**Katselin-haku** is an Android-focused fork of Meilisearch.

The goal of this project is to make Meilisearch run natively on Android by removing Linux-specific assumptions and adding Android NDK support. The long-term vision is to provide a fast, local, privacy-respecting search engine for Android applications.

## Why?

Meilisearch is an excellent search engine written primarily in Rust. However, its official binaries target Linux environments using glibc and cannot run directly on Android, which uses the bionic libc.

This project explores what is needed to make Meilisearch work as a native Android library.

## Goals

- Android NDK support
- Native Rust implementation
- Minimal platform-specific code
- Fast local indexing
- Fast full-text search
- Compatibility with Meilisearch features whenever practical

## Use Cases

- Android browsers
- Offline search
- Document search
- Embedded applications
- Local knowledge bases
- Privacy-first applications

## Origin

Katselin-haku was started as part of the **Katselin** browser project.

Katselin is a Servo-based browser that aims to provide a privacy-focused and European alternative for Android. It requires a fast local search engine for curated web content, making Android support for Meilisearch an important building block.

Although motivated by Katselin, this project is intended to become a general-purpose Android port that can benefit the wider open source community.

## Roadmap

- [ ] Build Meilisearch with Android NDK
- [ ] Remove Linux-only assumptions
- [ ] Create Android-compatible build pipeline
- [ ] Expose a library API for Android apps
- [ ] Optimize indexing and search performance on mobile devices

## Current Status

🚧 Early development.

The project currently focuses on understanding Meilisearch's platform dependencies and making the core engine compile and run on Android.

## Contributing

Contributions, testing, and ideas are welcome.

If you are interested in Rust, Android NDK, search engines, or embedded systems, feel free to open an issue or submit a pull request.

## License

This project is based on Meilisearch and follows the licensing terms of the original project. See the LICENSE file for details.

## Acknowledgements

Many thanks to the Meilisearch team for creating an excellent open source search engine. This project aims to extend its reach to Android devices.