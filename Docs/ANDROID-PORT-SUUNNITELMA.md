# Katselin-haku – Android-portin toteutussuunnitelma

Päivämäärä: 2.8.2026

Status: Luonnos (v1)

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
println("Hello Android");
}

```

Paketoidaan se APK:n native library directoryyn.

Tavoite:

- prosessi käynnistyy
- stdout näkyy logcatissa
- exec onnistuu

Jos tämä epäonnistuu, koko arkkitehtuuri muuttuu.

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

---

## PoC 3

Lisätään actix-web.

Tavoite

Varmistaa että nykyinen palvelinarkkitehtuuri toimii Androidissa.

Jos actix toimii sellaisenaan, myöhemmät muutokset pienenevät huomattavasti.

---

## PoC 4

Rakennetaan nykyinen Meilisearch ilman indeksiä.

Tavoite

Saada binääri käynnistymään Androidissa.

Ei vielä hakua.

Ei vielä LMDB:tä.

Ei vielä dokumentteja.

Pelkkä käynnistyminen.

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

---

# Vaihe 3

LMDB

Projektin suurin tekninen riski.

Selvitettävät asiat

- kääntyykö LMDB Androidille
- tarvitseeko POSIX semaphore -muutoksia
- tarvitseeko heed muutoksia

Jos LMDB ei toimi, projekti pysähtyy tähän vaiheeseen.

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

LMDB

## Keskisuuri

actix-web Androidissa

## Keskisuuri

APK-koko

## Matala

HTTP API

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
