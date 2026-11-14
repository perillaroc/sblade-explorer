# FAQ

## Will the app modify my save?

No. The app only reads your save, and all parsing and analysis happen in memory. It never writes to the save file.
Settings are stored locally and are not written to your save or the Windows registry.

## Does the app need an internet connection?

No. All required data is bundled with the app, which does not connect to the internet at runtime.
Guide and search links open in your browser only when you click them.

## Which platforms are supported?

Windows 10 and 11. Automatic save discovery only covers the Windows paths listed under
[Save locations](./save-location.md). Other operating systems are not supported.

## Do I need to install it? How do I update?

No installation is required. Simply extract the portable app and run it. No Start menu shortcuts or uninstall entries are created, and the app does not write to the Windows registry.
To update, replace `sblade-explorer.exe` with the newer version.

## What should I do if the WebView2 Runtime is missing?

Install the Microsoft Edge WebView2 Evergreen Runtime and restart the app.
The runtime is usually installed with Microsoft Edge on Windows 10 and 11.

## What should I do if no save is found?

The app lists every directory it scanned. Click **Open** to view an existing directory in File Explorer.
You can also click **Choose a save file…** to select a `StellarBladeSave*.sav` file from any location.
For scan directories and file naming rules, see [Save locations](./save-location.md).

## Why don't albums count toward completion progress?

Catalog progress includes only the 13 collectible categories. Naytiba and character album entries are unlocked through the achievement aliases `Ach_Album_Unlock_*`
and are displayed separately in reports and the app, so they do not affect catalog progress.

## What does "Mapping unconfirmed" mean?

It means the name/alias mapping for an entry has low confidence. The catalog currently has **no** low-confidence entries:
the six Eidos 9 camps were confirmed by matching save coordinates from `DataMap_SBEnv.ZoneEnvStorageList` with each
table row's `CoinSpawnPointName` position. If you still see the mark with an older or custom catalog, that entry has
not been confirmed; it does not affect whether the item is detected as obtained.
See [Data sources & catalog](../reference/data.md#custom-catalog) for how to correct it.

## How do I fix missing or incorrect entries?

Create a user override file at `%LOCALAPPDATA%\sbsave\catalog.user.json`, or temporarily load an override with the CLI's `--catalog <path>` option.
You do not need to regenerate the catalog. For the file format, see [Custom catalog](../reference/data.md#custom-catalog).

## What should I do if parsing fails after a game update?

The app displays a specific error when parsing fails. Check that your game and save versions match,
and do not force an older catalog onto a save created by a newer version of the game.

## Is NG+ and DLC content supported?

Yes. The app reads `NewGamePlusPlayCount` from the save and labels items "Requires NG+" or "Requires NG++".
DLC and Deluxe Edition items are flagged separately and can be compared in the **DLC/Deluxe** column of the cycle matrix.
See [Cycles and the collection matrix](./collections.md#cycles-and-the-collection-matrix).
The save has no reliable DLC entitlement field, so owned DLC must be declared in Settings; see the next question.

## Why is a DLC item no longer marked as limited?

Open **Settings → Owned DLC** and check NieR / NIKKE, or use the CLI's `--dlc nier,nikke`.
Once declared, those items are no longer blocked by DLC ownership; they are labelled "owned, not yet obtained"
and enter the ready-to-collect list. DLC you did not declare stays limited. This setting is stored locally and never written to your save.

## Why do some items show "Missed" and others "Still available"?

The app reads quest state from the save and combines it with the missable rules in the catalog:

- **Missed**: this save has passed a point of no return, so the item can no longer be obtained;
- **Still available**: the item can still be collected now, so it is worth picking up before leaving;
- **Missable (mind the checkpoint)**: a rule is missing, or an older save has no quest data, so the app cannot tell.
  It never reports a false "missed" in this case.

Items blocked by a higher playthrough or an unowned DLC are shown under those limits first. See
[Save parsing and judgments](../reference/data.md#save-parsing-and-judgments) for the rules and JSON fields.

## Why does a category page report unlinked obtained aliases?

The save contains obtained aliases that could not be matched to catalog entries. This message helps indicate gaps in data coverage.
To correct a mapping, see [Custom catalog](../reference/data.md#custom-catalog).

## Are gear affixes (Gear) and skills (PT) supported?

Gear affixes are supported: the 77 pickup entries appear in the **Growth** section and do not count toward catalog progress.
The number of stars at the end of a guide title is its rarity (★ Common, ★★ Uncommon, ★★★ Rare), and the upgraded Mk2 copy
belongs to the same entry, so it is never double counted. Skills (PT) are still unsupported.

## Are exospines collectibles?

No. The 12 exospines appear in the **Growth** section and do not count toward catalog progress. Their obtained status is read
from the save's exospine enhancement state, and the upgraded Mk2 copy belongs to the same entry.

## Why are Body / Beta / Weapon Cores, Tumbler modules, and Drone modules not tracked?

They are consumables: the save only keeps the current inventory amount (also affected by shop purchases and NG+ carry-over),
and there is no per-pickup record. Tumbler and Drone guide entries also share one title each, so they cannot be tied to a
specific pickup. We therefore omit their progress instead of showing a misleading "obtained / total". Gear affixes and
exospines have reliable per-item state and are listed in the **Growth** section.

## Why are Supply Boxes and Supply Chests not tracked?

The 193 supply boxes and supply chests in the guide are generic containers holding consumables or random loot; their
contents never enter the item list. The save records their open state as `ChangeState_ZoneEnv_*_SupplyBox` achievements
(287 in the local save, 293 in the 100% reference save), but guide entries only carry area and location text, with no
container alias, and the two totals do not line up. Per-item progress is therefore not shown.

## Where are settings stored?

Appearance (Dark / Light / Follow system), the default search engine (Bing / Baidu / Google), the browser used to open links, the save opened on startup, and owned DLC can be configured in **Settings** at the bottom of the sidebar.
These preferences are stored only on your computer. Guide links open in your selected browser; if it cannot be launched, the app falls back to your system default browser.

## How do I use the CLI?

See the [CLI documentation](../cli/index.md): `saves` lists save files, `report` analyzes saves and exports reports,
`dump` provides debugging information, and `catalog` validates the catalog.
