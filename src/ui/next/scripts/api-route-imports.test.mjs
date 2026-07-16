import assert from "node:assert/strict";
import { access, readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { test } from "vitest";

const appRoot = path.resolve(import.meta.dirname, "../src/app");
const apiV1Root = path.join(appRoot, "api/v1");
const importPattern = /\bfrom\s+["']([^"']+)["']/g;
const extensions = [".ts", ".tsx", ".js", ".jsx"];

function withoutComments(source) {
  return source.replace(/\/\*[\s\S]*?\*\/|\/\/[^\n]*/g, "");
}

async function routeFiles(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(entries.map(async (entry) => {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) return routeFiles(file);
    return entry.name === "route.ts" ? [file] : [];
  }));
  return nested.flat();
}

async function exists(file) {
  try {
    await access(file);
    return true;
  } catch {
    return false;
  }
}

async function resolvesImport(routeFile, specifier) {
  const base = path.resolve(path.dirname(routeFile), specifier);
  const candidates = [
    base,
    ...extensions.map((extension) => `${base}${extension}`),
    ...extensions.map((extension) => path.join(base, `index${extension}`)),
  ];
  return (await Promise.all(candidates.map(exists))).some(Boolean);
}

test("every relative import in a v1 route resolves to an application module", async () => {
  const failures = [];
  for (const routeFile of await routeFiles(apiV1Root)) {
    const source = withoutComments(await readFile(routeFile, "utf8"));
    for (const match of source.matchAll(importPattern)) {
      const specifier = match[1];
      if (!specifier.startsWith(".")) continue;
      if (await resolvesImport(routeFile, specifier)) continue;
      failures.push(`${path.relative(appRoot, routeFile)} -> ${specifier}`);
    }
  }
  assert.deepEqual(failures, []);
});
