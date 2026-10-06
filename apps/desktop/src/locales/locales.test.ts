import { describe, expect, it } from "vitest";
import { messages } from "./index";

type Tree = Record<string, unknown>;

function flatten(value: Tree, prefix = ""): Map<string, string> {
  const entries = new Map<string, string>();
  for (const [key, child] of Object.entries(value)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof child === "string") {
      entries.set(path, child);
    } else if (child && typeof child === "object") {
      for (const [nestedKey, nestedValue] of flatten(child as Tree, path)) {
        entries.set(nestedKey, nestedValue);
      }
    }
  }
  return entries;
}

function placeholders(value: string): string[] {
  return [...value.matchAll(/\{(\w+)\}/g)].map((match) => match[1]).sort();
}

describe("locale catalogs", () => {
  const zh = flatten(messages.zh as Tree);
  const en = flatten(messages.en as Tree);

  it("expose the same keys in both locales", () => {
    expect([...en.keys()].sort()).toEqual([...zh.keys()].sort());
  });

  it("do not contain empty messages", () => {
    for (const [key, value] of zh) {
      expect(value.trim(), `zh:${key}`).not.toBe("");
    }
    for (const [key, value] of en) {
      expect(value.trim(), `en:${key}`).not.toBe("");
    }
  });

  it("use the same interpolation parameters", () => {
    for (const [key, value] of zh) {
      expect(placeholders(en.get(key) ?? ""), `en:${key}`).toEqual(placeholders(value));
    }
  });
});
