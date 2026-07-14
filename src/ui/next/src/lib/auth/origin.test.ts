import { describe, expect, it } from "vitest";
import { isTrustedMutationOrigin } from "./origin";

const canonical = "https://app.example.com";

function headers(values: Record<string, string> = {}): Headers {
  return new Headers(values);
}

describe("unsafe request origin policy", () => {
  it("accepts only a browser-valid exact same-origin request", () => {
    expect(
      isTrustedMutationOrigin(
        headers({ origin: canonical, "sec-fetch-site": "same-origin" }),
        canonical,
      ),
    ).toBe(true);
  });

  it.each([
    [{ "sec-fetch-site": "same-origin" }, "missing origin"],
    [{ origin: canonical }, "missing fetch metadata"],
    [{ origin: "null", "sec-fetch-site": "same-origin" }, "opaque origin"],
    [{ origin: "https://evil.example", "sec-fetch-site": "cross-site" }, "cross site"],
    [{ origin: "https://sibling.example.com", "sec-fetch-site": "same-site" }, "same-site sibling"],
    [{ origin: "https://app.example.com:444", "sec-fetch-site": "same-origin" }, "wrong port"],
    [{ origin: "https://user@app.example.com", "sec-fetch-site": "same-origin" }, "credentials"],
    [{ origin: `${canonical}, https://evil.example`, "sec-fetch-site": "same-origin" }, "combined origin"],
    [{ origin: canonical, "sec-fetch-site": "none" }, "non same-origin metadata"],
  ] as const)("rejects %#", (values, _label) => {
    expect(isTrustedMutationOrigin(headers(values), canonical)).toBe(false);
  });

  it("ignores forged host and forwarding headers", () => {
    expect(
      isTrustedMutationOrigin(
        headers({
          origin: "https://evil.example",
          "sec-fetch-site": "same-origin",
          host: "app.example.com",
          forwarded: "host=app.example.com;proto=https",
          "x-forwarded-host": "app.example.com",
        }),
        canonical,
      ),
    ).toBe(false);
  });
});
