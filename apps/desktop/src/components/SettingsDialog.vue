<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, RotateCcw, Search, Settings, X } from "@lucide/vue";
import {
  resetSettings,
  SEARCH_ENGINES,
  settings,
  type BrowserInfo,
  type SearchEngineId,
} from "../lib/settings";

const emit = defineEmits<{ close: [] }>();

const browsers = ref<BrowserInfo[]>([]);
const browsersLoaded = ref(false);

async function loadBrowsers() {
  try {
    browsers.value = await invoke<BrowserInfo[]>("list_browsers");
  } catch (reason) {
    console.warn("读取浏览器列表失败", reason);
  } finally {
    browsersLoaded.value = true;
  }
}

function selectEngine(id: SearchEngineId) {
  settings.searchEngine = id;
}

function selectSystemDefault() {
  settings.browserPath = "";
  settings.browserName = "";
}

function selectBrowser(browser: BrowserInfo) {
  settings.browserPath = browser.path;
  settings.browserName = browser.name;
}

function isSelected(path: string): boolean {
  return settings.browserPath !== "" && settings.browserPath === path;
}

function fileName(path: string): string {
  return path.split(/[\\/]/).pop() || path;
}

async function pickCustomBrowser() {
  try {
    const picked = await open({
      multiple: false,
      directory: false,
      title: "选择浏览器可执行文件",
      filters: [{ name: "可执行文件", extensions: ["exe"] }],
    });
    if (typeof picked === "string") {
      settings.browserPath = picked;
      settings.browserName = fileName(picked);
    }
  } catch (reason) {
    console.warn("选择浏览器失败", reason);
  }
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") emit("close");
}

onMounted(() => {
  window.addEventListener("keydown", onKeydown);
  void loadBrowsers();
});
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/70 p-4"
      @click.self="emit('close')"
    >
      <section
        class="flex max-h-[85vh] w-[36rem] max-w-full flex-col rounded-lg border border-slate-700 bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-800 px-4 py-3">
          <div class="flex items-start gap-2">
            <Settings class="mt-0.5 h-4 w-4 shrink-0 text-emerald-400" />
            <div>
              <h3 class="text-sm font-semibold text-slate-100">设置</h3>
              <p class="mt-0.5 text-xs text-slate-500">仅保存在本机，不会写入存档</p>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            关闭
          </button>
        </header>

        <div class="space-y-5 overflow-y-auto px-4 py-3">
          <section>
            <h4 class="text-xs font-semibold text-slate-400">默认搜索引擎</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              「搜索图文攻略」使用所选引擎检索收集物；视频搜索始终使用 B 站。
            </p>
            <div class="mt-2 grid grid-cols-3 gap-2">
              <button
                v-for="engine in SEARCH_ENGINES"
                :key="engine.id"
                type="button"
                class="inline-flex items-center justify-center gap-1.5 rounded border px-2 py-1.5 text-xs"
                :class="
                  settings.searchEngine === engine.id
                    ? 'border-emerald-600 bg-emerald-900/30 text-emerald-200'
                    : 'border-slate-700 text-slate-300 hover:border-slate-600 hover:bg-slate-800'
                "
                @click="selectEngine(engine.id)"
              >
                <Search class="h-3.5 w-3.5" />
                {{ engine.name }}
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-400">打开链接的浏览器</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              图文攻略、视频攻略与搜索链接都在所选浏览器中打开；启动失败时自动回退到系统默认浏览器。
            </p>
            <div class="mt-2 space-y-1.5">
              <button
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border px-3 py-2 text-left text-xs"
                :class="
                  settings.browserPath === ''
                    ? 'border-emerald-600 bg-emerald-900/30 text-emerald-200'
                    : 'border-slate-700 text-slate-300 hover:border-slate-600 hover:bg-slate-800'
                "
                @click="selectSystemDefault"
              >
                <span>系统默认浏览器</span>
                <Check v-if="settings.browserPath === ''" class="h-3.5 w-3.5 shrink-0" />
              </button>

              <button
                v-for="browser in browsers"
                :key="browser.id"
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border px-3 py-2 text-left text-xs"
                :class="
                  isSelected(browser.path)
                    ? 'border-emerald-600 bg-emerald-900/30 text-emerald-200'
                    : 'border-slate-700 text-slate-300 hover:border-slate-600 hover:bg-slate-800'
                "
                @click="selectBrowser(browser)"
              >
                <span class="min-w-0">
                  <span class="block truncate font-medium">{{ browser.name }}</span>
                  <span class="mt-0.5 block truncate font-mono text-[10px] text-slate-500">
                    {{ browser.path }}
                  </span>
                </span>
                <Check v-if="isSelected(browser.path)" class="h-3.5 w-3.5 shrink-0" />
              </button>

              <div
                v-if="settings.browserPath !== '' && !browsers.some((b) => b.path === settings.browserPath)"
                class="flex w-full items-center justify-between gap-2 rounded border border-emerald-600 bg-emerald-900/30 px-3 py-2 text-left text-xs text-emerald-200"
              >
                <span class="min-w-0">
                  <span class="block truncate font-medium">
                    {{ settings.browserName || "自定义浏览器" }}
                  </span>
                  <span class="mt-0.5 block truncate font-mono text-[10px] text-emerald-300/70">
                    {{ settings.browserPath }}
                  </span>
                </span>
                <Check class="h-3.5 w-3.5 shrink-0" />
              </div>

              <button
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border border-dashed border-slate-700 px-3 py-2 text-left text-xs text-slate-400 hover:border-slate-600 hover:bg-slate-800"
                @click="pickCustomBrowser"
              >
                <span>
                  {{ settings.browserPath !== '' ? "重新选择浏览器可执行文件…" : "选择自定义浏览器…" }}
                </span>
                <span class="shrink-0 text-[10px] text-slate-500">.exe</span>
              </button>
            </div>
            <p v-if="browsersLoaded && browsers.length === 0" class="mt-1.5 text-[11px] leading-4 text-slate-500">
              未检测到已注册的浏览器，可选择系统默认或手动指定可执行文件。
            </p>
          </section>

          <div class="flex items-center justify-between border-t border-slate-800 pt-3">
            <button
              type="button"
              class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-400 hover:bg-slate-800"
              @click="resetSettings"
            >
              <RotateCcw class="h-3.5 w-3.5" />
              恢复默认
            </button>
            <button
              type="button"
              class="rounded border border-emerald-700 px-3 py-1 text-xs text-emerald-200 hover:bg-emerald-900/30"
              @click="emit('close')"
            >
              完成
            </button>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
