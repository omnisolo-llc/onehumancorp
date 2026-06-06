import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const repoRoot = resolve(__dirname, "../../../../../..");
const canonical = "src/server/monitoring/dashboards/hybrid-telemetry.json";
const mirroredDashboards = [
  "deploy/helm/ohc/dashboards/hybrid-telemetry.json",
  "deploy/grafana/dashboards/hybrid-telemetry.json",
  "deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json",
];

describe("hybrid telemetry dashboard mirrors", () => {
  it("keeps deploy copies byte-for-byte aligned with the canonical dashboard", () => {
    const canonicalJson = readFileSync(resolve(repoRoot, canonical), "utf8");

    for (const mirror of mirroredDashboards) {
      expect(readFileSync(resolve(repoRoot, mirror), "utf8"), mirror).toBe(canonicalJson);
    }
  });
});
