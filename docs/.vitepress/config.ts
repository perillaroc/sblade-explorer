import { defineConfig } from "vitepress";

// sblade-explorer 在线文档站点配置（中文为默认语言，英文在 /en/ 下）。
// 正式地址：https://sblade-explorer.perillaroc.wang/
export default defineConfig({
  // 中文（默认语言，挂在站点根路径）。
  lang: "zh-CN",
  title: "sblade-explorer",
  description: "《剑星》(Stellar Blade) 存档收集度分析工具：读取 Steam PC 存档，报告尚未收集的收集物。",

  locales: {
    root: {
      label: "简体中文",
      lang: "zh-CN",
      themeConfig: {
        outlineTitle: "本页目录",
        langMenuLabel: "切换语言",
        sidebarMenuLabel: "菜单",
        returnToTopLabel: "回到顶部",
        darkModeSwitchTitle: "切换到深色模式",
        lightModeSwitchTitle: "切换到浅色模式",
        darkModeSwitchLabel: "深色模式",
      },
    },
    en: {
      label: "English",
      lang: "en",
      title: "sblade-explorer",
      description:
        "Stellar Blade save completion analyzer: read a Steam PC save and report the collectibles you are still missing.",
      // 英文站点的主题文案（其余主题配置继承 themeConfig 的默认值）。
      themeConfig: {
        outlineTitle: "On this page",
        langMenuLabel: "Change language",
        sidebarMenuLabel: "Menu",
        returnToTopLabel: "Return to top",
        darkModeSwitchTitle: "Switch to dark theme",
        lightModeSwitchTitle: "Switch to light theme",
        darkModeSwitchLabel: "Appearance",
        nav: [
          { text: "Home", link: "/en/" },
          { text: "Guide", link: "/en/guide/getting-started" },
          { text: "CLI", link: "/en/cli/" },
          { text: "Data", link: "/en/reference/data" },
          { text: "Development", link: "/en/development/" },
          { text: "GitHub", link: "https://github.com/perillaroc/sblade-explorer" },
        ],
        sidebar: {
          "/en/guide/": [
            { text: "Getting started", link: "/en/guide/getting-started" },
            { text: "Download & install", link: "/en/guide/installation" },
            { text: "Save locations", link: "/en/guide/save-location" },
            { text: "Completion analysis", link: "/en/guide/collections" },
            {
              text: "Cycles & matrix",
              link: "/en/guide/collections#cycles-and-the-collection-matrix",
            },
            { text: "Exporting reports", link: "/en/guide/export" },
            { text: "FAQ", link: "/en/guide/faq" },
          ],
          "/en/cli/": [{ text: "sbsave CLI", link: "/en/cli/" }],
          "/en/reference/": [
            { text: "Data sources & catalog", link: "/en/reference/data" },
          ],
          "/en/development/": [{ text: "Development", link: "/en/development/" }],
        },
        editLink: {
          pattern: "https://github.com/perillaroc/sblade-explorer/edit/main/docs/:path",
          text: "Edit this page on GitHub",
        },
        footer: {
          message:
            "Not affiliated with Shift Up or Sony Interactive Entertainment; for personal save analysis only.",
          copyright: "Copyright © 2026 perillaroc · Apache License 2.0",
        },
      },
    },
  },

  // 站点部署在独立域名根路径，不使用 GitHub Pages 子路径，因此 base 固定为 "/"。
  base: "/",
  cleanUrls: true,

  // 站点图标（与桌面应用同一标记，见 public/logo.svg）。
  head: [["link", { rel: "icon", type: "image/svg+xml", href: "/logo.svg" }]],

  themeConfig: {
    logo: { src: "/logo.svg", width: 24, height: 24 },

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
      options: {
        locales: {
          root: {
            translations: {
              button: {
                buttonText: "搜索文档",
                buttonAriaLabel: "搜索文档",
              },
              modal: {
                displayDetails: "显示详细列表",
                resetButtonTitle: "清除搜索",
                backButtonTitle: "关闭搜索",
                noResultsText: "未找到结果：",
                footer: {
                  selectText: "选择",
                  selectKeyAriaLabel: "回车",
                  navigateText: "切换",
                  navigateUpKeyAriaLabel: "上箭头",
                  navigateDownKeyAriaLabel: "下箭头",
                  closeText: "关闭",
                  closeKeyAriaLabel: "Esc",
                },
              },
            },
          },
          en: {
            translations: {
              button: {
                buttonText: "Search",
                buttonAriaLabel: "Search",
              },
              modal: {
                displayDetails: "Display detailed list",
                resetButtonTitle: "Reset search",
                backButtonTitle: "Close search",
                noResultsText: "No results for",
                footer: {
                  selectText: "to select",
                  selectKeyAriaLabel: "enter",
                  navigateText: "to navigate",
                  navigateUpKeyAriaLabel: "up arrow",
                  navigateDownKeyAriaLabel: "down arrow",
                  closeText: "to close",
                  closeKeyAriaLabel: "escape",
                },
              },
            },
          },
        },
      },
    },
  },

  sitemap: {
    hostname: "https://sblade-explorer.perillaroc.wang",
  },
});
