---
layout: home

hero:
  name: sblade-explorer
  text: Save completion analyzer for Stellar Blade
  tagline: Analyze your PC save and quickly find the outfits, records, cans, camps, and other collectibles you're still missing.
  image:
    src: /screenshots/overview.webp
    alt: sblade-explorer summary page showing catalog progress, album progress, and a category overview
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
    title: Safe and read-only
    details: Never modifies your save files
  - icon: ✈️
    title: Fully offline
    details: All parsing and analysis happen on your computer
  - icon: 🔍
    title: Automatic discovery
    details: Finds your Steam saves automatically
  - icon: 📦
    title: Completion analysis
    details: Covers the main collectibles and in-game albums
  - icon: 🔁
    title: Playthrough matrix
    details: Compare item variants across Base, NG+, and NG++
  - icon: 🌐
    title: Bilingual data
    details: Chinese and English interface and item names
---

## Download

Get the portable desktop app or the `sbsave` CLI from
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases/latest)
(Windows 10 or 11 and the WebView2 Runtime required). See
[Download & install](./guide/installation.md) for system requirements and installation and upgrade instructions.

## Documentation

- [User guide](./guide/getting-started.md): Installation, save locations, completion analysis, report exports, and FAQs
- [CLI](./cli/index.md): `sbsave` commands and options
- [Data overview](./reference/data.md): Data sources, the catalog, and the alias model
- [Development](./development/index.md): Development environment, data pipeline, CI, and releases
- [中文文档](/)：本文档的简体中文版

Visit the [GitHub repository](https://github.com/perillaroc/sblade-explorer) for source code, issue reporting, and all releases.
