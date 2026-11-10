# Download & install

## Download

The desktop app and CLI are available from [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases):

| Release asset | Description |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | Portable desktop app: extract the archive and run `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | Command-line tool: extract the archive and run `sbsave.exe`; see the [CLI documentation](../cli/index.md) |

Download the latest version here: <https://github.com/perillaroc/sblade-explorer/releases/latest>

## System requirements

- Windows 10 or Windows 11;
- **WebView2 Runtime**, which is usually installed with Microsoft Edge. If the app reports that it is missing, download and install the Evergreen Runtime from Microsoft.

## Portable app

- No installation is required, and no Start menu shortcuts or uninstall entries are created.
- The app does not write to the Windows registry or modify your save files.
- To update, replace `sblade-explorer.exe` with the newer version.

## Run from source

Rust and Node.js are required. See the [development documentation](../development/index.md) for common commands.
