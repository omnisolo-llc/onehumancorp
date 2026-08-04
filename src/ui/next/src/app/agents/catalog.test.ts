import { describe, expect, it } from "vitest";
import { experts } from "./catalog";

describe("Expert catalog connectors", () => {
  it("Customer Ambassador advertises only native/direct omnichannel connectors", () => {
    const ambassador = experts.find((agent) => agent.name === "Customer Ambassador");
    expect(ambassador?.connectors).toEqual([
      "Native Omnichannel Inbox", "Instagram DMs", "WhatsApp", "SMS", "Email",
    ]);
  });

  it("Growth Strategist advertises proper connectors", () => {
    const growth = experts.find((agent) => agent.name === "Growth Strategist");
    expect(growth?.connectors).toEqual([
      "Google Analytics", "Stripe", "Mailchimp",
    ]);
  });

  it("Finance Controller advertises proper connectors", () => {
    const finance = experts.find((agent) => agent.name === "Finance Controller");
    expect(finance?.connectors).toEqual([
      "Stripe", "Square", "QuickBooks",
    ]);
  });

  it("Operations Manager advertises proper connectors", () => {
    const operations = experts.find((agent) => agent.name === "Operations Manager");
    expect(operations?.connectors).toEqual([
      "Google Calendar", "Shippo", "Tencent Docs",
    ]);
  });
});
