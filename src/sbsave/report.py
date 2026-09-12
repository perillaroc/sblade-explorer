"""Console / JSON / Markdown rendering of an analysis result."""

from __future__ import annotations

import json
from dataclasses import asdict
from typing import Any

from rich.console import Console
from rich.table import Table

from .analyze import Analysis, CategoryResult, ItemStatus
from .savegame import SaveData

STATUS_STYLE = {
    "需要二周目(NG+)": "yellow",
    "需要三周目(NG++)": "magenta",
    "尼尔 DLC": "cyan",
    "NIKKE DLC": "cyan",
    "豪华版": "cyan",
    "预购特典": "cyan",
    "夏日更新": "cyan",
}

_MATRIX_LEGEND = "✅ 已获得 · ❌ 未获得 · 🔒 需更高周目 · 🎁 DLC/特典 · ➖ 默认外观"

_MATRIX_COLUMNS = ("首周目", "二周目(NG+)", "三周目(NG++)", "DLC/特典")

_MATRIX_CATEGORIES = frozenset(
    {"nano_suits", "earrings", "glasses", "drone_seals", "adam_costumes", "lily_costumes"}
)

_AREA_ORDER = {
    "Default": 0,
    "Eidos 7": 1,
    "Xion": 2,
    "Wasteland": 3,
    "Matrix 11": 5,
    "Great Desert": 6,
    "Abyss Levoire": 7,
    "Eidos 9": 8,
    "Spire 4": 9,
    "Boss Challenge": 11,
}


def _location_label(status: ItemStatus) -> str:
    item = status.item
    parts = [part for part in (item.area, item.location) if part]
    return " · ".join(parts)


def _obtain_label(status: ItemStatus) -> str:
    item = status.item
    parts: list[str] = []
    if item.obtain:
        parts.append(item.obtain)
    if item.note:
        parts.append(item.note)
    if status.reason:
        parts.append(status.reason)
    return " | ".join(parts)


def _item_emoji(status: ItemStatus, save: SaveData) -> str:
    if status.obtained:
        return "✅"
    item = status.item
    if item.dlc:
        return "🎁"
    if item.ng_plus > save.ng_plus_count:
        return "🔒"
    if not item.aliases:
        return "➖"
    return "❌"


def _item_matrix(statuses: list[ItemStatus], save: SaveData) -> dict[str, dict[str, list[str]]]:
    """Group collectible statuses as area -> location -> cells for 首周目/NG+/NG++/DLC."""
    areas: dict[str, dict[str, list[str]]] = {}
    for status in statuses:
        item = status.item
        area = item.area or "未分类"
        location = item.location or "未分类"
        cells = areas.setdefault(area, {}).setdefault(location, [""] * len(_MATRIX_COLUMNS))
        column = 3 if item.dlc else min(item.ng_plus, 2)
        entry = f"{_item_emoji(status, save)} {item.name}"
        cells[column] = f"{cells[column]}\n{entry}" if cells[column] else entry
    return {
        area: locations
        for area, locations in sorted(areas.items(), key=lambda kv: (_AREA_ORDER.get(kv[0], 99), kv[0]))
    }


def _used_matrix_columns(locations: dict[str, list[str]]) -> list[int]:
    return [i for i in range(len(_MATRIX_COLUMNS)) if any(cells[i] for cells in locations.values())]


def _print_item_matrix(result: CategoryResult, save: SaveData, console: Console) -> None:
    console.print(
        f"[bold]{result.category.name}[/bold] 已获得 {result.obtained_count}/{result.total} · 图例：{_MATRIX_LEGEND}"
    )
    for area, locations in _item_matrix(result.statuses, save).items():
        used = _used_matrix_columns(locations)
        table = Table(title=area)
        table.add_column("地点", style="bold")
        for index in used:
            table.add_column(_MATRIX_COLUMNS[index])
        for location, cells in locations.items():
            row = [location] + [cells[index] or "[dim]—[/dim]" for index in used]
            table.add_row(*row)
        console.print(table)


