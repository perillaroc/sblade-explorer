"""Command line interface for the Stellar Blade save explorer."""

from __future__ import annotations

import json
import sys
from datetime import datetime
from pathlib import Path

import typer
from rich.console import Console
from rich.table import Table

from . import __version__
from .analyze import analyze
from .catalog import load_catalog
from .gvas import to_jsonable
from .report import LANGUAGES, print_report, write_json, write_markdown
from .savegame import SaveError, SaveSlot, discover_saves, load_save, pick_default_save

for _stream in (sys.stdout, sys.stderr):
    try:
        _stream.reconfigure(encoding="utf-8", errors="replace")
    except (AttributeError, ValueError):  # pragma: no cover - non reconfigurable stream
        pass

app = typer.Typer(
    name="sbsave",
    help="剑星 (Stellar Blade) Steam 存档收集度分析工具",
    no_args_is_help=True,
    add_completion=False,
)
catalog_app = typer.Typer(help="查看/校验目录数据库", no_args_is_help=True)
app.add_typer(catalog_app, name="catalog")

console = Console()


def _version_callback(value: bool) -> None:
    if value:
        console.print(f"sbsave {__version__}")
        raise typer.Exit()


@app.callback()
def main(
    version: bool | None = typer.Option(
        None, "--version", callback=_version_callback, is_eager=True, help="显示版本"
    ),
) -> None:
    pass


def _resolve_slots(save: str | None, slot: int | None) -> list[SaveSlot]:
    if save:
        path = Path(save)
        if not path.is_file():
            raise typer.BadParameter(f"存档不存在: {path}")
        steam_id = path.parent.name if path.parent.name.isdigit() else None
        return [SaveSlot(path=path, steam_id=steam_id, slot=0, mtime=0.0)]
    slots = discover_saves()
    if slot is not None:
        slots = [s for s in slots if s.slot == slot]
    if not slots:
        raise typer.BadParameter("没有找到存档，请用 --save 指定")
    return slots


def _load(save: str | None, slot: int | None):
    if save or slot is not None:
        slots = _resolve_slots(save, slot)
        chosen = pick_default_save(slots)
    else:
        chosen = pick_default_save()
    return load_save(chosen.path, steam_id=chosen.steam_id)


def _resolve_categories(catalog, raw: str | None) -> list[str] | None:
    if not raw:
        return None
    keys: list[str] = []
    for token in raw.replace("，", ",").split(","):
        token = token.strip()
        if not token:
            continue
        if token in catalog.categories:
            keys.append(token)
            continue
        match = next((c for c in catalog.categories.values() if c.name == token), None)
        if match is None:
            raise typer.BadParameter(f"未知分类: {token}")
        keys.append(match.key)
    return keys


@app.command("saves")
def list_saves() -> None:
    """列出自动探测到的存档。"""
    slots = discover_saves()
    if not slots:
        console.print("[yellow]未找到存档[/yellow]")
        return
    table = Table(title="检测到的存档")
    table.add_column("槽位")
    table.add_column("SteamID")
    table.add_column("大小")
    table.add_column("修改时间")
    table.add_column("路径")
    for item in slots:
        size = f"{item.path.stat().st_size / 1024 / 1024:.1f} MB"
        table.add_row(
            f"{item.slot:02d}",
            item.steam_id or "-",
            size,
            datetime.fromtimestamp(item.mtime).strftime("%Y-%m-%d %H:%M"),
            str(item.path),
        )
    console.print(table)


@app.command("report")
def report_command(
    save: str | None = typer.Option(None, "--save", "-s", help="指定 .sav 存档路径"),
    slot: int | None = typer.Option(None, "--slot", help="存档槽位编号"),
    category: str | None = typer.Option(None, "--category", "-c", help="只分析指定分类，逗号分隔"),
    lang: str = typer.Option("zh", "--lang", help="文本语言：zh（中文，默认）、en（英文）、both（中英对照）"),
    all_items: bool = typer.Option(False, "--all", help="同时列出已收集的物品"),
    json_out: str | None = typer.Option(None, "--json", help="导出 JSON 报告"),
    markdown_out: str | None = typer.Option(None, "--markdown", help="导出 Markdown 报告"),
    catalog_path: str | None = typer.Option(None, "--catalog", help="附加的目录覆盖 JSON 文件"),
) -> None:
    """分析存档收集情况并输出报告。"""
    if lang not in LANGUAGES:
        raise typer.BadParameter(f"未知语言: {lang}（可选 {', '.join(LANGUAGES)}）")
    try:
        save_data = _load(save, slot)
    except SaveError as exc:
        console.print(f"[red]读取存档失败: {exc}[/red]")
        raise typer.Exit(code=1) from exc
    catalog = load_catalog([Path(catalog_path)] if catalog_path else None)
    keys = _resolve_categories(catalog, category)
    result = analyze(save_data, catalog, categories=keys)
    print_report(result, console=console, show_all=all_items, lang=lang)
    if json_out:
        write_json(result, json_out)
        console.print(f"[green]JSON 报告已写入 {json_out}[/green]")
    if markdown_out:
        write_markdown(result, markdown_out, lang=lang)
        console.print(f"[green]Markdown 报告已写入 {markdown_out}[/green]")


