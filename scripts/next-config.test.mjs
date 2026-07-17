import assert from "node:assert/strict";
import test from "node:test";

import { allowedDevOrigins } from "../src/ui/next/next.config.mjs";

test("allows the exact private LAN hostname in explicit local development", () => {
  assert.deepEqual(
    allowedDevOrigins({
      OHC_WEB_LOCAL_DEV: "true",
      OHC_WEB_CANONICAL_ORIGIN: "http://192.168.8.35:3000",
    }),
    ["127.0.0.1", "localhost", "192.168.8.35"],
  );
});

test("does not allow public or malformed canonical hosts", () => {
  for (const origin of [
    "https://example.com",
    "http://8.8.8.8:3000",
    "not a URL",
  ]) {
    assert.deepEqual(
      allowedDevOrigins({
        OHC_WEB_LOCAL_DEV: "true",
        OHC_WEB_CANONICAL_ORIGIN: origin,
      }),
      ["127.0.0.1", "localhost"],
    );
  }
});

test("ignores the canonical origin outside explicit local development", () => {
  assert.deepEqual(
    allowedDevOrigins({
      OHC_WEB_LOCAL_DEV: "false",
      OHC_WEB_CANONICAL_ORIGIN: "http://192.168.8.35:3000",
    }),
    ["127.0.0.1", "localhost"],
  );
});
