# sblade-explorer (Stellar Blade save completion analyzer)

**[English](README.en.md) | [简体中文](README.md)**

[![CI](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/perillaroc/sblade-explorer)](https://github.com/perillaroc/sblade-explorer/releases)
![100% Vibe Coded](https://img.shields.io/badge/100%25-Vibe_Coded-blueviolet)

Read your Steam *Stellar Blade* save and report the collectibles you have not obtained yet.
**The Windows desktop app is the primary distribution**, with the `sbsave` CLI as an alternative.
Read-only and offline: saves are never modified, parsing and analysis run entirely in memory, and the
app never goes online at runtime.

> [!IMPORTANT]
> This project is developed with Vibe Coding: code, tests and documentation are primarily
> implemented by AI agents.

## Download

Both the desktop app and the CLI are published on
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases):

| Release asset | Description |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | Portable desktop app: unzip and run `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | CLI: unzip and run `sbsave.exe` |

- Requires Windows 10/11 and the **WebView2 runtime** (usually already present with Edge).
- The portable build has no installer: no Start menu shortcut and no uninstall entry; the app is
  read-only and does not write to the registry. Upgrading means replacing the exe.

## Features

- **Progress summary**: save information (SteamID, playthrough/difficulty/play time) and per-category
  progress for the 13 collectible categories, each with its own page;
- **Album**: the Naytiba and character albums are shown separately and excluded from catalog progress;
- **Browsing and filtering**: filter by collection status and search by name/location; item details
  provide how to obtain, guide links and the reason an item is missing;
- **Cycles and records views**: some categories compare one acquisition spot across
  area × location × playthrough; the records page is organised by in-game type and area;
- **Languages and export**: the interface supports Chinese / English, item text supports
  Chinese / English / bilingual, and JSON or Markdown reports can be exported in one click;
- **Local settings**: appearance theme, the browser used for guide links and the search engine are
  stored on your machine only.

## Quick start

1. Unzip the downloaded archive and run `sblade-explorer.exe`;
2. On first launch the app discovers your save and selects the most recently written main slot; you
   can switch slots manually or refresh;
3. Check overall progress on **Summary** and select a category in the sidebar to see what is missing
   and how to obtain it;
4. Export JSON / Markdown reports from the top right.

A more detailed walkthrough is in
[Getting started](https://sblade-explorer.perillaroc.wang/en/guide/getting-started).

## Documentation

Complete documentation (Chinese / English, English site by default below):

**https://sblade-explorer.perillaroc.wang/en/**（[简体中文](https://sblade-explorer.perillaroc.wang/)）

- [User guide](https://sblade-explorer.perillaroc.wang/en/guide/getting-started): download and
  installation, save locations, completion analysis, exporting and FAQ
- [Data notes](https://sblade-explorer.perillaroc.wang/en/reference/data): data sources, catalog and
  alias model, custom overrides

## CLI

The `sbsave` command-line tool shares the same core as the desktop app: the same features without a
graphical interface. It is also published on
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases).
All commands and options are documented in the
[CLI docs](https://sblade-explorer.perillaroc.wang/en/cli/).

## Development

The development environment, project structure, data pipeline, tests and releases are covered in the
[development docs](https://sblade-explorer.perillaroc.wang/en/development/); the desktop app's own
structure and commands are documented in [apps/desktop/README.md](apps/desktop/README.md) (currently
in Chinese).

## Data sources

- [stellarbladeguide.com](https://stellarbladeguide.com) — English item names, location descriptions
  and cycle tags
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —
  Simplified Chinese item names and alias mappings
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —
  full alias universe and data validation
- Game data tables (`ItemTable`, `ZoneCampTable`, `AlbumTable`) and `Game.locres` (zh-Hans/en) —
  exact mapping from internal aliases to official names
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) and
  [Gamersky](https://www.gamersky.com/handbook/202507/1953527.shtml) — in-game Data Bank order of
  Memory Sticks
- [Gamersky](https://www.gamersky.com/handbook/202404/1738505.shtml) and
  [Bilibili](https://www.bilibili.com/video/BV1Wfj1ztEuj) — built-in Chinese guide links (only opened
  in the system browser when clicked)

The full source list and instructions for refreshing the raw data are in
[data/raw/README.md](data/raw/README.md) (currently in Chinese); the catalog and alias model are
documented in the [online docs](https://sblade-explorer.perillaroc.wang/en/reference/data).

## License

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

The interface and app icon use [Lucide](https://lucide.dev) (ISC license); icon notes are in
[apps/desktop/src-tauri/icons/README.md](apps/desktop/src-tauri/icons/README.md).

Not affiliated with Shift Up or Sony Interactive Entertainment; for personal save analysis only.
