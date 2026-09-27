import { defineConfig } from "vitepress";

// sblade-explorer 在线文档站点配置。
// 正式地址：https://sblade-explorer.perillaroc.wang/
export default defineConfig({
  lang: "zh-CN",
  title: "sblade-explorer",
  description: "《剑星》(Stellar Blade) 存档收集度分析工具：读取 Steam PC 存档，报告尚未收集的收集物。",

  // 站点部署在独立域名根路径，不使用 GitHub Pages 子路径，因此 base 固定为 "/"。
  base: "/",
  cleanUrls: true,

  // D02：现有 docs/*.md 暂不迁移（属 D03），且其链接指向 docs/ 之外，先排除出站点构建；
  // D03 迁移完成后移除此项。
  srcExclude: ["cli.md", "data.md", "development.md", "documentation-inventory.md"],

  themeConfig: {
    nav: [
      { text: "首页", link: "/" },
      // TODO(D03)：迁移现有文档后补全 指南 / CLI / 数据 / 开发
      { text: "GitHub", link: "https://github.com/perillaroc/sblade-explorer" },
    ],

    // TODO(D03)：按目标信息架构（指南 / CLI / 数据 / 开发）补充
    sidebar: [
      {
        text: "开始",
        items: [{ text: "首页", link: "/" }],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/perillaroc/sblade-explorer" },
    ],

    editLink: {
      pattern: "https://github.com/perillaroc/sblade-explorer/edit/main/docs/:path",
      text: "在 GitHub 上编辑此页",
    },

    footer: {
      message: "本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。",
      copyright: "Copyright © 2026 perillaroc · Apache License 2.0",
    },

    search: {
      provider: "local",
    },
  },

  sitemap: {
    hostname: "https://sblade-explorer.perillaroc.wang",
  },
});
