"""Tests for the suit acquisition matrix rendering."""

from __future__ import annotations

import io

from rich.console import Console

from sbsave.analyze import analyze
from sbsave.catalog import load_catalog
from sbsave.report import print_report, render_markdown


def test_suit_matrix_markdown(make_save):
    catalog = load_catalog()
    save = make_save(obtained={"BS_09_2"})
    text = render_markdown(analyze(save, catalog, categories=["nano_suits"]))
    assert "## 纳米战衣获取一览" in text
    assert "### Eidos 7" in text
    assert "✅ 星球空降服（第7小队）第2版" in text
    assert "❌ 废土冒险家" in text
    assert "🔒 星球空降服（第7小队）第3版" in text
    assert "🔒 星球空降防护服（第7小队）" in text


def test_suit_matrix_unlocks_with_ng_plus(make_save):
    catalog = load_catalog()
    text_ng0 = render_markdown(analyze(make_save(ng_plus=0), catalog, categories=["nano_suits"]))
    text_ng1 = render_markdown(analyze(make_save(ng_plus=1), catalog, categories=["nano_suits"]))
    assert "🔒 星球空降服（第7小队）第3版" in text_ng0
    assert "❌ 星球空降服（第7小队）第3版" in text_ng1


def test_suit_matrix_dlc_column(make_save):
    text = render_markdown(analyze(make_save(), load_catalog(), categories=["nano_suits"]))
    assert "| 地点 | 首周目 | 二周目(NG+) | 三周目(NG++) | DLC/特典 |" in text
    assert "🎁 寄叶二号B型制服" in text


def test_print_report_suit_table(make_save):
    buffer = io.StringIO()
    console = Console(file=buffer, width=160, force_terminal=False)
    print_report(analyze(make_save(obtained={"BS_09_2"}), load_catalog()), console=console)
    output = buffer.getvalue()
    assert "纳米战衣" in output
    assert "✅" in output
    assert "🔒" in output
    assert "Flooded Commercial Sector" in output