def _item_markdown(result: CategoryResult, save: SaveData) -> list[str]:
    lines = [f"## {result.category.name}获取一览", ""]
    lines.append(f"已获得 {result.obtained_count}/{result.total} · 图例：{_MATRIX_LEGEND}")
    for area, locations in _item_matrix(result.statuses, save).items():
        used = _used_matrix_columns(locations)
        lines.append("")
        lines.append(f"### {area}")
        lines.append("")
        headers = ["地点"] + [_MATRIX_COLUMNS[index] for index in used]
        lines.append("| " + " | ".join(headers) + " |")
        lines.append("| --- | " + " | ".join("---" for _ in used) + " |")
        for location, cells in locations.items():
            values = [
                cells[index].replace("\n", "<br>") if cells[index] else "—"
                for index in used
            ]
            lines.append(f"| {location} | " + " | ".join(value.replace("|", "\\|") for value in values) + " |")
    return lines


def print_report(analysis: Analysis, console: Console | None = None, show_all: bool = False) -> None:
    console = console or Console()
    save = analysis.save
    console.print(f"[bold]剑星存档分析[/bold] — {save.path.name}")
    console.print(
        f"SteamID: {save.steam_id or '未知'} | 周目: {save.playthrough_label} "
        f"(NG+{save.ng_plus_count}) | 难度: {save.difficulty_label} | 游玩时间: {save.play_time_label}"
    )
    console.print(
        f"已获得物品别名: {len(save.all_obtained)} | 目录进度: "
        f"{analysis.catalog_obtained}/{analysis.catalog_total} ({analysis.percent:.1f}%) | "
        f"未收集: {analysis.missing_total}"
    )
    console.print()

    summary = Table(title="分类汇总", show_lines=False)
    summary.add_column("分类", style="bold")
    summary.add_column("进度", justify="right")
    summary.add_column("缺失", justify="right")
    summary.add_column("其中需多周目/DLC", justify="right")
    summary.add_column("未映射别名", justify="right")
    for result in analysis.categories:
        style = "green" if not result.missing else "white"
        summary.add_row(
            f"[{style}]{result.category.name}[/{style}]",
            f"{result.obtained_count}/{result.total} ({result.percent:.0f}%)",
            str(len(result.missing)),
            str(result.blocked_count),
            str(len(result.extra_obtained)),
        )
    console.print(summary)

    for result in analysis.categories:
        if result.category.key not in _MATRIX_CATEGORIES:
            continue
        console.print()
        _print_item_matrix(result, save, console)

    for result in analysis.categories:
        if result.category.key in _MATRIX_CATEGORIES or not result.missing:
            continue
        console.print()
        console.print(
            f"[bold]{result.category.name}[/bold] {result.obtained_count}/{result.total} · 缺 {len(result.missing)}"
        )
        for status in result.missing:
            flags = status.flags
            flag_text = f" [{'/'.join(flags)}]" if flags else ""
            location = _location_label(status)
            obtain = _obtain_label(status)
            line = f"  [cyan]{status.item.id}[/cyan] {status.item.name}"
            if location:
                line += f"  [dim]{location}[/dim]"
            if obtain:
                line += f"  [dim]({obtain})[/dim]"
            line += flag_text
            console.print(line)

    if show_all:
        for result in analysis.categories:
            if result.category.key in _MATRIX_CATEGORIES or not result.obtained_items:
                continue
            console.print()
            console.print(f"[bold]{result.category.name}[/bold] 已收集:")
            for item in result.obtained_items:
                console.print(f"  [green]✓[/green] {item.id} {item.name}")

    if analysis.unmapped_obtained:
        from .analyze import unmapped_summary

        console.print()
        console.print(
            f"[yellow]有 {len(analysis.unmapped_obtained)} 个已获得别名不在目录库中[/yellow]"
            "（不影响已收集判定，可补充到用户覆盖文件）"
        )
        for prefix, count in unmapped_summary(analysis.unmapped_obtained)[:15]:
            console.print(f"  [dim]{prefix}: {count}[/dim]")


