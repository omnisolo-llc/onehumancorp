import { describe, expect, it } from "vitest";
import {
  OMNISOLO_BRAND,
  OMNISOLO_CLOUD_ORIGIN,
  cloudUrl,
} from "./branding";

describe("OmniSolo branding contract", () => {
  it("exposes the canonical brand and cloud origin", () => {
    expect(OMNISOLO_BRAND).toBe("OmniSolo");
    expect(OMNISOLO_CLOUD_ORIGIN).toBe("https://cloud.omnisolo.co");
  });

  it("builds absolute cloud URLs for generated customer embeds", () => {
    expect(cloudUrl("/api/v1/growth/customer-referral/embed?tenant=demo")).toBe(
      "https://cloud.omnisolo.co/api/v1/growth/customer-referral/embed?tenant=demo",
    );
  });

  it("rejects external absolute URLs so generated links cannot drift to a legacy origin", () => {
    expect(() => cloudUrl("https://cloud.omnisolo.co/api/v1/growth/customer-referral/embed")).toThrow(
      /cloud paths must be relative/,
    );
  });
});
