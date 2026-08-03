# Katselin-haku – Android-portin toteutussuunnitelma

Päivämäärä: 2.8.2026

Päivitetty: 3.8.2026 – **vaihe 3 Meilisearch Android PASS** (spawn + /health + 2382 dokuments indeksi emulaattorissa)

Status: Meilisearch Android -ydin valmis → M4 dump-CDN valinnainen; seuraava työ INTEGRAATIO-SUUNNITELMA (Consent / UX)

---

# Tavoite

Mahdollistaa Meilisearch Community Editionin käyttö Androidissa muuttamatta Katselinin hakuarkkitehtuuria.

AGENT.md määrittää arkkitehtuurin:

- Meilisearch toimii omana prosessinaan.
- Katselin käyttää HTTP API:a.
- Hakumoottoria ei upoteta selaimen prosessiin.

Tämän projektin tarkoitus on saada nykyinen Meilisearch toimimaan Androidissa mahdollisimman pienillä muutoksilla upstreamiin.

---

# Päätavoitteet

Projektilla on kolme tavoitetta.

1. Android-yhteensopiva binääri

Rakentuu NDK:lla ilman glibc-riippuvuutta.

2. Androidissa käynnistyvä palvelin

Prosessi voidaan käynnistää native library -hakemistosta.

3. Täysi HTTP-yhteensopivuus

Kotisataman SearchClient toimii muuttamatta nykyistä API:a.

---

# Kehitysperiaatteet

Projektissa pyritään muuttamaan mahdollisimman vähän Meilisearchia.

Tavoitteena EI ole tehdä Android-versiota Meilisearchista.

Tavoitteena on tehdä Community Editionistä Androidissa käynnistyvä versio.

Jos upstream julkaisee myöhemmin Android-tuen, muutokset voidaan helposti poistaa.

---

# Toteutusvaiheet

## Vaihe 0

Proof of Concept

Tämä vaihe ratkaisee projektin tärkeimmän kysymyksen.

Kysymys:

"Voidaanko Meilisearch käynnistää Androidissa omana prosessinaan?"

Jos vastaus on ei, projekti keskeytetään eikä tehdä suuria muutoksia.

---

## PoC 1

Rakennetaan täysin tyhjä Rust-ohjelma Androidille.

Esimerkiksi

```

fn main() {
println!("Hello Android");
}

```

Paketoidaan se APK:n native library directoryyn nimellä `libhello.so` (jniLibs-kaavio, ks. paketointivaatimukset alla).

Tavoite:

- prosessi käynnistyy
- stdout näkyy logcatissa
- exec onnistuu

Jos tämä epäonnistuu, koko arkkitehtuuri muuttuu. Fallback tällöin: `seed_search`-varahaku rikastetaan (ks. Katselin `android/INTEGRAATIO-SUUNNITELMA.md`, vaihe 3) ja projekti keskeytetään.

### PoC 1 – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| Crate | `Katselin-haku/android-poc/hello` (erillinen `[workspace]`, NDK r28 linkkerit `.cargo/config.toml`) |
| Targetit | `x86_64-linux-android`, `aarch64-linux-android` — PIE ELF, linker `/system/bin/linker64` |
| jniLibs | `Katselin/android/apk/servoapp/src/main/jniLibs/{x86_64,arm64-v8a}/libhello.so` |
| Extract | `packaging.jniLibs.useLegacyPackaging = true` (`servoapp/build.gradle.kts`) → APK:ssa `extractNativeLibs=true` (AGP ei salli attribuuttia Manifestissa eksplisiittisesti) |
| Exec laitteella | **PASS**: `adb shell …/lib/x86_64/libhello.so` → `Katselin PoC1: Hello Android` (emulator-5554) |

---

## PoC 2

Korvataan Hello World pienellä HTTP-palvelimella.

Esimerkiksi

GET /

palauttaa

```

OK

```

Tavoite

Android pystyy

- käynnistämään prosessin
- avaamaan localhost-portin
- vastaamaan HTTP-pyyntöihin

Tässä vaiheessa ei käytetä Meilisearchia lainkaan.

