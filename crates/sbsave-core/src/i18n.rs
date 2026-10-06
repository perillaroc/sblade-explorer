//! Localized user-facing messages for the CLI, the report and the desktop app.
//!
//! The catalog only covers presentation chrome (labels, headers, errors); item
//! names and guide text stay bilingual in the catalog data and are selected by
//! the report language (`zh` / `en` / `both`).

/// Locale of the user interface (help text, headers, errors).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    Zh,
    En,
}

impl Locale {
    /// Parses a locale code such as `zh`, `zh-CN`, `en` or `en-US`.
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim().to_ascii_lowercase();
        if value == "zh" || value.starts_with("zh-") || value.starts_with("zh_") {
            return Some(Locale::Zh);
        }
        if value == "en" || value.starts_with("en-") || value.starts_with("en_") {
            return Some(Locale::En);
        }
        None
    }

    pub const fn code(self) -> &'static str {
        match self {
            Locale::Zh => "zh",
            Locale::En => "en",
        }
    }
}

/// A message table for one locale.
#[derive(Debug, Clone, Copy)]
pub struct Messages {
    locale: Locale,
}

impl Messages {
    pub const fn new(locale: Locale) -> Self {
        Self { locale }
    }

    pub const fn locale(&self) -> Locale {
        self.locale
    }

    const fn is_zh(&self) -> bool {
        matches!(self.locale, Locale::Zh)
    }

    // -- Common ---------------------------------------------------------