@app.command("dump")
def dump_command(
    save: str | None = typer.Option(None, "--save", "-s", help="指定 .sav 存档路径"),
    slot: int | None = typer.Option(None, "--slot", help="存档槽位编号"),
    json_out: str | None = typer.Option(None, "--json", help="导出存档结构摘要 JSON"),
    tree_out: str | None = typer.Option(None, "--tree", help="导出完整解析树 JSON（可能很大）"),
    obtained: bool = typer.Option(False, "--obtained", help="列出已获得物品别名"),
) -> None:
    """解析存档并输出结构信息（调试用）。"""
    try:
        save_data = _load(save, slot)
    except SaveError as exc:
        console.print(f"[red]读取存档失败: {exc}[/red]")
        raise typer.Exit(code=1) from exc

    gvas = save_data.gvas
    console.print(f"[bold]{save_data.path}[/bold]")
    console.print(
        f"引擎: {gvas.header.engine} | 类: {gvas.header.save_game_class_name} | "
        f"EVAS: {gvas.header.has_evas_prefix} | 顶层属性: {len(gvas.properties)}"
    )
    table = Table(title="顶层属性")
    table.add_column("名称")
    table.add_column("类型")
    table.add_column("大小", justify="right")
    for name, prop in gvas.properties.items():
        table.add_row(name, prop.type, str(prop.size))
    console.print(table)

    console.print(
        f"已获得别名: {len(save_data.obtained_items)} | 成就记录: {len(save_data.achievements)} | "
        f"购买记录: {len(save_data.shop_purchases)} | NG+ 次数: {save_data.ng_plus_count}"
    )

    if obtained:
        for alias in sorted(save_data.obtained_items):
            console.print(f"  {alias}")

    if json_out:
        summary = {
            "path": str(save_data.path),
            "class": gvas.header.save_game_class_name,
            "engine": str(gvas.header.engine),
            "properties": [
                {"name": name, "type": prop.type, "size": prop.size, "struct": prop.struct_name}
                for name, prop in gvas.properties.items()
            ],
            "counters": save_data.counters,
            "string_counters": save_data.string_counters,
            "obtained_count": len(save_data.obtained_items),
        }
        Path(json_out).write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")
        console.print(f"[green]摘要已写入 {json_out}[/green]")

    if tree_out:
        tree = {name: to_jsonable(prop) for name, prop in gvas.properties.items()}
        with open(tree_out, "w", encoding="utf-8") as handle:
            json.dump(tree, handle, ensure_ascii=False, indent=1)
        console.print(f"[green]完整解析树已写入 {tree_out}[/green]")


@catalog_app.command("list")
def catalog_list(
    catalog_path: str | None = typer.Option(None, "--catalog", help="附加的目录覆盖 JSON 文件"),
) -> None:
    """列出目录库分类与数量。"""
    catalog = load_catalog([Path(catalog_path)] if catalog_path else None)
    table = Table(title=f"目录库 v{catalog.version}")
    table.add_column("分类键")
    table.add_column("分类名")
    table.add_column("条目数", justify="right")
    table.add_column("已映射别名", justify="right")
    table.add_column("需多周目", justify="right")
    table.add_column("DLC", justify="right")
    for category in catalog.category_list():
        items = catalog.by_category(category.key)
        mapped = sum(len(item.satisfy_aliases) for item in items)
        ng = sum(1 for item in items if item.ng_plus)
        dlc = sum(1 for item in items if item.dlc)
        table.add_row(category.key, category.name, str(len(items)), str(mapped), str(ng), str(dlc))
    console.print(table)


@catalog_app.command("check")
def catalog_check(
    catalog_path: str | None = typer.Option(None, "--catalog", help="附加的目录覆盖 JSON 文件"),
) -> None:
    """校验目录库（重复别名、缺失分类等）。"""
    catalog = load_catalog([Path(catalog_path)] if catalog_path else None)
    problems: list[str] = []
    seen: dict[str, str] = {}
    for item in catalog.items:
        for alias in item.satisfy_aliases:
            if alias in seen and seen[alias] != item.id:
                problems.append(f"别名 {alias} 同时映射到 {seen[alias]} 和 {item.id}")
            seen[alias] = item.id
        if item.category not in catalog.categories:
            problems.append(f"条目 {item.id} 的分类 {item.category} 未定义")
    low = [item.id for item in catalog.items if item.confidence != "high"]
    if problems:
        for problem in problems:
            console.print(f"[red][X] {problem}[/red]")
        raise typer.Exit(code=1)
    console.print(
        f"[green][OK] 目录库正常：{len(catalog.items)} 个条目，{len(seen)} 个别名映射[/green]"
        f"（低置信度 {len(low)} 条）"
    )


def run() -> None:  # pragma: no cover - convenience entry point
    sys.exit(app())


if __name__ == "__main__":  # pragma: no cover
    run()
