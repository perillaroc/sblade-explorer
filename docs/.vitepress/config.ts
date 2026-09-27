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

  // D01 盘点产物（内部工作文档）不发布到站点，留待 D13 整理时处理。
  srcExclude: ["documentation-inventory.md"],

  themeConfig: {
    nav: [
      { text: "首页", link: "/" },
      { text: "指南", link: "/guide/getting-started" },
      { text: "CLI", link: "/cli/" },
      { text: "数据", link: "/reference/data" },
      { text: "开发", link: "/development/" },
      { text: "GitHub", link: "https://github.com/perillaroc/sblade-explorer" },
    ],

    sidebar: {
      "/guide/": [
        { text: "快速开始", link: "/guide/getting-started" },
        { text: "下载与安装", link: "/guide/installation" },
        { text: "存档位置", link: "/guide/save-location" },
        { text: "收集度分析", link: "/guide/collections" },
        { text: "周目与收集矩阵", link: "/guide/collections#周目与收集矩阵" },
        { text: "导出报告", link: "/guide/export" },
        { text: "FAQ", link: "/guide/faq" },
      ],
      "/cli/": [{ text: "sbsave CLI", link: "/cli/" }],
      "/reference/": [{ text: "数据来源与目录库", link: "/reference/data" }],
      "/development/": [{ text: "开发", link: "/development/" }],
    },

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
