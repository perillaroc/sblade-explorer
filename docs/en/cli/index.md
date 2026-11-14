# `sbsave` CLI

The `sbsave` CLI shares `sbsave-core` with the desktop app and provides the same analysis features without a graphical interface.
Download `sbsave-<tag>-x86_64-pc-windows-msvc.zip` from
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases), extract it, and run `sbsave.exe`. You can also run it from source:

```powershell
cargo run -p sbsave-cli -- <command>
```

## Commands

### `saves`

Lists automatically discovered saves, including their slot, SteamID, size, modification time, and path.

```powershell
cargo run -p sbsave-cli -- saves
```

### `report`

Analyzes a save and prints a report to the console, with optional JSON and Markdown exports.
You can specify a save, slot, category, or language and include obtained items. The console interface is in Chinese by default; use `--ui-lang` to change it.

The report has two sections: **collectibles** (13 categories included in catalog progress) and the
**album** (67 Naytiba entries and 55 character pages, listed separately and excluded from catalog progress).
In JSON, `summary` contains both `catalog_*` (collectibles) and `album_*` (album) fields, and every category has a
`section` field (`collection` or `album`). Album entries also include official descriptions in `desc_zh` and `desc_en`.

```powershell
cargo run -p sbsave-cli -- report
cargo run -p sbsave-cli -- report --save "C:\...\StellarBladeSave00.sav" --slot 0
cargo run -p sbsave-cli -- report --category "Nano Suits" --lang both
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
```

### `dump`

Parses a save and prints structural information for debugging. It can export a JSON summary, the full parse tree as JSON, or a list of obtained aliases.

```powershell
cargo run -p sbsave-cli -- dump --obtained
```

### `catalog`

Inspects or validates the catalog, including category counts and duplicate aliases.

```powershell
cargo run -p sbsave-cli -- catalog list
cargo run -p sbsave-cli -- catalog check
```

## Options

| Command | Options |
| --- | --- |
| `report` | `--save/-s`, `--slot`, `--category/-c` (category key, Chinese name, or English name; separate multiple categories with commas), `--lang` (item text: `zh` (default) / `en` / `both`), `--all` (include obtained items), `--json`, `--markdown`, `--catalog`, `--dlc` (owned DLC, comma separated: `nier,nikke`) |
| `dump` | `--save/-s`, `--slot`, `--json` (summary), `--tree` (full parse tree), `--obtained` (aliases) |
| `catalog` | `list` / `check`; both support `--catalog` to load an override file |

Every command accepts the global options `--ui-lang zh|en` and `--dlc nier,nikke`; the former controls help,
table headers, errors, and other interface text, the latter declares owned DLC (the save has no reliable
entitlement field, so it only affects DLC-limited judgments).
If the language is omitted, the system language is used, with Chinese as the fallback. `--lang` affects only item text and exported report content.

`--catalog` temporarily loads a user override file; see [data.md](../reference/data.md#custom-catalog) for the format.
For obtained-item detection, NG+/DLC labels, and low-confidence entries, see [data.md](../reference/data.md).
