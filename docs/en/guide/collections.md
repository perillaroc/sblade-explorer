# Completion analysis

## Summary page

The **Summary** page in the sidebar shows save information and separate progress totals for the catalog and albums:

- **Save information**: SteamID, playthrough (including the NG+ count), difficulty (Easy / Normal / Hard), and play time;
- **Catalog progress** `x/y (n%)`: Shows the number of missing items and obtained-alias statistics;
- **Album progress** `x/y (n%)`: Shown separately and marked as excluded from catalog progress.

The sidebar organizes content into two groups:

- **Collectible categories** (13 categories included in catalog progress): Nano Suits, Cans,
  Records (Documents/Memory Sticks), Passcodes, Camps, Hairstyles, Glasses & Face Accessories,
  Earrings, Drone Skins, Adam's Outfits, Lily's Outfits, Design Patterns, and Fish;
- **Album (excluded from total progress)**: 67 Naytiba entries and 55 character pages.

## Category pages

Select a category in the sidebar to open its page. Each category page includes:

- **All / Obtained / Missing** filters at the top, with the number of items in each group;
- Search by name, location, or ID;
- A progress bar and a summary reading "Obtained x/y · Missing z · n%";
- Full item details, available by selecting any entry.

Some categories are organized differently:

- **Records**: Split into tabs by in-game type (Memory Sticks, Documents, etc.). Memory Sticks are further grouped by area, with collapsible sections;
- **Naytiba**: Grouped by area, with collapsible sections.

## Item details

Selecting an item opens a dialog containing:

- Name, ID, collection status, area, and location;
- **Album description** (album entries only);
- **How to obtain**;
- **Chinese guides**: Built-in links are shown as **Image guide** and **Video guide**. **Search image guides** (using the search engine selected in Settings) and **Search video guides** (Bilibili) are always available;
- **Why it is missing**: For example, "Requires NG+", "Requires NG++", or "Missable (mind the checkpoint)";
- **Details**: Playthrough, DLC, missable status, record type, mapping confidence, and data source;
- Aliases and notes.

## Cycles and the collection matrix

Nano Suits, Design Patterns, Earrings, Glasses & Face Accessories, Drone Skins, Adam's Outfits, and
Lily's Outfits offer both a **Cycle matrix** and a **List** view. The matrix makes it easy to compare the items available at the same location across playthroughs:

- Columns represent **Base / NG+ / NG++ / DLC/Deluxe**;
- Each row represents one acquisition location. The item found there changes in later playthroughs, so comparing entries across the row reveals what you are missing;
- Statuses include Obtained, Missing, Higher playthrough required, DLC/Deluxe, and Default appearance;
- Select a row to view acquisition instructions and other details.

## How items are counted

- An item counts as obtained if any of its aliases appears in the save's item set, an alias derived from achievement records, or an album achievement alias (`Ach_Album_Unlock_*`). Album entries are unlocked through their corresponding achievements.
- The app reads `NewGamePlusPlayCount` from the save to determine whether to label an item "Requires NG+" or "Requires NG++".
- Based on community resources, 211 entries are marked as missable. These include items in Xion (locked after completing Abyss Levoire), Eidos 9 (available once Lily's progress reaches 100%), and side-quest and bulletin-board request rewards.
  Missing-item lists show "Missable (mind the checkpoint)", and the details and matrix views display a **Missable** badge.
- Six entries have a mapping confidence of "Low (mapping unconfirmed)", all of them camps in Eidos 9.
  A category page may also report that "N more obtained aliases are not linked to catalog entries".

The catalog contains 930 entries in 15 categories: 808 collectibles across 13 categories and 122 album entries.
See [Data sources & catalog](../reference/data.md) for details about the alias model and data sources.
