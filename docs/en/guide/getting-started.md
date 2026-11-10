# Getting started

You can check your collection progress in just a few steps: **download → extract → launch → select a save → review missing items → export a report**.
The app is read-only and works offline. All parsing and analysis happen on your computer, and your save is never modified.

## 1. Download

Download `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` from
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases), or go directly to the
[latest release](https://github.com/perillaroc/sblade-explorer/releases/latest).
For information about release assets, the CLI, and system requirements, see [Download & install](./installation.md).

## 2. Extract and launch

Extract the ZIP archive and run `sblade-explorer.exe`. No installation is required. The app supports Windows 10 and 11
and requires the WebView2 Runtime, which is usually installed with Microsoft Edge.

## 3. Select a save

On first launch, the app automatically scans the default save directories and opens the most recently modified main slot.
Use the **Save** dropdown at the top to switch slots. The file size and modification time are shown alongside it;
click **Refresh** to scan again. The app remembers saves you select manually, and **Choose a save file…** lets you open one from any location.
If no save is found, the app lists the directories it scanned and the actions you can take. See [Save locations](./save-location.md) for details.

## 4. Check your progress

The **Summary** page in the sidebar shows:

- **Save information**: SteamID, playthrough (including the NG+ count), difficulty, and play time;
- **Catalog progress**: Overall progress across 13 collectible categories and the number of missing items;
- **Album progress**: Naytiba and character album progress, shown separately and not included in catalog progress.

Select a category in the sidebar to open its page. You can filter by **All / Obtained / Missing**,
search by name, location, or ID, and select any item to view its full details. See [Completion analysis](./collections.md).

## 5. Review missing items

Select **Missing** on a category page, or search for an item by name. Select an item to open its details,
including acquisition instructions, Chinese guides, and the reason it is still missing (for example, "Requires NG+").

## 6. Export a report

Click **Export JSON** or **Export Markdown** in the top-right corner to save a report.
The **Content** button switches item details between Chinese, English, and both languages.
To change the language of menus and buttons, go to **Settings → Interface language**. See [Exporting reports](./export.md).

## Next steps

- [Download & install](./installation.md)
- [Save locations](./save-location.md)
- [Completion analysis](./collections.md)
- [Exporting reports](./export.md)
- [FAQ](./faq.md)
