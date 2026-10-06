# FAQ

## Will it modify my save?

No. The app only reads the save; parsing and analysis happen entirely in memory and the save file is
never written to. Local settings stay on your machine and are not written to the save or the
registry.

## Does it need an internet connection?

No. The data is bundled with the app and nothing goes online at runtime; the guide and search
buttons only open links in your browser when you click them.

## Which platforms are supported?

Windows 10/11. Save discovery only covers the Windows paths listed under
[Save locations](./save-location.md); other platforms are not supported.

## Is there an installer? How do I upgrade?

The build is portable: unzip and run. There is no installer, Start menu shortcut or uninstall entry,
and the app is read-only and does not write to the registry. To upgrade, replace the exe with the new
version.

## It reports that the WebView2 runtime is missing

Install the Microsoft Edge WebView2 Evergreen Runtime and restart the app (Windows 10/11 usually
ships with it through Edge).

## No save is found

The app lists the directories it actually scanned (open them in File Explorer with **Open**), and
**Choose a save file…** lets you pick any `StellarBladeSave*.sav` from anywhere. Directory list and
file naming rules are in [Save locations](./save-location.md).

## Why is the album excluded from completion progress?

Catalog progress only counts the 13 collectible categories; the album (Naytiba and characters) is
shown separately in reports and the UI and is unlocked through the achievement aliases
`Ach_Album_Unlock_*`, so it does not affect catalog progress.

## What does "Mapping unconfirmed" mean?

It means the name/alias mapping for that entry has low confidence (69 entries in total: a few record
version variants and unlinked camps). It does not affect obtained detection; if an entry is missing
or mapped incorrectly you can fix it with a custom catalog, see
[Data sources & catalog](../reference/data.md#custom-catalog).

## How do I fix missing or incorrect entries?

Create the user override file `%LOCALAPPDATA%\sbsave\catalog.user.json`, or attach an override
temporarily with the CLI's `--catalog <path>`; no catalog regeneration is needed. The format is
documented under [Custom catalog](../reference/data.md#custom-catalog).

## What if parsing fails after a game update?

A parse failure produces a clear error. Check that the game and the save versions match, and do not
force an old catalog onto a save from a newer game version.

## Are NG+ and DLC contents supported?

Yes. The app reads `NewGamePlusPlayCount` and labels items with "Requires NG+" / "Requires NG++";
DLC and deluxe items are flagged separately and appear in the "DLC/Deluxe" column of the cycle
matrix, see [Cycles and the collection matrix](./collections.md#cycles-and-the-collection-matrix).

## What is "N obtained aliases are not linked to catalog entries" at the bottom of a category page?

It means the save contains obtained aliases that could not be matched to catalog entries; it reflects
data coverage. To fix a mapping, see [Custom catalog](../reference/data.md#custom-catalog).

## Are gear affixes (Gear) and skills (PT) supported?

Not in v1. Only collectibles and the album are analysed.

## Where are settings stored?

The appearance (Dark / Light / Follow system), default search engine (Bing / Baidu / Google), the
browser used for links and the save opened on startup are configured under **Settings** at the bottom
of the sidebar and are stored locally only. Guide links open in the selected browser, falling back to
the system default when launching fails.

## Where do I use the CLI?

See the [CLI docs](../cli/index.md): `saves` lists saves, `report` analyses and exports, `dump`
debugs and `catalog` validates the catalog.
