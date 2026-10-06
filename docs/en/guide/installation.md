# Download & install

## Download

Both the desktop app and the CLI are published on
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases):

| Release asset | Description |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | Portable desktop app: unzip and run `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | CLI: unzip and run `sbsave.exe`, see the [CLI docs](../cli/index.md) |

Latest release: <https://github.com/perillaroc/sblade-explorer/releases/latest>

## System requirements

- Windows 10 / 11;
- The **WebView2 runtime** must be installed (usually present with Edge; if the app reports it is
  missing, install the Evergreen Runtime from Microsoft).

## About the portable build

- No installer: no Start menu shortcut and no uninstall entry;
- The app itself is read-only: it writes neither to the registry nor to your save files;
- Upgrading: simply replace the exe with the new version.

## Running from source

Rust and Node.js are required; common commands are listed in the
[development docs](../development/index.md).
