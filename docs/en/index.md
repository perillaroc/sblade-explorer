---
layout: home

hero:
  name: sblade-explorer
  text: Stellar Blade save completion analyzer
  tagline: Read a Stellar Blade PC save and quickly find the outfits, records, cans, camps and more you have not collected yet.
  image:
    src: /screenshots/overview.webp
    alt: sblade-explorer summary page with catalog progress, album progress and category summary
  actions:
    - theme: brand
      text: Download the latest release
      link: https://github.com/perillaroc/sblade-explorer/releases/latest
    - theme: alt
      text: Get started
      link: /en/guide/getting-started
    - theme: alt
      text: GitHub
      link: https://github.com/perillaroc/sblade-explorer

features:
  - icon: 🔒
    title: Read-only and safe
    details: Never modifies save files
  - icon: ✈️
    title: Fully offline
    details: Parsing and analysis run entirely on your machine
  - icon: 🔍
    title: Auto discovery
    details: Finds your Steam save automatically
  - icon: 📦
    title: Completion analysis
    details: Covers the main collectibles and the in-game album
  - icon: 🔁
    title: Cycle matrix
    details: Analyses Base / NG+ / NG++ item variants
  - icon: 🌐
    title: Bilingual
    details: Interface and item names in Chinese / English
---

## Download

Grab the portable desktop app and the `sbsave` CLI from
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases/latest)
(Windows 10/11, WebView2 runtime required); see
[Download & install](./guide/installation.md) for the release list and system requirements.

## Documentation

- [User guide](./guide/getting-started.md): installation, save locations, completion analysis, exporting and FAQ
- [CLI](./cli/index.md): `sbsave` commands and options
- [Data notes](./reference/data.md): data sources, catalog and alias model
- [Development](./development/index.md): dev environment, data pipeline, CI and releases

Source code, bug reports and all releases live in the
[GitHub repository](https://github.com/perillaroc/sblade-explorer).
