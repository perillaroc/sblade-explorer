#!/usr/bin/env node
// Bump the app version in every manifest that must stay in sync:
//   - apps/desktop/src-tauri/tauri.conf.json   (app/bundle version read by `tauri build`)
//   - apps/desktop/package.json                (npm package version + corepack pinning)
//   - Cargo.toml `[workspace.package] version` (all Rust crates, incl. `sbsave --version`)
//
// Cargo.lock is refreshed here so `cargo test` does not leave the tree dirty.
//
// Usage: pnpm bump 0.2.0 | pnpm bump 1.0.0-rc.1
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const desktopDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(desktopDir, "..", "..");

// Both JSON manifests keep their top level "version" field as the first match.
const JSON_VERSION = /("version"\s*:\s*)"([^"]*)"/;
const CARGO_WORKSPACE_VERSION =
  /(\[workspace\.package\][\s\S]*?\nversion\s*=\s*)"([^"]*)"/;

// SemVer 2.0.0 core version plus an optional pre-release suffix. Build metadata
// (`+...`) is rejected: it adds nothing for users and breaks file names.
const VERSION =
  /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*)(?:\.(?:0|[1-9]\d*|\d*[A-Za-z-][0-9A-Za-z-]*))*))?$/;

const version = process.argv[2] ?? "";
if (!VERSION.test(version)) {
  console.error(
    "用法: pnpm bump <x.y.z> 或 <x.y.z-预发布号>，例如 pnpm bump 0.2.0、pnpm bump 1.0.0-rc.1",
  );
  process.exit(1);
}

function bump(path, pattern) {
  const before = readFileSync(path, "utf8");
  const match = before.match(pattern);
  if (!match) {
    throw new Error(`${path} 中未找到版本号字段`);
  }
  const current = match[2];
  if (current === version) {
    console.log(`${path}: 已是 ${current}，跳过`);
    return;
  }
  writeFileSync(path, before.replace(pattern, (_, prefix) => `${prefix}"${version}"`));
  console.log(`${path}: ${current} -> ${version}`);
}

const tauriConf = join(desktopDir, "src-tauri", "tauri.conf.json");
const packageJson = join(desktopDir, "package.json");
const cargoToml = join(repoRoot, "Cargo.toml");

bump(tauriConf, JSON_VERSION);
bump(packageJson, JSON_VERSION);
bump(cargoToml, CARGO_WORKSPACE_VERSION);

// Fail loudly instead of leaving a half bumped tree behind.
for (const [path, pattern] of [
  [tauriConf, JSON_VERSION],
  [packageJson, JSON_VERSION],
  [cargoToml, CARGO_WORKSPACE_VERSION],
]) {
  if (readFileSync(path, "utf8").match(pattern)?.[2] !== version) {
    throw new Error(`${path} 版本号校验失败`);
  }
}

const cargo = process.platform === "win32" ? "cargo.exe" : "cargo";
execFileSync(cargo, ["update", "--workspace"], { cwd: repoRoot, stdio: "inherit" });

console.log(`\n全部版本号已更新为 ${version}。下一步：`);
console.log(`  git commit -am ":bookmark: 发布 v${version}"`);
console.log(`  git tag v${version}`);
console.log(`  git push --follow-tags`);