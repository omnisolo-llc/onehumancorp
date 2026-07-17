import { describe, expect, test } from "vitest";
import { proposalBackendPath, validateLegacyProposalBody } from "./proposalBackend";

describe("legacy proposal request bodies", () => {
  test("preserves valid JSON bytes without losing number precision", () => {
    const source = ' { "amount": 9007199254740993 } ';
    expect(
      new TextDecoder().decode(
        validateLegacyProposalBody(new TextEncoder().encode(source)),
      ),
    ).toBe(source);
  });

  test("falls back to an empty object for empty, malformed, and invalid UTF-8 bodies", () => {
    for (const body of [
      new Uint8Array(new ArrayBuffer(0)),
      new TextEncoder().encode("{"),
      Uint8Array.from([0xc3, 0x28]),
    ]) {
      expect(new TextDecoder().decode(validateLegacyProposalBody(body))).toBe("{}");
    }
  });
});

describe("proposal backend paths", () => {
  test.each([".", ".."])('rejects exact dot-segment proposal ID "%s"', (id) => {
    expect(() => proposalBackendPath(id)).toThrow("invalid proposal ID");
  });
});