### PoC 2 – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| Crate | `Katselin-haku/android-poc/http` (stdlib `TcpListener`, erillinen `[workspace]`, samat NDK r28 -linkkerit kuin PoC 1) |
| Targetit | `x86_64-linux-android`, `aarch64-linux-android` — PIE ELF, linker `/system/bin/linker64` |
| jniLibs | `Katselin/android/apk/servoapp/src/main/jniLibs/{x86_64,arm64-v8a}/libhttp.so` |
| Kuuntelu | `127.0.0.1:17700`, `GET /` → body `OK` |
| Exec + HTTP | **PASS**: `adb push` → `/data/local/tmp/libhttp.so`; log `listening on http://127.0.0.1:17700`; laitteella `nc 127.0.0.1 17700` → `HTTP/1.1 200 OK` + `OK`; host `adb forward` + HTTP 200 (emulator-5554) |

---

## PoC 3

Lisätään actix-web.

Tavoite

Varmistaa että nykyinen palvelinarkkitehtuuri toimii Androidissa.

Jos actix toimii sellaisenaan, myöhemmät muutokset pienenevät huomattavasti.

### PoC 3 – toteutus (lukittu 3.8.2026)

PoC 2:n kontrahti näyttää, että stdlib-soketti toimii. Meilisearchin HTTP-pino perustuu `actix-web 4.13` + `tokio`-ajonaikaan (`crates/meilisearch/Cargo.toml:22–28, 82`). PoC 3 testaa nimenomaan tätä yhdistelmää ennen täyttä meilisearch-käännöstä.

| Asia | Ratkaisu |
|---|---|
| Crate | `Katselin-haku/android-poc/actix` (erillinen `[workspace]` kuten hello/http) |
| Riippuvuudet | `actix-web = { version = "4", default-features = false, features = ["macros"] }` + `tokio = { version = "1", features = ["rt-multi-thread", "macros"] }` |
| Endpoint | `GET /health` → `{"status":"ok"}`; kuuntelee `127.0.0.1:17701` |
| Targetit | samat `x86_64-linux-android`, `aarch64-linux-android`, samat NDK r28 -linkkerit |
| jniLibs | `Katselin/android/apk/servoapp/src/main/jniLibs/{x86_64,arm64-v8a}/libactix.so` |
| Varmistus | `adb push` → exec → log `listening`; `nc 127.0.0.1 17701` → HTTP 200 + JSON |

**Miksi tämä välivaihe (eikä suoraan Vaihe 1):** PoC 2 validoi stdlibin, mutta ei tokion ajonaikaa, signaalinkäsittelyä eikä actixin accept-loopia. Tämä on tarkoituksellinen "puoliväli" ennen raskasta meilisearch-puuta. Jos actix onnistuu, Vaihe 1 keskittyy puhtaasti milli/heed/LMDB:hen. Jos actix epäonnistuu (esim. tokio/`epoll`-ongelma bionicissa), ratkaistaan se pienessä binäärissä ennen koko riippuvuuspuun käännöstä.

**PoC 3:n rajaus:** ei TLS:tä, ei brotli/gzip-pakkausta, ei `actix-cors`:ia, ei autentikointia — vain raaka HTTP-serveri. Tavoite on todistaa, että `actix-web::HttpServer` käynnistyy ja vastaa bionicissa.

### PoC 3 – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| Crate | `Katselin-haku/android-poc/actix` (`actix-web 4.14`, `tokio 1.53`, erillinen `[workspace]`, NDK r28) |
| Targetit | `x86_64-linux-android`, `aarch64-linux-android` — PIE ELF, linker `/system/bin/linker64` |
| jniLibs | `Katselin/android/apk/servoapp/src/main/jniLibs/{x86_64,arm64-v8a}/libactix.so` (~2.9 MB x86_64) |
| Endpoint | `GET /health` → `{"status":"ok"}`; kuuntelee `127.0.0.1:17701` |
| Exec + HTTP | **PASS**: `adb push` → `/data/local/tmp/libactix.so`; log `listening on http://127.0.0.1:17701`; `nc 127.0.0.1 17701/health` → HTTP 200 + JSON; host `adb forward` → HTTP 200 (emulator-5554) |

---

## PoC 4a (uusi) – heed/LMDB-välitestaus

Vaihe 3 (LMDB) on projektin suurin tekninen riski. Sen sijaan että LMDB:n toimivuus jätetään täyden meilisearch-käännöksen (Vaihe 1–2) varaan, testataan se **erikseen pienessä PoC:ssa** heti PoC 3:n jälkeen. Tämä antaa varhaisen, halvan signaalin suurimmasta riskistä.

### PoC 4a – toteutus

