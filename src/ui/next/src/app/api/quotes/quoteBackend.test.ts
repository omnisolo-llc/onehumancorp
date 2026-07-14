import { describe, expect, test } from "vitest";
import { normalizeLegacyQuoteBody, quoteBackendPath } from "./quoteBackend";

describe("legacy quote request bodies", () => {
  test("canonically reserializes valid JSON", () => {
    expect(
      new TextDecoder().decode(
        normalizeLegacyQuoteBody(
          new TextEncoder().encode(' { "status": "SENT" } '),
        ),
      ),
    ).toBe('{"status":"SENT"}');
  });

  test("falls back to an empty object for empty, malformed, and invalid UTF-8 bodies", () => {
    for (const body of [
      new Uint8Array(new ArrayBuffer(0)),
      new TextEncoder().encode("{"),
      Uint8Array.from([0xc3, 0x28]),
    ]) {
      expect(new TextDecoder().decode(normalizeLegacyQuoteBody(body))).toBe("{}");
    }
  });
});

describe("quote backend paths", () => {
  test.each([".", ".."])('rejects exact dot-segment quote ID "%s"', (id) => {
    expect(() => quoteBackendPath(id)).toThrow("invalid quote ID");
  });
});
