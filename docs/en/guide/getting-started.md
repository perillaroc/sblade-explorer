# Getting started

This page walks new users through the shortest path: **download → unzip → launch → find your save → check completion → review what is missing → export the result**.
The app is read-only and offline: parsing and analysis run entirely on your machine and it never modifies your save.

## 1. Download

The desktop app is published on [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases):
download `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` (latest version on
[releases/latest](https://github.com/perillaroc/sblade-explorer/releases/latest)).
The release list, CLI and system requirements are covered in [Download & install](./installation.md).

## 2. Unzip and launch

Unzip the archive and run `sblade-explorer.exe`. The portable build needs no installation; it requires
Windows 10/11 and the WebView2 runtime (usually already present with Edge).

## 3. Find your save

On first launch the app scans the default directories, picks the most recently written main slot and
opens it. The **Save** dropdown at the top switches slots and shows the file size and modification
time; **Refresh** rescans the directories. Manually picked saves are remembered, and
**Choose a save file…** opens a save from anywhere.
If no save is found, the app lists the directories it scanned together with actions you can take; see
[Save locations](./save-location.md).

## 4. Check your completion

The **Summary** page on the left shows:

- Save info: SteamID, playthrough (including the NG+ count), difficulty and play time;
- **Catalog progress**: total completion of the 13 collectible categories and how many are missing;
- **Album progress**: the Naytiba and character albums, shown separately and excluded from catalog progress.

Select any category in the sidebar to open its own page. Category pages support the
**All / Obtained / Missing** filters and search by name, location or ID; select an item for full
details. See [Completion analysis](./collections.md).

## 5. Review what is missing

Pick the **Missing** filter on a category page, or search by name; selecting an item opens the detail
dialog with how to obtain it, the Chinese guide buttons and the reason it is missing
(for example "Requires NG+").

## 6. Export the result

The top right offers **Export JSON** and **Export Markdown**. The **Content** switch next to them
toggles item text between 中文 / English / Both, and the interface language is changed separately
under **Settings → Interface language**. See [Exporting reports](./export.md).

## Next steps

- [Download & install](./installation.md)
- [Save locations](./save-location.md)
- [Completion analysis](./collections.md)
- [Exporting reports](./export.md)
- [FAQ](./faq.md)
