# Save locations

## Automatic discovery

On startup the app recursively searches the directories below for `StellarBladeSave*.sav`. Any
directory named `Backup` is ignored:

| Location | Description |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | Main save of the full game |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | Some versions / demo |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | Demo |

- Slot files are named `StellarBladeSave00.sav`, `StellarBladeSave01.sav`, and so on;
  `<SteamID64>` is the numeric account directory;
- The app selects the **most recently written main slot** (`StellarBladeSave00`) by default; other
  slots can be picked in the top dropdown;
- The manually selected save is remembered (including other slots and files opened with
  **Choose a save file…**) and reopened on the next launch. When the file no longer exists the app
  falls back to the newest main slot; **Switch to automatic** in the settings restores that behaviour;
- The selected save shows its file size, modification time and full path.

## When no save is found

- The app lists the directories it actually scanned and marks which ones exist; existing directories
  can be opened in File Explorer with **Open**;
- **Choose a save file…** opens saves outside the default directories (backups, other disks, Steam
  Cloud copies, ...); the file name does not have to follow the game's naming scheme;
- Alternatively put the save into one of the directories above and press **Refresh** at the top to
  rescan;
- Backup files inside a `Backup` directory are not discovered automatically; open them manually with
  **Choose a save file…**.

## Troubleshooting with the CLI

`sbsave saves` lists the discovered saves (slot, SteamID, size, modification time, path).
Use `report --save <path>` or `--slot <slot>` to select any save; see the
[CLI docs](../cli/index.md).
