# `sbsave` CLI

The `sbsave` command-line tool shares `sbsave-core` with the desktop app: the same features without a
graphical interface. Download `sbsave-<tag>-x86_64-pc-windows-msvc.zip` from
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) and run `sbsave.exe`, or run
it from source:

```powershell
cargo run -p sbsave-cli -- <command>
```

## Commands

### `saves`

Lists the automatically discovered saves (slot, SteamID, size, modification time, path).

```powershell
cargo run -p sbsave-cli -- saves
```

### `report`

Analyses the save and prints a console report (Chinese by default, switchable with `--ui-lang`), and
can export JSON/Markdown. It accepts a save path/slot/category/language and can list obtained items.

The report has two sections: **collectibles** (13 categories, counted in catalog progress) and the
**album** (67 Naytiba + 55 character pages, listed separately and excluded from catalog progress).
In JSON, `summary` contains both `catalog_*` (collectibles) and `album_*` (album) fields, and every
category carries a `section` (`collection`/`album`); album entries also include the official
`desc_zh`/`desc_en` description text.

```powershell
cargo run -p sbsave-cli -- report
cargo run -p sbsave-cli -- report --save "C:\...\StellarBladeSave00.sav" --slot 0
cargo run -p sbsave-cli -- report --category "Nano Suits" --lang both
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
```

### `dump`

Parses the save and prints structural information (for debugging); can export a summary JSON, the
full parse tree JSON and the obtained aliases.

```powershell
cargo run -p sbsave-cli -- dump --obtained
```

### `catalog`

Inspects or validates the catalog (category counts, duplicate aliases, ...).

```powershell
cargo run -p sbsave-cli -- catalog list
cargo run -p sbsave-cli -- catalog check
```

## Options

| Command | Options |
| --- | --- |
| `report` | `--save/-s`, `--slot`, `--category/-c` (category key, Chinese or English name, comma separated), `--lang` (item text: `zh` default / `en` / `both`), `--all` (list obtained items), `--json`, `--markdown`, `--catalog` |
| `dump` | `--save/-s`, `--slot`, `--json` (summary), `--tree` (full parse tree), `--obtained` (aliases) |
| `catalog` | `list` / `check`, both accept `--catalog` to attach an override file |

Every command accepts the global `--ui-lang zh|en`, which controls help, headers, errors and other
interface text; when omitted the system language is used, falling back to Chinese. `--lang` only
affects item text and exported report content.

`--catalog` temporarily attaches a user override file; the format is documented under
[data.md](../reference/data.md#custom-catalog). Obtained detection, NG+/DLC labels and low-confidence
entries are explained in [data.md](../reference/data.md).