    pub const fn unknown(&self) -> &'static str {
        if self.is_zh() {
            "未知"
        } else {
            "Unknown"
        }
    }

    pub const fn uncategorized(&self) -> &'static str {
        if self.is_zh() {
            "未分类"
        } else {
            "Uncategorized"
        }
    }

    /// Separator between list details in prose output (`a；b` / `a; b`).
    pub const fn list_separator(&self) -> &'static str {
        if self.is_zh() {
            "；"
        } else {
            "; "
        }
    }

    // -- Save labels ----------------------------------------------------

    pub fn ng_plus_label(&self, ng_plus: i64) -> String {
        match (self.is_zh(), ng_plus) {
            (true, 0) => "首周目".to_string(),
            (true, 1) => "二周目(NG+)".to_string(),
            (true, 2) => "三周目(NG++)".to_string(),
            (false, 0) => "Base".to_string(),
            (false, 1) => "NG+".to_string(),
            (false, 2) => "NG++".to_string(),
            (_, other) => format!("NG+{other}"),
        }
    }

    pub fn dlc_label(&self, dlc: &str) -> String {
        match (self.is_zh(), dlc) {
            (true, "nier") => "尼尔 DLC".to_string(),
            (true, "nikke") => "NIKKE DLC".to_string(),
            (true, "deluxe") => "豪华版".to_string(),
            (true, "preorder") => "预购特典".to_string(),
            (true, "summer") => "夏日更新".to_string(),
            (false, "nier") => "NieR DLC".to_string(),
            (false, "nikke") => "NIKKE DLC".to_string(),
            (false, "deluxe") => "Deluxe Edition".to_string(),
            (false, "preorder") => "Pre-order Bonus".to_string(),
            (false, "summer") => "Summer Update".to_string(),
            (_, other) => other.to_string(),
        }
    }

    pub fn playthrough_label(&self, ng_plus_count: i64) -> String {
        if ng_plus_count <= 0 {
            if self.is_zh() {
                "一周目".to_string()
            } else {
                "First playthrough".to_string()
            }
        } else {
            format!("NG+{ng_plus_count}")
        }
    }

    pub fn play_time_label(&self, seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        if self.is_zh() {
            format!("{hours}小时{minutes:02}分")
        } else {
            format!("{hours}h {minutes:02}m")
        }
    }

    pub fn difficulty_label(&self, difficulty: i64) -> String {
        match (self.is_zh(), difficulty) {
            (true, 0) => "简单".to_string(),
            (true, 1) => "普通".to_string(),
            (true, 2) => "困难".to_string(),
            (false, 0) => "Easy".to_string(),
            (false, 1) => "Normal".to_string(),
            (false, 2) => "Hard".to_string(),
            (_, other) => format!("{}({other})", self.unknown()),
        }
    }

    // -- Item status ----------------------------------------------------

    pub const fn flag_missable(&self) -> &'static str {
        if self.is_zh() {
            "可错过"
        } else {
            "Missable"
        }
    }

    pub const fn flag_unconfirmed(&self) -> &'static str {
        if self.is_zh() {
            "映射待确认"
        } else {
            "Mapping unconfirmed"
        }
    }

    pub fn reason_ng_plus(&self, ng_plus: i64) -> String {
        let label = self.ng_plus_label(ng_plus);
        if self.is_zh() {
            format!("需要{label}")
        } else {
            format!("Requires {label}")
        }
    }

    pub fn reason_dlc(&self, dlc: &str) -> String {
        let label = self.dlc_label(dlc);
        if self.is_zh() {
            format!("{label}限定")
        } else {
            format!("{label} only")
        }
    }

    pub const fn reason_missable(&self) -> &'static str {
        if self.is_zh() {
            "可错过(注意节点)"
        } else {
            "Missable (mind the checkpoint)"
        }
    }

    // -- Console report -------------------------------------------------

    pub fn report_title(&self, file_name: &str) -> String {
        if self.is_zh() {
            format!("剑星存档分析 — {file_name}")
        } else {
            format!("Stellar Blade save analysis — {file_name}")
        }
    }

    pub fn report_save_line(
        &self,
        steam_id: &str,
        playthrough: &str,
        ng_plus_count: i64,
        difficulty: &str,
        play_time: &str,
    ) -> String {
        if self.is_zh() {
            format!(
                "SteamID: {steam_id} | 周目: {playthrough} (NG+{ng_plus_count}) | 难度: {difficulty} | 游玩时间: {play_time}"
            )
        } else {
            format!(
                "SteamID: {steam_id} | Playthrough: {playthrough} (NG+{ng_plus_count}) | Difficulty: {difficulty} | Play time: {play_time}"
            )
        }
    }

    pub fn report_progress_line(
        &self,
        aliases: usize,
        obtained: usize,
        total: usize,
        percent: f64,
        missing: usize,
    ) -> String {
        if self.is_zh() {
            format!(
                "已获得物品别名: {aliases} | 目录进度: {obtained}/{total} ({percent:.1}%) | 未收集: {missing}"
            )
        } else {
            format!(
                "Obtained aliases: {aliases} | Catalog progress: {obtained}/{total} ({percent:.1}%) | Missing: {missing}"
            )
        }
    }

    pub fn report_album_line(
        &self,
        obtained: usize,
        total: usize,
        percent: f64,
        missing: usize,
    ) -> String {
        if self.is_zh() {
            format!("图鉴进度: {obtained}/{total} ({percent:.1}%) | 未收集: {missing}（不计入目录进度）")
        } else {
            format!(
                "Album progress: {obtained}/{total} ({percent:.1}%) | Missing: {missing} (excluded from catalog progress)"
            )
        }
    }

    pub const fn category_summary(&self) -> &'static str {
        if self.is_zh() {
            "分类汇总"
        } else {
            "Category summary"
        }
    }

    pub const fn category_summary_header(&self) -> &'static str {
        if self.is_zh() {
            "分类 | 进度 | 缺失 | 其中需多周目/DLC | 未映射别名"
        } else {
            "Category | Progress | Missing | Requires NG+/DLC | Unmapped aliases"
        }
    }

    pub const fn album_summary(&self) -> &'static str {
        if self.is_zh() {
            "图鉴汇总（不计入目录进度）"
        } else {
            "Album summary (excluded from catalog progress)"
        }
    }

    pub const fn album_summary_header(&self) -> &'static str {
        if self.is_zh() {
            "图鉴 | 进度 | 缺失"
        } else {
            "Album | Progress | Missing"
        }
    }

    pub const fn matrix_legend(&self) -> &'static str {
        if self.is_zh() {
            "✅ 已获得 · ❌ 未获得 · 🔒 需更高周目 · 🎁 DLC/特典 · ➖ 默认外观"
        } else {
            "✅ Obtained · ❌ Missing · 🔒 Higher playthrough · 🎁 DLC/Deluxe · ➖ Default appearance"
        }
    }

    pub const fn matrix_columns(&self) -> [&'static str; 4] {
        if self.is_zh() {
            ["首周目", "二周目(NG+)", "三周目(NG++)", "DLC/特典"]
        } else {
            ["Base", "NG+", "NG++", "DLC/Deluxe"]
        }
    }

    pub const fn location_header(&self) -> &'static str {
        if self.is_zh() {
            "地点"
        } else {
            "Location"
        }
    }

    pub fn category_matrix_line(&self, name: &str, obtained: usize, total: usize) -> String {
        if self.is_zh() {
            format!(
                "{name} 已获得 {obtained}/{total} · 图例：{}",
                self.matrix_legend()
            )
        } else {
            format!(
                "{name}: obtained {obtained}/{total} · Legend: {}",
                self.matrix_legend()
            )
        }
    }

    pub fn category_missing_line(
        &self,
        name: &str,
        obtained: usize,
        total: usize,
        missing: usize,
    ) -> String {
        if self.is_zh() {
            format!("{name} {obtained}/{total} · 缺 {missing}")
        } else {
            format!("{name} {obtained}/{total} · {missing} missing")
        }
    }

    pub fn obtained_heading(&self, name: &str) -> String {
        if self.is_zh() {
            format!("{name} 已收集:")
        } else {
            format!("{name} obtained:")
        }
    }

    pub fn unmapped_note(&self, count: usize) -> String {
        if self.is_zh() {
            format!("有 {count} 个已获得别名不在目录库中（不影响已收集判定，可补充到用户覆盖文件）")
        } else {
            format!(
                "{count} obtained aliases are not in the catalog (progress is unaffected; add them to a user override file)"
            )
        }
    }

    // -- Markdown report ------------------------------------------------

    pub fn markdown_category_heading(&self, name: &str) -> String {
        if self.is_zh() {
            format!("## {name}获取一览")
        } else {
            format!("## {name} acquisition list")
        }
    }

    pub fn markdown_legend_line(&self, obtained: usize, total: usize) -> String {
        if self.is_zh() {
            format!("已获得 {obtained}/{total} · 图例：{}", self.matrix_legend())
        } else {
            format!(
                "Obtained {obtained}/{total} · Legend: {}",
                self.matrix_legend()
            )
        }
    }

    pub const fn markdown_summary_heading(&self) -> &'static str {
        if self.is_zh() {
            "## 分类汇总"
        } else {
            "## Category summary"
        }
    }

    pub const fn markdown_summary_header(&self) -> &'static str {
        if self.is_zh() {
            "| 分类 | 进度 | 缺失 | 需多周目/DLC |"
        } else {
            "| Category | Progress | Missing | Requires NG+/DLC |"
        }
    }

    pub const fn markdown_album_heading(&self) -> &'static str {
        if self.is_zh() {
            "## 图鉴汇总（不计入目录进度）"
        } else {
            "## Album summary (excluded from catalog progress)"
        }
    }

    pub const fn markdown_album_header(&self) -> &'static str {
        if self.is_zh() {
            "| 图鉴 | 进度 | 缺失 |"
        } else {
            "| Album | Progress | Missing |"
        }
    }

    pub const fn markdown_missing_heading(&self) -> &'static str {
        if self.is_zh() {
            "## 未收集清单"
        } else {
            "## Missing items"
        }
    }

    pub fn markdown_unmapped_heading(&self, count: usize) -> String {
        if self.is_zh() {
            format!("## 未映射别名 ({count})")
        } else {
            format!("## Unmapped aliases ({count})")
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn markdown_save_block(
        &self,
        steam_id: &str,
        playthrough: &str,
        ng_plus_count: i64,
        difficulty: &str,
        play_time: &str,
        catalog_obtained: usize,
        catalog_total: usize,
        catalog_percent: f64,
        album_obtained: usize,
        album_total: usize,
        album_percent: f64,
        missing: usize,
        album_missing: usize,
    ) -> String {
        if self.is_zh() {
            format!(
                "- SteamID: {steam_id}\n- 周目: {playthrough} (NG+{ng_plus_count})\n- 难度: {difficulty}\n- 游玩时间: {play_time}\n- 目录进度: {catalog_obtained}/{catalog_total} ({catalog_percent:.1}%)\n- 图鉴进度: {album_obtained}/{album_total} ({album_percent:.1}%)\n- 未收集: {missing}（图鉴 {album_missing}）"
            )
        } else {
            format!(
                "- SteamID: {steam_id}\n- Playthrough: {playthrough} (NG+{ng_plus_count})\n- Difficulty: {difficulty}\n- Play time: {play_time}\n- Catalog progress: {catalog_obtained}/{catalog_total} ({catalog_percent:.1}%)\n- Album progress: {album_obtained}/{album_total} ({album_percent:.1}%)\n- Missing: {missing} (album {album_missing})"
            )
        }
    }

    /// Separator for inline lists (`` `a`，`b` `` / `` `a`, `b` ``).
    pub const fn inline_separator(&self) -> &'static str {
        if self.is_zh() {
            "，"
        } else {
            ", "
        }
    }

    // -- CLI help -------------------------------------------------------

    pub const fn cli_about(&self) -> &'static str {
        if self.is_zh() {
            "剑星 (Stellar Blade) Steam 存档收集度分析工具"
        } else {
            "Stellar Blade (剑星) Steam save completion analyzer"
        }
    }

    pub const fn cli_ui_lang_help(&self) -> &'static str {
        if self.is_zh() {
            "界面语言：zh（中文，默认）、en（英文）；默认跟随系统"
        } else {
            "UI language: zh (Chinese, default), en (English); defaults to the system language"
        }
    }

    pub const fn cli_saves_about(&self) -> &'static str {
        if self.is_zh() {
            "列出自动探测到的存档。"
        } else {
            "List automatically discovered saves."
        }
    }

    pub const fn cli_report_about(&self) -> &'static str {
        if self.is_zh() {
            "分析存档收集情况并输出报告。"
        } else {
            "Analyze save completion and print a report."
        }
    }

    pub const fn cli_dump_about(&self) -> &'static str {
        if self.is_zh() {
            "解析存档并输出结构信息（调试用）。"
        } else {
            "Parse a save and dump its structure (debugging)."
        }
    }

    pub const fn cli_catalog_about(&self) -> &'static str {
        if self.is_zh() {
            "查看/校验目录数据库"
        } else {
            "Inspect or validate the catalog database"
        }
    }

    pub const fn cli_catalog_list_about(&self) -> &'static str {
        if self.is_zh() {
            "列出目录库分类与数量。"
        } else {
            "List catalog categories and counts."
        }
    }

    pub const fn cli_catalog_check_about(&self) -> &'static str {
        if self.is_zh() {
            "校验目录库（重复别名、缺失分类等）。"
        } else {
            "Validate the catalog (duplicate aliases, missing categories, ...)."
        }
    }

    pub const fn help_save(&self) -> &'static str {
        if self.is_zh() {
            "指定 .sav 存档路径"
        } else {
            "Path to a .sav save file"
        }
    }

    pub const fn help_slot(&self) -> &'static str {
        if self.is_zh() {
            "存档槽位编号"
        } else {
            "Save slot number"
        }
    }

    pub const fn help_category(&self) -> &'static str {
        if self.is_zh() {
            "只分析指定分类，逗号分隔"
        } else {
            "Only analyze the given categories, comma separated"
        }
    }

    pub const fn help_lang(&self) -> &'static str {
        if self.is_zh() {
            "文本语言：zh（中文，默认）、en（英文）、both（中英对照）"
        } else {
            "Text language: zh (Chinese, default), en (English), both (bilingual)"
        }
    }

    pub const fn help_all(&self) -> &'static str {
        if self.is_zh() {
            "同时列出已收集的物品"
        } else {
            "Also list obtained items"
        }
    }

    pub const fn help_json(&self) -> &'static str {
        if self.is_zh() {
            "导出 JSON 报告"
        } else {
            "Export a JSON report"
        }
    }

    pub const fn help_markdown(&self) -> &'static str {
        if self.is_zh() {
            "导出 Markdown 报告"
        } else {
            "Export a Markdown report"
        }
    }

    pub const fn help_catalog(&self) -> &'static str {
        if self.is_zh() {
            "附加的目录覆盖 JSON 文件"
        } else {
            "Additional catalog override JSON file"
        }
    }

    pub const fn help_tree(&self) -> &'static str {
        if self.is_zh() {
            "导出完整解析树 JSON（可能很大）"
        } else {
            "Export the full parse tree JSON (may be large)"
        }
    }

    pub const fn help_obtained(&self) -> &'static str {
        if self.is_zh() {
            "列出已获得物品别名"
        } else {
            "List obtained item aliases"
        }
    }

    // -- CLI output and errors ------------------------------------------

    pub const fn no_saves_found(&self) -> &'static str {
        if self.is_zh() {
            "未找到存档"
        } else {
            "No saves found"
        }
    }

    pub const fn detected_saves(&self) -> &'static str {
        if self.is_zh() {
            "检测到的存档"
        } else {
            "Detected saves"
        }
    }

    pub const fn saves_header(&self) -> &'static str {
        if self.is_zh() {
            "槽位 | SteamID | 大小 | 修改时间 | 路径"
        } else {
            "Slot | SteamID | Size | Modified | Path"
        }
    }

    pub fn error_save_not_found(&self, path: &str) -> String {
        if self.is_zh() {
            format!("存档不存在: {path}")
        } else {
            format!("Save not found: {path}")
        }
    }

    pub const fn error_no_saves(&self) -> &'static str {
        if self.is_zh() {
            "没有找到存档，请用 --save 指定"
        } else {
            "No saves found; specify one with --save"
        }
    }

    pub fn error_load_save(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("读取存档失败: {detail}")
        } else {
            format!("Failed to read save: {detail}")
        }
    }

    pub fn error_load_catalog(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("读取目录库失败: {detail}")
        } else {
            format!("Failed to read catalog: {detail}")
        }
    }

    pub fn error_unknown_category(&self, token: &str) -> String {
        if self.is_zh() {
            format!("未知分类: {token}")
        } else {
            format!("Unknown category: {token}")
        }
    }

    pub fn error_unknown_language(&self, lang: &str, options: &str) -> String {
        if self.is_zh() {
            format!("未知语言: {lang}（可选 {options}）")
        } else {
            format!("Unknown language: {lang} (expected one of {options})")
        }
    }

    pub fn error_write_json(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("写入 JSON 失败: {detail}")
        } else {
            format!("Failed to write JSON: {detail}")
        }
    }

    pub fn write_json_ok(&self, path: &str) -> String {
        if self.is_zh() {
            format!("JSON 报告已写入 {path}")
        } else {
            format!("JSON report written to {path}")
        }
    }

    pub fn error_write_markdown(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("写入 Markdown 失败: {detail}")
        } else {
            format!("Failed to write Markdown: {detail}")
        }
    }

    pub fn write_markdown_ok(&self, path: &str) -> String {
        if self.is_zh() {
            format!("Markdown 报告已写入 {path}")
        } else {
            format!("Markdown report written to {path}")
        }
    }

    pub fn dump_engine_line(
        &self,
        engine: &str,
        class: &str,
        evas: bool,
        properties: usize,
    ) -> String {
        if self.is_zh() {
            format!("引擎: {engine} | 类: {class} | EVAS: {evas} | 顶层属性: {properties}")
        } else {
            format!("Engine: {engine} | Class: {class} | EVAS: {evas} | Top-level properties: {properties}")
        }
    }

    pub const fn dump_top_properties(&self) -> &'static str {
        if self.is_zh() {
            "顶层属性"
        } else {
            "Top-level properties"
        }
    }

    pub const fn dump_properties_header(&self) -> &'static str {
        if self.is_zh() {
            "名称 | 类型 | 大小"
        } else {
            "Name | Type | Size"
        }
    }

    pub fn dump_obtained_counts(
        &self,
        aliases: usize,
        achievements: usize,
        purchases: usize,
        ng_plus: i64,
    ) -> String {
        if self.is_zh() {
            format!(
                "已获得别名: {aliases} | 成就记录: {achievements} | 购买记录: {purchases} | NG+ 次数: {ng_plus}"
            )
        } else {
            format!(
                "Obtained aliases: {aliases} | Achievements: {achievements} | Purchases: {purchases} | NG+ count: {ng_plus}"
            )
        }
    }

    pub fn error_write_summary(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("写入摘要失败: {detail}")
        } else {
            format!("Failed to write summary: {detail}")
        }
    }

    pub fn summary_written(&self, path: &str) -> String {
        if self.is_zh() {
            format!("摘要已写入 {path}")
        } else {
            format!("Summary written to {path}")
        }
    }

    pub fn error_write_tree(&self, detail: &str) -> String {
        if self.is_zh() {
            format!("写入解析树失败: {detail}")
        } else {
            format!("Failed to write parse tree: {detail}")
        }
    }

    pub fn tree_written(&self, path: &str) -> String {
        if self.is_zh() {
            format!("完整解析树已写入 {path}")
        } else {
            format!("Full parse tree written to {path}")
        }
    }

    pub fn catalog_version(&self, version: i64) -> String {
        if self.is_zh() {
            format!("目录库 v{version}")
        } else {
            format!("Catalog v{version}")
        }
    }

    pub const fn catalog_list_header(&self) -> &'static str {
        if self.is_zh() {
            "分类键 | 分类名 | 段 | 条目数 | 已映射别名 | 需多周目 | DLC"
        } else {
            "Key | Name | Section | Items | Mapped aliases | Requires NG+ | DLC"
        }
    }

    pub fn check_alias_duplicate(&self, alias: &str, previous: &str, current: &str) -> String {
        if self.is_zh() {
            format!("别名 {alias} 同时映射到 {previous} 和 {current}")
        } else {
            format!("Alias {alias} is mapped to both {previous} and {current}")
        }
    }

    pub fn check_category_undefined(&self, item: &str, category: &str) -> String {
        if self.is_zh() {
            format!("条目 {item} 的分类 {category} 未定义")
        } else {
            format!("Item {item} references undefined category {category}")
        }
    }

    pub fn check_missing_name_en(&self, count: usize) -> String {
        if self.is_zh() {
            format!("有 {count} 个分类缺少英文名 name_en")
        } else {
            format!("{count} categories have no English name (name_en)")
        }
    }

    pub fn check_ok(&self, items: usize, aliases: usize, low_confidence: usize) -> String {
        if self.is_zh() {
            format!("[OK] 目录库正常：{items} 个条目，{aliases} 个别名映射（低置信度 {low_confidence} 条）")
        } else {
            format!(
                "[OK] Catalog is valid: {items} items, {aliases} alias mappings ({low_confidence} low confidence)"
            )
        }
    }

    // -- Desktop (Tauri) errors -----------------------------------------

    pub fn error_inspect_save(&self, path: &str) -> String {
        if self.is_zh() {
            format!("无法读取存档文件: {path}")
        } else {
            format!("Cannot read save file: {path}")
        }
    }

    pub const fn error_not_scanned_dir(&self) -> &'static str {
        if self.is_zh() {
            "不是已扫描的存档目录"
        } else {
            "Not a scanned save directory"
        }
    }

    pub fn error_unknown_export_format(&self, format: &str) -> String {
        if self.is_zh() {
            format!("未知导出格式: {format}")
        } else {
            format!("Unknown export format: {format}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Locale, Messages};

    #[test]
    fn parses_locale_codes() {
        assert_eq!(Locale::parse("zh"), Some(Locale::Zh));
        assert_eq!(Locale::parse("zh-CN"), Some(Locale::Zh));
        assert_eq!(Locale::parse("zh_CN"), Some(Locale::Zh));
        assert_eq!(Locale::parse("EN-us"), Some(Locale::En));
        assert_eq!(Locale::parse("fr"), None);
        assert_eq!(Locale::parse(""), None);
    }

    #[test]
    fn english_and_chinese_labels_stay_in_sync() {
        let zh = Messages::new(Locale::Zh);
        let en = Messages::new(Locale::En);
        assert_eq!(zh.ng_plus_label(1), "二周目(NG+)");
        assert_eq!(en.ng_plus_label(1), "NG+");
        assert_eq!(zh.difficulty_label(2), "困难");
        assert_eq!(en.difficulty_label(2), "Hard");
        assert_eq!(zh.play_time_label(3660), "1小时01分");
        assert_eq!(en.play_time_label(3660), "1h 01m");
        assert_eq!(zh.dlc_label("nier"), "尼尔 DLC");
        assert_eq!(en.dlc_label("nier"), "NieR DLC");
        assert!(zh.category_matrix_line("罐子", 3, 4).contains("已获得 3/4"));
        assert!(en
            .category_matrix_line("Cans", 3, 4)
            .contains("obtained 3/4"));
    }
}
