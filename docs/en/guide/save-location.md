# Save locations

## Automatic discovery

When the app starts, it recursively searches the following directories for `StellarBladeSave*.sav`, ignoring any directory named `Backup`:

| Location | Description |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | Main save for the full game |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | Saves for some versions or the demo |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | Demo saves |

- Save slots are named `StellarBladeSave00.sav`, `StellarBladeSave01.sav`, and so on. `<SteamID64>` is the numeric account directory.
- By default, the app opens the **most recently modified main slot** (`StellarBladeSave00`). You can select another slot from the dropdown at the top.
- The app remembers the last save you selected, including other slots and files opened with **Choose a save file…**, and opens it on the next launch.
  If that file no longer exists, the app falls back to the most recently modified main slot. You can also select **Switch to automatic** in Settings.
- The selected save's file size, modification time, and full path are displayed.

## If no save is found

- The app lists every directory it scanned and indicates whether each one exists. For directories that do exist, click **Open** to view them in File Explorer.
- To open a save outside the default directories, such as a backup, a save on another drive, or a Steam Cloud copy, click **Choose a save file…**. The file name does not need to follow the game's naming convention.
- Alternatively, place the save in one of the directories listed above and click **Refresh** at the top to scan again.
- Backup files inside a `Backup` directory are not discovered automatically. Use **Choose a save file…** to open them manually.

## Troubleshoot with the CLI

`sbsave saves` lists discovered saves, including their slot, SteamID, size, modification time, and path.
Use `report --save <path>` or `--slot <slot>` to select a save for analysis. See the [CLI documentation](../cli/index.md) for details.
