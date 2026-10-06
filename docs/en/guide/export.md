# Exporting reports

## Desktop app

The top right offers two export buttons:

| Button | Content |
| --- | --- |
| **Export JSON** | Complete analysis data, always with both Chinese and English fields |
| **Export Markdown** | Readable report generated for the current content language |

Pick a destination (default file names `sblade-report.json` / `sblade-report.md`); on success the app
shows `Exported <path>`. Exporting only reads the save and never modifies any file.

There are two independent language settings:

- **Content language** (the **Content** switch at the top right): toggles 中文 / English / Both and
  affects item text and the Markdown report content; JSON always contains both languages.
- **Interface language** (Settings → Interface language): menus, buttons, prompts and the exported
  Markdown headings/table headers; on first launch it follows the system language.

The Markdown report contains:

- Save information: SteamID, playthrough, difficulty, play time, catalog/album progress;
- Category summary and album summary tables;
- Comparison tables for matrix categories;
- Missing items: matrix categories as comparison tables, other categories as a list with location,
  how to obtain, the reason it is missing and flags;
- Unmapped aliases (if any).

## CLI

`sbsave report` prints a console report and can export the same JSON / Markdown:

```powershell
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
cargo run -p sbsave-cli -- report --ui-lang en --lang en    # English interface + English item text
```

`--lang zh|en|both` controls the item text (JSON is always bilingual), `--ui-lang zh|en` controls
help, headers, errors and other interface text (defaults to the system language); `--all` also lists
obtained items. See the [CLI docs](../cli/index.md) for all commands and options.
