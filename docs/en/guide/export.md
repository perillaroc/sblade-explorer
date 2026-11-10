# Exporting reports

## Desktop app

Two export formats are available in the top-right corner:

| Button | Content |
| --- | --- |
| **Export JSON** | Complete analysis data, always including both Chinese and English fields |
| **Export Markdown** | A readable report generated in the current content language |

Click a button and choose where to save the report. The default file names are `sblade-report.json` and `sblade-report.md`.
Once the export succeeds, the app displays `Exported <path>`. Exporting only reads your save and does not modify any files.

The two language settings are independent:

- **Content language** (the **Content** button in the top-right corner): Choose Chinese, English, or both for item details in the app and Markdown reports. JSON always includes both languages.
- **Interface language** (**Settings → Interface language**): Controls menus, buttons, prompts, and the headings and table headers in exported Markdown reports. It is inferred from your system language on first launch.

A Markdown report includes:

- Save information: SteamID, playthrough, difficulty, play time, and catalog/album progress;
- Category and album summary tables;
- Comparison tables for categories that support the cycle matrix;
- Missing items: comparison tables for matrix categories, and a list of locations, acquisition instructions, reasons items are missing, and flags for other categories;
- Unmapped aliases, if any.

## CLI

`sbsave report` prints a report to the console and can also export the same data as JSON or Markdown:

```powershell
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
cargo run -p sbsave-cli -- report --ui-lang en --lang en    # English interface + English item text
```

`--lang zh|en|both` controls item text in reports (JSON is always bilingual), while `--ui-lang zh|en` controls help, table headers, errors, and other interface text (the system language is used by default).
Add `--all` to include obtained items. See the [CLI documentation](../cli/index.md) for all commands and options.