| Asia | Ratkaisu |
|---|---|
| Crate | `Katselin-haku/android-poc/lmdb` |
| Riippuvuudet | `heed = { version = "0.22.1", default-features = false, features = ["serde-json"] }` — **ei** `posix-sem` Androidilla |
| Operaatiot | smoke write→read→reopen; HTTP `127.0.0.1:17702` `/put` + `/get`; DB `/data/local/tmp` (exec-rajoitus ei koske dataa) |
| Käännösasetukset | `lmdb-master-sys` C-käännös NDK:lla; oletus `MDB_USE_POSIX_MUTEX` (`__ANDROID__`) |
| jniLibs | `jniLibs/{x86_64,arm64-v8a}/liblmdb.so` |
| Varmistus | kirjoita + lue laitteella; env uudelleenavaus; HTTP put/get |

**Feature-profiili lukittu:** milli käyttää `heed` 0.22.1 + `serde-json`/`serde-bincode`. **Androidilla EI oteta `posix-sem`-featurea** (ks. toteutus alla).

**Päätöksenteko:** 
- PoC 4a **PASS** → jatketaan Vaiheeseen 1 täysin luottamuksin; LMDB-riski on rauhoitettu.
- PoC 4a **FAIL** → tutkitaan `MDB_USE_ROBUST=0` / heed-patch / `EnvFlags::NO_LOCK`. Jos LMDB ei millään käännö tai toimi bionicissa, **projekti pysähtyy** ja fallback on rikastettu `seed_search` (INTEGRAATIO-SUUNNITELMA vaihe 3). Tämä tehdään ENNEN Vaiheen 1–2 työtä, jotta ei polteta aikaa mahdottomaan.

### PoC 4a – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| Crate | `Katselin-haku/android-poc/lmdb` (`heed 0.22.1`, `serde-json`, erillinen `[workspace]`, NDK r28 + CC/AR) |
| Locking | **ei** `posix-sem` — bionicissa `sem_open` → ENOSYS (os error 38). NDK:lla (`__ANDROID__`) LMDB valitsee oletuksena `MDB_USE_POSIX_MUTEX` + `MDB_USE_ROBUST=0` |
| Targetit | `x86_64-linux-android`, `aarch64-linux-android` — PIE ELF |
| jniLibs | `Katselin/android/apk/servoapp/src/main/jniLibs/{x86_64,arm64-v8a}/liblmdb.so` (~740 KB) |
| Smoke | **PASS**: write+read OK → reopen+read OK (`/data/local/tmp/poc4a-db`) |
| HTTP | **PASS**: `127.0.0.1:17702` `/put?key=k&value=v` → `PUT_OK`; `/get?key=k` → `v` (emulator-5554) |

**Korjaus suunnitelmaan:** aiempi oletus "ota `lmdb-posix-sem` päälle bionicille" oli väärä. Millin `lmdb-posix-sem` (`heed/posix-sem`) on tarkoitettu Applen App Sandboxille (named POSIX semaphores). Androidissa named semaphores puuttuvat; oikea polku on LMDB:n oletus POSIX mutex. Vaihe 1:ssä **älä** kytke `lmdb-posix-sem`-featurea Android-targetille.

---

## PoC 4 (entinen PoC 4)

Rakennetaan nykyinen Meilisearch ilman indeksiä.

Tavoite

Saada binääri käynnistymään Androidissa.

Ei vielä hakua.

Ei vielä LMDB:tä (validoitu jo PoC 4a:ssa erikseen).

Ei vielä dokumentteja.

Pelkkä käynnistyminen.

### PoC 4 / M1 – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| Build | `android/build-android.sh` — NDK r28, `--no-default-features` |
| Patchit | mimalloc pois Androidilla; `tokenizers` `fancy-regex` (ei `onig`); `.cargo/config.toml` Android-linkkerit |
| Feature | **ei** `all-tokenizations` (lindera-ko-dic/unidic build-scriptit kaatuvat cross-käännöksessä); **ei** `lmdb-posix-sem`; **ei** mini-dashboard |
| Targetit | `x86_64-linux-android` (~49 MB stripattu), `aarch64-linux-android` — PIE ELF |
| jniLibs | `Katselin/.../jniLibs/{x86_64,arm64-v8a}/libmeilisearch.so` |
| Käynnistys | **PASS**: `cd` kirjoitettavaan hakemistoon + `--db-path` / `--dump-dir` / `--snapshot-dir` alle `/data/local/tmp`; `--env development --no-analytics` |
| HTTP | **PASS**: `GET /health` → `{"status":"available"}` portissa `127.0.0.1:7700` (emulator-5554, Meilisearch 1.51.0) |

