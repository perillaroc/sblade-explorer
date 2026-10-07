# Completion analysis

## Summary page

**Summary** on the left shows save information and two levels of progress:

- Save info: SteamID, playthrough (including the NG+ count), difficulty (Easy / Normal / Hard) and
  play time;
- **Catalog progress** `x/y (n%)`: the missing count and obtained-alias statistics;
- **Album progress** `x/y (n%)`: shown separately and marked as excluded from catalog progress.

The sidebar groups content into two sections:

- **Collectible categories** (counted in catalog progress, 13 categories): Nano Suits, Cans,
  Records (Documents/Memory Sticks), Passcodes, Camps, Hairstyles, Glasses & Face Accessories,
  Earrings, Drone Skins, Adam's Outfits, Lily's Outfits, Design Patterns and Fish;
- **Album (excluded from total progress)**: 67 Naytiba entries and 55 character pages.

## Category pages

Select a category in the sidebar to open its own page:

- **All / Obtained / Missing** filters at the top, each with its item count;
- Search by name, location or ID;
- A category progress bar plus "Obtained x/y · Missing z · n%";
- Select any item for full details.

Some categories are organised differently:

- **Records**: tabbed by in-game type (Memory Sticks, Documents, ...); Memory Sticks are further
  grouped by area and collapsible;
- **Naytiba**: grouped by area and collapsible.

## Item details

The item detail dialog contains:

- Name, ID, collection status, area and location;
- **Album description** (album entries);
- **How to obtain**;
- **Chinese guides**: built-in links appear as "Image guide" and "Video guide", and
  "Search image guides" (the search engine is configurable in the settings) plus
  "Search video guides" (Bilibili) are always available;
- **Why it is missing**: for example "Requires NG+", "Requires NG++" or
  "Missable (mind the checkpoint)";
- **Details**: cycle, DLC, missable, record type, mapping confidence and source;
- Aliases and notes.

## Cycles and the collection matrix

Nano Suits, Design Patterns, Earrings, Glasses & Face Accessories, Drone Skins, Adam's Outfits and
Lily's Outfits offer both a **cycle matrix** and a **list** view. The matrix compares one acquisition
spot across playthroughs:

- Columns are **Base / NG+ / NG++ / DLC/Deluxe**;
- Each row is one acquisition spot; later cycles replace the item found there, so compare
  horizontally to spot gaps;
- Status legend: Obtained, Missing, Higher playthrough required, DLC/Deluxe, Default appearance;
- Select a row for details such as how to obtain the item.

## How counts are calculated

- An item counts as obtained when any of its aliases appears in the save's item set, in an alias
  derived from the achievement records, or in an album achievement alias (`Ach_Album_Unlock_*`);
  album entries are unlocked through their achievements.
- The app reads `NewGamePlusPlayCount` from the save and labels items with
  "Requires NG+" / "Requires NG++".
- A few entries have low mapping confidence ("Mapping unconfirmed", 6 in total, all Eidos 9 camps);
  a category page may also point out that "N obtained aliases are not linked to catalog entries".

The catalog has 922 entries in 15 categories (800 collectibles in 13 categories + 122 album pages);
see [Data sources & catalog](../reference/data.md) for the alias model and data sources.
