import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

describe("Next visual-test configuration", () => {
  it("disables the development indicator so it cannot cover application content", () => {
    const config = readFileSync(join(process.cwd(), "next.config.mjs"), "utf8");

    expect(config).toMatch(/devIndicators:\s*false/);
  });
});
