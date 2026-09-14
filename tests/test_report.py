"""Tests for the multi-cycle acquisition matrix rendering."""

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
    assert "### 埃多斯7号" in text
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
    assert "耳饰" in output
    assert "✅" in output
    assert "🔒" in output
    assert "淹水商业区" in output


def test_language_option(make_save):
    catalog = load_catalog()
    analysis = analyze(make_save(), catalog, categories=["nano_suits"])
    zh = render_markdown(analysis)
    en = render_markdown(analysis, lang="en")
    both = render_markdown(analysis, lang="both")
    assert "### 埃多斯7号" in zh and "Flooded Commercial Sector" not in zh
    assert "### Eidos 7" in en and "### 埃多斯7号" not in en
    assert "Flooded Commercial Sector" in en
    assert "埃多斯7号（Eidos 7）" in both
    assert "淹水商业区（Flooded Commercial Sector）" in both


def test_missing_list_uses_chinese_obtain(make_save):
    analysis = analyze(make_save(), load_catalog(), categories=["cans"])
    zh = render_markdown(analysis)
    en = render_markdown(analysis, lang="en")
    both = render_markdown(analysis, lang="both")
    assert "在施工区东侧" in zh
    assert "On the east side of the Construction Zone" in en
    assert "On the east side of the Construction Zone" not in zh
    assert "在施工区东侧" in both and "On the east side of the Construction Zone" in both


def test_earring_matrix_markdown(make_save):
    catalog = load_catalog()
    text_ng0 = render_markdown(analyze(make_save(ng_plus=0), catalog, categories=["earrings"]))
    assert "## 耳饰获取一览" in text_ng0
    assert "### 埃多斯7号" in text_ng0
    assert "❌ 绯红泪珠" in text_ng0
    assert "🔒 高贵泪珠" in text_ng0
    assert "🔒 黄金之心" in text_ng0
    text_ng2 = render_markdown(analyze(make_save(ng_plus=2), catalog, categories=["earrings"]))
    assert "❌ 高贵泪珠" in text_ng2
    assert "❌ 黄金之心" in text_ng2


def test_other_matrix_categories(make_save):
    save = make_save(ng_plus=1)
    catalog = load_catalog()
    cases = [
        ("glasses", "## 眼镜/面饰获取一览", "❌ 超大圆框眼镜"),
        ("drone_seals", "## 无人机外观获取一览", "❌ 铁甲套装"),
        ("adam_costumes", "## 亚当服装获取一览", "❌ 夜鹰"),
        ("lily_costumes", "## 莉莉服装获取一览", "❌ 雨天"),
    ]
    for key, heading, needle in cases:
        text = render_markdown(analyze(save, catalog, categories=[key]))
        assert heading in text, key
        assert needle in text, key


def test_non_matrix_categories_keep_flat_list(make_save):
    text = render_markdown(analyze(make_save(), load_catalog(), categories=["design_patterns"]))
    assert "## 设计图案获取一览" not in text
    assert "### 设计图案 (0/87)" in text