**Huomiot M3:lle:** oletus-cwd `/` on lukuoikeudeton → spawnissa asetettava cwd sovelluksen `files`-hakemistoon ja kaikki Meilisearch-polut sinne. Ensimmäinen yritys ilman `cd` → `Read-only file system (os error 30)`.

### M2 / M3 – toteutus (3.8.2026)

| Asia | Tulos |
|---|---|
| jniLibs | Vain `libmeilisearch.so` (PoC hello/http/actix/lmdb poistettu APK-koon vuoksi) |
| Packaging | `useLegacyPackaging = true` (jo M1:stä) |
| Assets | `KotisatamaAssets`: ensisijainen `${nativeLibraryDir}/libmeilisearch.so`; legacy asset-binääri vain fallback |
| Env | `KOTISATAMA_MEILISEARCH_BIN`, `_DB`, `_DUMP_DIR`, `_SNAPSHOT_DIR`, `_CWD` → `files/kotisatama/` |
| Spawn | `kotisatama-search`: `current_dir(CWD)` + `--dump-dir`/`--snapshot-dir`/`--no-analytics` |
| Fetch | `build-android.sh` ohittaa glibc-fetchen jos jniLibs-so löytyy; `fetch-meilisearch.sh --android-ndk` ohjaa NDK-polkuun |
| APK-varmistus | **PASS** (3.8.2026): x64Release asennettu emulator-5554; logcat `KotisatamaAssets: Meilisearch binary: …/lib/x86_64/libmeilisearch.so` |
| Live haku | **PASS** (3.8.2026): omnibox-haku → `libmeilisearch.so` lapsiprosessi; `GET /health` → available; `/version` 1.51.0; indeksi `documents` **2382** dokumenttia (seed `documents.json`) |

**M4-tila:** seed-polku toimii jo laitteella (documents.json → indeksi). CDN-dump (`--import-dump`) on optimoiva jatkokehitys, ei enää Android-portin blokki.

---

# Vaihe 1

Riippuvuuksien kartoitus

Selvitetään kaikki crate:t jotka estävät Android-käännöksen.

Esimerkiksi

- heed
- lmdb-master-sys
- onig
- candle
- mimalloc

Jokainen dokumentoidaan.

Tuloksena syntyy taulukko

| crate | Android | ratkaisu |
|--------|----------|----------|

### Vaihe 1 – esikartoitus (3.8.2026, lukittu)

Tehty riippuvuuspuun lukemalla (`Cargo.toml`/`Cargo.lock`), ennen ensimmäistäkään meilisearch-käännösyritystä. **Oletus: hyväksytään vain käännösyritykset `x86_64-linux-android`- ja `aarch64-linux-android`-targeteilla, `crates/milli`- ja `crates/meilisearch`-feature-profiileilla.**

| crate / pino | Rooli | Android-status (arvio) | Lukittu ratkaisu |
|---|---|---|---|
| `heed` 0.22.1 | LMDB-rust -sidokset | 🟢 PoC 4a PASS | `default-features=false`, `serde-json`(+`serde-bincode` millissä); **ei** `posix-sem` Androidilla |
| `lmdb-master-sys` | LMDB C-koodi | 🟢 PoC 4a PASS | NDK C-käännös; oletus `MDB_USE_POSIX_MUTEX` (`__ANDROID__`); `posix-sem` → ENOSYS |
| `actix-web` 4.13 + `tokio` | HTTP-palvelin + ajonaika | 🟡 testataan PoC 3:ssa | `default-features=false`; ei TLS:ää PoC:ssa; rustls-0_23 jos TLS myöhemmin |
| `charabia` 0.9.9 | tokenisointi | 🟡 | `default-features=false` (milli jo tekee näin); onig-fallback |
| `onig` / `onig_sys` | regex tokenisoinnissa | 🟠 | **pois** (`onig`-feature ei oletus; käytetään charabian fallbackia) |
| `candle` / `tokenizers` (ML) | embeddings / vector | 🟠 | **pois kokonaan** — embeddings ei Katselin-tarve; binary size + muisti |
| `mimalloc` 0.1.48 (`v3`,`override`) | allokointi | 🟡 | jos NDK-käännös onnistuu pidetään; muuten `#[cfg(target_os="android")]` → system-allocator |
| `memmap2` | tiedosto-mmap | 🟢 | toimii Androidissa (POSIX) |
| `rustls 0.23` + `ring` | TLS | 🟢 | `ring` kääntyy Androidille; tarvitaan vain jos TLS käytössä (localhost: ei) |
| `sysinfo` 0.38 | resurssimonitorointi | 🟡 | `system`+`disk`-featuret; Android-procfs; poistettavissa jos ongelmia |
| `num_cpus` | säikeiden määrä | 🟢 | toimii (sched_getaffinity) |
| `platform-dirs` | hakemistopolut | 🟠 | Android-polut tulevat `KotisatamaAssets`/`KOTISATAMA_*`-env:stä — ohitetaan |
| `utoipa` + swagger | OpenAPI | 🟠 | **pois** (`swagger`-feature ei oletus) — ei tarvita |
| `mini-dashboard` | staattinen UI | 🟠 | **pois** (`default` poistetaan; `meilisearch-types/all-tokenizations` jätetään) |
| `segment` (analytics) | telemetria | 🟠 | **pois/kuollut koodi** Androidissa — ei telemetriaa Katselinissa |
| `cellulite`, `geojson`, `geoutils`, `rstar` | geo-haku | 🟡 | säilyy (milli vaatii); kääntyy puhtaana Rustina |
| `grenad`, `fst`, `roaring`, `obkv`, `levenshtein_automata`, `big_s` | indeksiydin | 🟢 | puhdasta Rustia, pitäisi kääntyä |

