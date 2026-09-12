"""CLI smoke tests."""

from __future__ import annotations

from typer.testing import CliRunner

from sbsave.cli import app
from sbsave.savegame import discover_saves

runner = CliRunner()


def test_help():
    result = runner.invoke(app, ["--help"])
    assert result.exit_code == 0
    assert "report" in result.output


def test_version():
    result = runner.invoke(app, ["--version"])
    assert result.exit_code == 0
    assert "sbsave" in result.output


def test_catalog_list():
    result = runner.invoke(app, ["catalog", "list"])
    assert result.exit_code == 0
    assert "nano_suits" in result.output


def test_catalog_check():
    result = runner.invoke(app, ["catalog", "check"])
    assert result.exit_code == 0


def test_report_with_real_save():
    slots = discover_saves()
    if not slots:
        return
    result = runner.invoke(app, ["report", "--category", "cans"])
    assert result.exit_code == 0
    assert "罐子" in result.output


def test_dump_with_real_save():
    slots = discover_saves()
    if not slots:
        return
    result = runner.invoke(app, ["dump"])
    assert result.exit_code == 0
    assert "SBSaveGame" in result.output
