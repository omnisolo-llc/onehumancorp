import { describe, expect, it } from "vitest";
import { experts } from "./catalog";

describe("Customer Ambassador connectors", () => {
  it("advertises only native/direct omnichannel connectors", () => {
    const ambassador = experts.find((agent) => agent.name === "Customer Ambassador");
    expect(ambassador?.connectors).toEqual([
      "Native Omnichannel Inbox", "Instagram DMs", "WhatsApp", "SMS", "Email",
    ]);
  });
});
