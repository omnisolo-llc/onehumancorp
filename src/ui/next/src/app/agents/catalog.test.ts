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

describe("Customer Ambassador verification", () => {
  it("includes all the requested capabilities for the expert", () => {
    const ambassador = experts.find((agent) => agent.name === "Customer Ambassador");
    expect(ambassador).toBeDefined();
    expect(ambassador?.capabilities).toContain("Drafts replies for chat, email, Instagram, WhatsApp, and web inquiries");
    expect(ambassador?.capabilities).toContain("Maintains customer context, notes, preferences, tags, follow-ups, and review requests");
    expect(ambassador?.connectors).not.toContain("Chatwoot");
  });
});