**Oletus (lukittu):** `heed`/`lmdb-master-sys` ja `actix-web`/`tokio` ovat ainoat oikeat tekniset riskit. Kaikki muu on joko puhdasta Rustia tai feature-poistettavissa ilman milli-rikkoutumista. Siksi PoC 3 (actix) ja PoC 4a (heed) ajetaan ENNEN tätä vaihetta — ne ratkaisevat molemmat riskit ennen työlästä koko-puun käännöstä.

**Vaiheen 1 tehtäväksi jää:** käännä koko `meilisearch`-binääri `--no-default-features` + tarvittavat tokenization-featuret, tarkista että yllä oleva taulukko pitää paikkansa, ja dokumentoi poikkeamat.

---

# Vaihe 2

Feature-siivous

Poistetaan Android-buildista kaikki ominaisuudet joita Katselin ei tarvitse.

Ensimmäisiä ehdokkaita

- embeddings
- AI
- vector search
- oniguruma
- experimental

Tavoitteena on pienin mahdollinen toimiva binääri.

### Vaihe 2 – lukittu feature-profiili (3.8.2026)

Katselin-haku ajetaan laitteella ilman `default`-featureja. Lukittu `--no-default-features`-profiili (`crates/meilisearch/Cargo.toml:140–165`):

| Feature | Päätös | Perustelu |
|---|---|---|
| `default` | **pois** | sisältää `mini-dashboard` (staattinen UI, ei tarve Androidissa) |
| `meilisearch-types/all-tokenizations` | **pois Androidilla** | vetää lindera-ko-dic/unidic; build-scriptit kaatuvat NDK-crossissa. Latin-tokenisointi riittää FI/SV |
| `mini-dashboard` | **pois** | ei UI:ta; poistaa `static-files`,`reqwest`,`zip` build-depsit |
| `swagger` | **pois** | OpenAPI-dokumentaatio ei tarve |
| `chinese`, `japanese`, `korean`, `thai`, ... | **pois** (ellei `all-tokenizations` vaadi) | vain Katselinin tarvitsemat tokenizationit |
| `swedish-recomposition` | **arvioitava** | ruotsinkielinen sisältö mahdollinen (sv-locale olemassa) — pidetään jos halpa |
| embeddings / vector / `candle` | **pois kokonaan** | ei AI-hakua Katselinissa; isoin binary-size -säästö |
| `onig` | **pois** | ei oletus; charabia-fallback riittää |
| telemetria (`segment`) | **pois** | ei analytiikkaa |
| `rustls`/TLS | **pois localhostissa** | kuunnellaan `127.0.0.1`; ei TLS-terminointia |

Tavoitteena on pienin mahdollinen toimiva binääri. APK-kokoon vaikuttaa suoraan: `strip = true` release-profiiliin + `opt-level = "z"` harkittavissa Android-targetille.

**Esitieto Vaiheen 2 lopputulokselle:** PoC 4 (ilman indeksiä) käynnistyy tällä profiililla.

---

# Vaihe 3

LMDB

Projektin suurin tekninen riski.

