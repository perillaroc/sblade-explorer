# sblade-explorer (save completion analyzer for Stellar Blade)

**[English](README.en.md) | [简体中文](README.md)**

[![CI](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/perillaroc/sblade-explorer)](https://github.com/perillaroc/sblade-explorer/releases)
![100% Vibe Coded](https://img.shields.io/badge/100%25-Vibe_Coded-blueviolet)
[![Netlify Status](https://api.netlify.com/api/v1/badges/12901971-6ff6-4dfe-bc46-08d735ffa6aa/deploy-status)](https://app.netlify.com/projects/sblade-explorer/deploys)

Analyze your Steam save for *Stellar Blade* and find the collectibles you're still missing.
**The Windows desktop app is the primary way to use sblade-explorer**; a command-line tool, `sbsave`, is also available.
Both are read-only and work offline: they never modify your saves, all parsing and analysis happen locally in memory, and no network connection is used at runtime.

> [!IMPORTANT]
> This project is built using Vibe Coding, with AI agents writing most of the code, tests, and documentation.

## Download

Download the desktop app or CLI from [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases):

| Release asset | Description |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | Portable desktop app: extract the archive and run `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | Command-line tool: extract the archive and run `sbsave.exe` |

- Requires Windows 10 or 11 and the **WebView2 Runtime** (usually installed with Microsoft Edge).
- The portable app requires no installation and creates no Start menu shortcuts or uninstall entries. It does not write to the Windows registry. To update it, replace `sblade-explorer.exe` with the newer version.

## Features

- **Progress overview**: View save details such as SteamID, playthrough, difficulty, play time, current area, and current main quest, and track progress across 13 collectible categories, each with its own page.
- **Albums**: The Naytiba and character albums are listed separately and do not count toward catalog progress.
- **Growth**: 77 gear affixes and 12 exospines have their own section and do not count toward catalog progress; upgrading to Mk2 still counts as obtained.
- **Browsing and filtering**: Filter by collection status, missable state (missed / still available), and current area; search by name or location. Item details include acquisition instructions, guide links, and reasons an item is still missing.
- **Save-aware judgments**: Quest progress from the save splits missable entries into missed / still available / unknown, without false "missed" reports when quest data is unavailable.
- **Playthrough and record views**: Supported categories let you compare items at the same location across playthroughs. Records are organized by their in-game type and area.
- **Languages and exports**: Choose Chinese or English for the interface, and Chinese, English, or both for item details. Export reports as JSON or Markdown with one click.
- **Local settings**: Your theme, preferred browser for guide links, search engine, owned DLC, and other preferences are stored only on your computer.

## Quick start

1. Extract the downloaded ZIP archive and run `sblade-explorer.exe`.
2. On first launch, the app automatically finds your saves and opens the most recently modified main slot. You can switch slots or refresh the list at any time.
3. Open **Summary** for an overview, or select a category in the sidebar to see missing items and how to obtain them.
4. Use the buttons in the top-right corner to export a JSON or Markdown report.

For a detailed walkthrough, see [Getting started](https://sblade-explorer.perillaroc.wang/en/guide/getting-started).

## Documentation

The full documentation is available in English and Chinese:

**<https://sblade-explorer.perillaroc.wang/en/>** ([简体中文](https://sblade-explorer.perillaroc.wang/))

- [User guide](https://sblade-explorer.perillaroc.wang/en/guide/getting-started): Download and installation, save locations, completion analysis, report exports, and FAQs.
- [Data overview](https://sblade-explorer.perillaroc.wang/en/reference/data): Data sources, the catalog and alias model, and custom overrides.

## CLI

The `sbsave` CLI uses the same core as the desktop app and provides the same analysis features without a graphical interface.
It is also available from [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases).
See the [CLI documentation](https://sblade-explorer.perillaroc.wang/en/cli/) for a complete list of commands and options.

## Development

For the development environment, project structure, data pipeline, tests, and release process, see the [development documentation](https://sblade-explorer.perillaroc.wang/en/development/).
For the desktop app's structure and commands, see [apps/desktop/README.md](apps/desktop/README.md) (currently in Chinese).

## Data sources

- [stellarbladeguide.com](https://stellarbladeguide.com): English item names, location descriptions, and playthrough tags.
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor): Simplified Chinese item names and alias mappings.
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file): A complete set of aliases used for data validation.
- Game data tables (`ItemTable`, `ZoneCampTable`, `AlbumTable`) and `Game.locres` (zh-Hans/en): Mappings from internal aliases to official names.
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) and [Gamersky](https://www.gamersky.com/handbook/202507/1953527.shtml): The order of Memory Sticks in the in-game Data Bank.
- [Gamersky](https://www.gamersky.com/handbook/202404/1738505.shtml) and [Bilibili](https://www.bilibili.com/video/BV1Wfj1ztEuj): Built-in Chinese guide links, opened in your system browser only when clicked.

For the complete source list and instructions on updating the raw data, see [data/raw/README.md](data/raw/README.md) (currently in Chinese).
For the catalog and alias model, see the [online documentation](https://sblade-explorer.perillaroc.wang/en/reference/data).

## License

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

The interface and app icon use [Lucide](https://lucide.dev) (ISC license). For icon details, see [apps/desktop/src-tauri/icons/README.md](apps/desktop/src-tauri/icons/README.md).

This project is not affiliated with Shift Up or Sony Interactive Entertainment. It is intended for personal save analysis only.