def analysis_to_dict(analysis: Analysis, include_obtained: bool = True) -> dict[str, Any]:
    save = analysis.save
    payload: dict[str, Any] = {
        "save": {
            "path": str(save.path),
            "steam_id": save.steam_id,
            "slot": save.slot,
            "playthrough": save.playthrough_label,
            "ng_plus_count": save.ng_plus_count,
            "difficulty": save.difficulty,
            "play_time_seconds": save.play_time_seconds,
        },
        "summary": {
            "catalog_total": analysis.catalog_total,
            "catalog_obtained": analysis.catalog_obtained,
            "missing_total": analysis.missing_total,
            "percent": round(analysis.percent, 2),
            "obtained_aliases": len(save.all_obtained),
            "unmapped_aliases": len(analysis.unmapped_obtained),
        },
        "categories": [_category_to_dict(result, include_obtained) for result in analysis.categories],
        "unmapped_obtained": analysis.unmapped_obtained,
    }
    return payload


def _item_to_dict(status: ItemStatus) -> dict[str, Any]:
    item = status.item
    return {
        "id": item.id,
        "name": item.name,
        "name_en": item.name_en,
        "aliases": item.satisfy_aliases,
        "area": item.area,
        "location": item.location,
        "obtain": item.obtain,
        "ng_plus": item.ng_plus,
        "dlc": item.dlc,
        "missable": item.missable,
        "note": item.note,
        "confidence": item.confidence,
        "reason": status.reason,
        "flags": status.flags,
    }


def _category_to_dict(result: CategoryResult, include_obtained: bool) -> dict[str, Any]:
    payload: dict[str, Any] = {
        "key": result.category.key,
        "name": result.category.name,
        "total": result.total,
        "obtained": result.obtained_count,
        "missing": [_item_to_dict(status) for status in result.missing],
        "extra_obtained_aliases": result.extra_obtained,
    }
    if include_obtained:
        payload["obtained_items"] = [asdict(item) for item in result.obtained_items]
    return payload


def write_json(analysis: Analysis, path: str, include_obtained: bool = True) -> None:
    payload = analysis_to_dict(analysis, include_obtained=include_obtained)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, indent=2)


def render_markdown(analysis: Analysis) -> str:
    save = analysis.save
    lines: list[str] = []
    lines.append(f"# 剑星存档分析 — {save.path.name}")
    lines.append("")
    lines.append(
        f"- SteamID: {save.steam_id or '未知'}\n"
        f"- 周目: {save.playthrough_label} (NG+{save.ng_plus_count})\n"
        f"- 难度: {save.difficulty_label}\n"
        f"- 游玩时间: {save.play_time_label}\n"
        f"- 目录进度: {analysis.catalog_obtained}/{analysis.catalog_total} ({analysis.percent:.1f}%)\n"
        f"- 未收集: {analysis.missing_total}"
    )
    lines.append("")
    lines.append("## 分类汇总")
    lines.append("")
    lines.append("| 分类 | 进度 | 缺失 | 需多周目/DLC |")
    lines.append("| --- | ---: | ---: | ---: |")
    for result in analysis.categories:
        lines.append(
            f"| {result.category.name} | {result.obtained_count}/{result.total} "
            f"({result.percent:.0f}%) | {len(result.missing)} | {result.blocked_count} |"
        )
    for result in analysis.categories:
        if result.category.key not in _MATRIX_CATEGORIES:
            continue
        lines.append("")
        lines.extend(_item_markdown(result, save))
    lines.append("")
    lines.append("## 未收集清单")
    for result in analysis.categories:
        if result.category.key in _MATRIX_CATEGORIES or not result.missing:
            continue
        lines.append("")
        lines.append(f"### {result.category.name} ({result.obtained_count}/{result.total})")
        lines.append("")
        for status in result.missing:
            item = status.item
            details: list[str] = []
            location = _location_label(status)
            if location:
                details.append(location)
            if item.obtain:
                details.append(item.obtain)
            if item.note:
                details.append(item.note)
            if status.reason:
                details.append(status.reason)
            flags = status.flags
            suffix = f" `{'/'.join(flags)}`" if flags else ""
            line = f"- **{item.name}** (`{item.id}`)"
            if details:
                line += " — " + "；".join(details)
            line += suffix
            lines.append(line)
    if analysis.unmapped_obtained:
        lines.append("")
        lines.append(f"## 未映射别名 ({len(analysis.unmapped_obtained)})")
        lines.append("")
        lines.append("，".join(f"`{alias}`" for alias in analysis.unmapped_obtained))
    lines.append("")
    return "\n".join(lines)


def write_markdown(analysis: Analysis, path: str) -> None:
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(render_markdown(analysis))