Selvitettävät asiat

- kääntyykö LMDB Androidille
- tarvitseeko POSIX semaphore -muutoksia
- tarvitseeko heed muutoksia

Jos LMDB ei toimi, projekti pysähtyy tähän vaiheeseen.

### Vaihe 3 – riskin aikaisistus (3.8.2026)

Tämän vaiheen sisältö on siirretty aikaisemmaksi **PoC 4a:han** (ks. yllä), jotta suurin riski ratkeaa ennen Vaiheiden 1–2 työtä. PoC 4a testaa heed 0.22.1 + `lmdb-master-sys` NDK-käännöksen ja perusoperaatiot laitteella samalla feature-profiililla kuin milli.

Kun PoC 4a on PASS, tämä vaihe supistuu varmistukseksi: koko meilisearch-binäärin (feature-siivottuna) LMDB-ympäristö avautuu ja toimii laitteella Vaiheen 4 dump-importin yhteydessä.

---

# Vaihe 4

Hakemiston lataus

Lisätään dump import.

Tavoitteena

```

meilisearch --import-dump dump.dump

```

toimii Androidissa.

---

# Vaihe 5

Suorituskyky

Mitataan

- RAM
- CPU
- käynnistysaika
- indeksin latausaika

Tarvittaessa poistetaan lisää ominaisuuksia.

---

# Vaihe 6

Julkaisubuild

Lisätään

GitHub Actions

joka rakentaa

- arm64
- x86_64

artefaktit automaattisesti.

---

# Paketointivaatimukset (integraatiokontrahti)

Kuluttaja on Katselin-päärepo (android/apk), ei Kotisatama-forkki suoraan (ks. `Katselin/docs/REPO-JAKO-SUUNNITELMA.md`).

Binäärin ja APK-puolen on sovittava seuraavista:

- Artefakti: PIE-suoritettava binääri (Rust tuottaa oletuksena; Android vaatii PIE:n)
- Nimi APK:ssa: `libmeilisearch.so` (jniLibs-kaavio: `jniLibs/arm64-v8a/` + `jniLibs/x86_64/`)
- Extract: `packaging.jniLibs.useLegacyPackaging = true` build-skriptissä (tuottaa `extractNativeLibs=true` APK:hon; älä aseta attribuuttia Manifestissa eksplisiittisesti – AGP hylkää sen)
- Ajonaikainen polku: `${ApplicationInfo.nativeLibraryDir}/libmeilisearch.so` → `KOTISATAMA_MEILISEARCH_BIN`
- Jakelu: GitHub Releases -artefaktit per abi (arm64, x86_64), versiointi sidottu Katselin-julkaisuihin

PoC 1–2 validoivat tämän kontrahdin pienimmällä binäärillä ennen Meilisearch-käännöstä: `libhello.so` ja `libhttp.so` käynnistyivät `nativeLibraryDir`:stä ja vastasivat HTTP:llä laitteella. PoC 3 (`libactix.so`) ja PoC 4a (`liblmdb.so`) käyttävät samaa kaavaa.

---

# Muutosten rajaus

Projekti EI muuta

- HTTP API:a
- indeksiformaattia
- SearchClientiä
- dump-formaattia

Android-portin tulee näyttää Kotisatamalle tavalliselta Meilisearch-palvelimelta.

---

# Riskit

## Korkea

LMDB — **PoC 4a PASS** (3.8.2026; POSIX mutex, ei posix-sem)

## Keskisuuri

actix-web / tokio Androidissa — **PoC 3 PASS** (3.8.2026)

## Keskisuuri

APK-koko (feature-siivous + strip)

## Matala

HTTP API (PoC 2 todisti peruskonseptin)

## Matala

SearchClient

---

# Hyväksymiskriteerit

Projekti onnistuu kun

✓ cargo build onnistuu Android-targetille

✓ binääri käynnistyy Androidissa

✓ localhost:7700 vastaa

✓ dump voidaan importata

✓ SearchClient toimii muuttamatta koodia

---

# Ei tavoitteena

Projektin tarkoitus ei ole tehdä Androidille optimoitua Meilisearch-haaraa.

Projektin tarkoitus on säilyttää mahdollisimman hyvä yhteensopivuus upstreamin kanssa.

Mitä vähemmän rivejä muuttuu, sitä parempi.

Android-portti on ensisijaisesti ylläpidettävä ratkaisu, ei forkki, joka erkanee nopeasti upstreamista.
