import { describe, expect, test } from "vitest";
import { quoteBackendPath, validateLegacyQuoteBody } from "./quoteBackend";

describe("legacy quote request bodies", () => {
  test("preserves valid JSON bytes without losing number precision", () => {
    const source = ' { "amount": 9007199254740993 } ';
    expect(
      new TextDecoder().decode(
        validateLegacyQuoteBody(new TextEncoder().encode(source)),
      ),
    ).toBe(source);
  });

  test("falls back to an empty object for empty, malformed, and invalid UTF-8 bodies", () => {
    for (const body of [
      new Uint8Array(new ArrayBuffer(0)),
      new TextEncoder().encode("{"),
      Uint8Array.from([0xc3, 0x28]),
    ]) {
      expect(new TextDecoder().decode(validateLegacyQuoteBody(body))).toBe("{}");
    }
  });
});

describe("quote backend paths", () => {
  test.each([".", ".."])('rejects exact dot-segment quote ID "%s"', (id) => {
    expect(() => quoteBackendPath(id)).toThrow("invalid quote ID");
  });
});
