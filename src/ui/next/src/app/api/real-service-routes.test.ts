import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const apiRoot = path.dirname(fileURLToPath(import.meta.url));

function routeFiles(directory: string): string[] {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) return routeFiles(fullPath);
    return /route\.tsx?$/.test(entry.name) ? [fullPath] : [];
  });
}

const mutationExport = /export\s+(?:async\s+function|function|const)\s+(POST|PUT|PATCH|DELETE)\b|proxyCurrentBackendPath\s+as\s+(POST|PUT|PATCH|DELETE)/;
const realAuthority = [
  /\bproxyBackend(?:Get|Post|Put|Patch|Delete|Request)\s*\(/,
  /proxyCurrentBackendPath\s+as\s+(?:POST|PUT|PATCH|DELETE)/,
  /\b(?:proxyPublicAuthentication|registerAndSealSession)\s*\(/,
  /\bfetch\s*\(/,
  /\bPool\b|\bpg\b|\bsqlx\b/i,
  /process\.env\.[A-Z0-9_]*(?:URL|DSN|ENDPOINT|HOST)/,
  /status:\s*(?:501|503)/,
];

describe("production API route authority", () => {
  it("does not keep business state or execute local subprocesses", () => {
    const violations = routeFiles(apiRoot).flatMap((file) => {
      const source = fs.readFileSync(file, "utf8");
      const mutates = mutationExport.test(source);
      const reasons = [
        /from\s+["'][^"']*\/store["']/.test(source) ? "imports an in-memory store" : "",
        /node:child_process/.test(source) ? "executes a local subprocess" : "",
        mutates && /\bMath\.random\s*\(/.test(source) ? "fabricates random output" : "",
        mutates && /\bsuccess\s*:\s*true\b/.test(source) ? "returns hardcoded success" : "",
        mutates && /\bstatus\s*:\s*["']success["']/.test(source) ? "returns hardcoded success status" : "",
      ].filter(Boolean);
      return reasons.map((reason) => `${path.relative(apiRoot, file)}: ${reason}`);
    });

    expect(violations).toEqual([]);
  });

  it("delegates every mutating route to a real authority or fails closed", () => {
    const violations = routeFiles(apiRoot).flatMap((file) => {
      const source = fs.readFileSync(file, "utf8");
      if (!mutationExport.test(source)) return [];
      if (realAuthority.some((pattern) => pattern.test(source))) return [];
      return [`${path.relative(apiRoot, file)}: no real authority or fail-closed response`];
    });

    expect(violations).toEqual([]);
  });

  it("keeps literal backend paths under the versioned API prefix", () => {
    const violations = routeFiles(apiRoot).flatMap((file) => {
      const source = fs.readFileSync(file, "utf8");
      const paths = Array.from(
        source.matchAll(
          /proxyBackend(?:Get|Post|Put|Patch|Delete|Request)\s*\(\s*[^,]+,\s*["'`]([^"'`]*)["'`]/g,
        ),
        (match) => match[1],
      );
      return paths
        .filter((backendPath) => !backendPath.startsWith("/api/v1/"))
        .map((backendPath) => `${path.relative(apiRoot, file)}: ${JSON.stringify(backendPath)}`);
    });

    expect(violations).toEqual([]);
  });
});
